use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::event_translation::{
    create_envelope, translate_countdown_started, translate_down_advanced,
    translate_drive_recorded, translate_duel_resolved, translate_out_of_bounds,
    translate_scoring_decision, translate_turnover,
};
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::transition;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::ArtrineDecisionKind;
use arlo_events::{CountdownReason, EventArtroPlacement, EventSink};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use uuid::Uuid;

pub fn apply_play_transition(
    state: &mut MatchState,
    pass_phase: PassPhaseResult<'_>,
    _decision: ArtrineDecisionKind,
    execution_outcome: ArtrineExecutionOutcome,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> DetailedPlayOutcome {
    for &row_index in &execution_outcome.drive_row_indices {
        state.increment_drives();
        let rx = (row_index as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
        let drive_event = translate_drive_recorded(
            pass_phase.artrine.id(),
            row_index,
            EventArtroPlacement::Central,
            state.drives_in_current_series(),
            rx,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, drive_event));
    }

    for duel in &execution_outcome.duels {
        let duel_event = translate_duel_resolved(
            duel,
            vec![pass_phase.artrine.id()],
            vec![pass_phase.goalguard.id()],
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, duel_event));
    }

    match &execution_outcome.scoring_decision {
        ScoringDecision::GoalPoint { .. } => {
            state.record_goal_point(offense_team_id);
        }
        ScoringDecision::FieldPoint { .. } => {
            state.record_field_point(offense_team_id);
        }
        ScoringDecision::FieldGoal { post, .. } => {
            state.record_field_goal(offense_team_id, *post);
        }
        _ => {}
    }

    if let Some(match_event) = translate_scoring_decision(&execution_outcome.scoring_decision) {
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, match_event));
    }

    let is_scored = execution_outcome.scoring_decision.is_scored();
    let is_missed = matches!(
        execution_outcome.scoring_decision,
        ScoringDecision::Missed { .. }
    );
    let pass_failed = !pass_phase.pass_completed;

    let out_of_bounds = pass_failed || is_scored || is_missed;
    let arbitral_stoppage = is_scored;

    let turnover = if execution_outcome.turnover.is_some() {
        execution_outcome.turnover
    } else if is_missed {
        Some(defense_team_id)
    } else {
        None
    };

    let recovering_player_id = if execution_outcome.recovering_player_id.is_some() {
        execution_outcome.recovering_player_id
    } else if turnover.is_some() {
        Some(pass_phase.goalguard.id())
    } else {
        None
    };

    let mut resolved_duels = vec![pass_phase.pass_duel_outcome];
    resolved_duels.extend(execution_outcome.duels);

    let detailed_outcome = DetailedPlayOutcome {
        offense_team_id,
        defense_team_id,
        passer_id: pass_phase.passer.id(),
        artrine_id: pass_phase.artrine.id(),
        down_number: pass_phase.down_number,
        scrimmage_x_mirim: pass_phase.scrimmage_x_mirim,
        pass_completed: pass_phase.pass_completed,
        pass_is_aerial: pass_phase.is_aerial,
        reception_point: pass_phase.reception_point,
        drives_recorded: execution_outcome.drives_recorded,
        mirins_advanced: execution_outcome.mirins_advanced,
        duels: resolved_duels,
        turnover,
        recovering_player_id,
        out_of_bounds,
        arbitral_stoppage,
        last_valid_possession_point: execution_outcome.end_position,
        possession_control_seconds: if pass_phase.pass_completed {
            Some(execution_outcome.duration_ledger.total_live().value())
        } else {
            None
        },
        scoring_decision: execution_outcome.scoring_decision,
    };

    let previous_down = state.possession().down() as u32;
    let possession_outcome = detailed_outcome.to_possession_outcome();
    let transition_result = transition(state.possession(), &possession_outcome);

    if let Some(new_offense) = detailed_outcome.turnover {
        let turnover_event = translate_turnover(
            offense_team_id,
            new_offense,
            detailed_outcome.recovering_player_id,
            !out_of_bounds,
            execution_outcome.end_position,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, turnover_event));
    }

    if detailed_outcome.out_of_bounds {
        let oob_event = translate_out_of_bounds(
            offense_team_id,
            Some(pass_phase.artrine.id()),
            execution_outcome.end_position,
            detailed_outcome
                .possession_control_seconds
                .map(|s| s < 0.7)
                .unwrap_or(false),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, oob_event));
    }

    let end_x_mirim = execution_outcome.end_position.raw().0 / MIRIM_TO_METERS;
    let new_down = transition_result.snapshot.down() as u32;
    let is_possession_change = transition_result.snapshot.role().offense() != offense_team_id;
    let is_first_down = (transition_result.snapshot.down() == 1 && previous_down > 1)
        || is_possession_change
        || transition_result.snapshot.down() == 1;

    let down_advanced_event = translate_down_advanced(
        previous_down,
        new_down,
        execution_outcome.mirins_advanced,
        transition_result.snapshot.advanced_mirins(),
        is_first_down,
        end_x_mirim,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, down_advanced_event));

    if transition_result.countdown_to_size_triggered {
        let reason = if detailed_outcome.scoring_decision.is_scored() {
            CountdownReason::AfterScore
        } else if detailed_outcome.turnover.is_some() {
            CountdownReason::OutOfBoundsAfterTurnover
        } else if is_possession_change {
            CountdownReason::TurnoverOnDowns
        } else {
            CountdownReason::OutOfBoundsPlayEnd
        };

        let countdown_event = translate_countdown_started(
            transition_result.snapshot.offense(),
            end_x_mirim,
            reason,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, countdown_event));
    }

    if detailed_outcome.scoring_decision.is_scored()
        || is_possession_change
        || transition_result.snapshot.down() == 1
    {
        state.reset_drives();
    }

    let mut next_snapshot = transition_result.snapshot;
    if detailed_outcome.scoring_decision.is_scored() {
        let center_scrimmage = VectorPosition::from_components(
            state.pitch().length().value() / 2.0,
            state.pitch().width().value() / 2.0,
            0.0,
        );
        next_snapshot.series_state_mut().reset(center_scrimmage);
    }

    *state.possession_mut() = next_snapshot;

    let reorganization_seconds = if !arbitral_stoppage {
        let mut time_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::SpatialNoise, seq);
        time_rng.gen_range(15.0f64..25.0f64)
    } else {
        0.0
    };

    let total_elapsed = execution_outcome.duration_ledger.total().value() + reorganization_seconds;

    let period_ended = state
        .clock_mut()
        .advance_seconds(total_elapsed);
    if period_ended {
        if state.clock().period() < 4 {
            state.clock_mut().next_period();
        } else if state.clock().period() == 4 {
            let home_pts = state.home_score().total_points;
            let away_pts = state.away_score().total_points;
            if home_pts == away_pts {
                state.clock_mut().next_period();
            } else {
                state.clock_mut().finish_match();
            }
        } else if state.clock().period() < 6 {
            state.clock_mut().next_period();
        } else {
            state.clock_mut().finish_match();
        }
    }

    detailed_outcome
}