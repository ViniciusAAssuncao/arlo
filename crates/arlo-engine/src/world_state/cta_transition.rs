use crate::match_decision::event_translation::{
    create_envelope, translate_countdown_started, translate_down_advanced,
    translate_out_of_bounds, translate_turnover,
};
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::transition;
use crate::world_state::cta_finishing::FinishingPhaseResult;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::cta_progression::ProgressionPhaseResult;
use crate::world_state::match_state::MatchState;
use arlo_events::{CountdownReason, EventSink};
use arlo_math::units::Position as VectorPosition;
use uuid::Uuid;

pub fn apply_play_transition(
    state: &mut MatchState,
    pass_phase: PassPhaseResult<'_>,
    prog_phase: ProgressionPhaseResult,
    finishing_phase: FinishingPhaseResult,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> DetailedPlayOutcome {
    let is_scored = finishing_phase.scoring_decision.is_scored();
    let is_missed = matches!(
        finishing_phase.scoring_decision,
        ScoringDecision::Missed { .. }
    );
    let pass_failed = !pass_phase.pass_completed;

    let out_of_bounds = pass_failed || is_scored || is_missed;
    let arbitral_stoppage = is_scored;

    let turnover = if is_missed {
        Some(defense_team_id)
    } else {
        None
    };

    let mut resolved_duels = vec![pass_phase.pass_duel_outcome, prog_phase.artro_duel_outcome];
    if let Some(finish_duel) = finishing_phase.finish_duel_outcome {
        resolved_duels.push(finish_duel);
    }

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
        drives_recorded: prog_phase.drives_recorded_count,
        mirins_advanced: prog_phase.mirins_advanced,
        duels: resolved_duels,
        turnover,
        recovering_player_id: if turnover.is_some() {
            Some(pass_phase.goalguard.id())
        } else {
            None
        },
        out_of_bounds,
        arbitral_stoppage,
        last_valid_possession_point: prog_phase.end_position,
        possession_control_seconds: if pass_phase.pass_completed {
            Some(2.0)
        } else {
            Some(0.8)
        },
        scoring_decision: finishing_phase.scoring_decision,
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
            prog_phase.end_position,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, turnover_event));
    }

    if detailed_outcome.out_of_bounds {
        let oob_event = translate_out_of_bounds(
            offense_team_id,
            Some(pass_phase.artrine.id()),
            prog_phase.end_position,
            detailed_outcome
                .possession_control_seconds
                .map(|s| s < 0.7)
                .unwrap_or(false),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, oob_event));
    }

    let new_down = transition_result.snapshot.down() as u32;
    let down_advanced_event = translate_down_advanced(
        previous_down,
        new_down,
        prog_phase.mirins_advanced,
        transition_result.snapshot.advanced_mirins(),
        transition_result.snapshot.down() == 1 && previous_down > 1,
        prog_phase.end_x_mirim,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, down_advanced_event));

    if transition_result.countdown_to_size_triggered {
        let reason = if detailed_outcome.scoring_decision.is_scored() {
            CountdownReason::AfterScore
        } else if detailed_outcome.turnover.is_some() {
            CountdownReason::OutOfBoundsAfterTurnover
        } else if transition_result.snapshot.role().offense() != offense_team_id {
            CountdownReason::TurnoverOnDowns
        } else {
            CountdownReason::OutOfBoundsPlayEnd
        };

        let countdown_event = translate_countdown_started(
            transition_result.snapshot.offense(),
            prog_phase.end_x_mirim,
            reason,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, countdown_event));
    }

    if detailed_outcome.scoring_decision.is_scored()
        || transition_result.snapshot.role().offense() != offense_team_id
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

    let period_ended = state.clock_mut().advance_seconds(25.0);
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