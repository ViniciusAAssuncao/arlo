use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::event_translation::{
    create_envelope, translate_countdown_started, translate_distribution_completed,
    translate_down_advanced, translate_drive_recorded, translate_duel_resolved,
    translate_out_of_bounds, translate_physical_strain_recorded,
    translate_reception_resolved, translate_recovery_interval_processed,
    translate_scoring_decision, translate_turnover,
};
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::systems::pacing::{calculate_player_pacing_multiplier, is_player_near_ball};
use crate::physical::systems::positional_strain::calculate_transit_strain_multiplier;
use crate::possession::{transition, TurnoverCategory};
use crate::resolution::DuelKind as EngineDuelKind;
use crate::time::DurationComponentKind;
use crate::world_state::context_analyzer::analyze_match_state;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::period_resolution::resolve_period_end;
use crate::world_state::reorganization::derive_and_apply_reorganization;
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_events::{CountdownReason, EventArtroPlacement, EventSink};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
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

    if let Some(flight_info) = &execution_outcome.distribution_flight {
        let dist_event = translate_distribution_completed(flight_info);
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, dist_event));
    }

    let mut play_duels = Vec::with_capacity(1 + execution_outcome.duels.len());
    play_duels.push(pass_phase.pass_duel_outcome.clone());
    play_duels.extend(execution_outcome.duels.iter().cloned());

    for duel in &play_duels {
        let duel_kind = duel.outcome().kind();
        let mult = crate::physical::models::anaerobic::calculate_duel_intensity_multiplier(duel_kind);
        for attacker_id in duel.attacker_ids() {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*attacker_id, 1.0, mult);
            let strain_ev = translate_physical_strain_recorded(*attacker_id, energy, w_bal, 0.0);
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, strain_ev));
        }
        for defender_id in duel.defender_ids() {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*defender_id, 1.0, mult);
            let strain_ev = translate_physical_strain_recorded(*defender_id, energy, w_bal, 0.0);
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, strain_ev));
        }
    }

    for duel in &execution_outcome.duels {
        let duel_event = translate_duel_resolved(
            duel.outcome(),
            duel.attacker_ids().to_vec(),
            duel.defender_ids().to_vec(),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, duel_event));

        if matches!(
            duel.outcome().kind(),
            EngineDuelKind::RouteContest | EngineDuelKind::AerialDuel
        ) {
            let receiver_id = duel
                .attacker_ids()
                .first()
                .copied()
                .unwrap_or(pass_phase.artrine.id());
            let reception_event = translate_reception_resolved(
                receiver_id,
                pass_phase.artrine.id(),
                duel.outcome().attacker_won(),
                duel.outcome().kind() == EngineDuelKind::AerialDuel,
            );
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, reception_event));
        }
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
    let is_distribution_dropped = execution_outcome
        .distribution_flight
        .as_ref()
        .map_or(false, |f| !f.caught && execution_outcome.turnover.is_none());

    let out_of_bounds = pass_failed || is_scored || is_missed || is_distribution_dropped;
    let arbitral_stoppage = is_scored;

    let (_turnover_category, turnover, recovering_player_id, lost_by_player_id) = if is_missed {
        let category = TurnoverCategory::MissedShot;
        let lost_by = match &execution_outcome.scoring_decision {
            ScoringDecision::Missed { scorer_id, .. } => Some(*scorer_id),
            _ => execution_outcome
                .receiver_id
                .or(Some(pass_phase.artrine.id())),
        };
        (
            Some(category),
            Some(defense_team_id),
            category.sanitize_recovering_player(None),
            lost_by,
        )
    } else if let Some(turnover_team) = execution_outcome.turnover {
        let category = if execution_outcome
            .distribution_flight
            .as_ref()
            .map_or(false, |f| !f.caught)
        {
            TurnoverCategory::Interception
        } else {
            TurnoverCategory::Dispossession
        };
        let lost_by = execution_outcome
            .receiver_id
            .or(Some(pass_phase.artrine.id()));
        (
            Some(category),
            Some(turnover_team),
            category.sanitize_recovering_player(execution_outcome.recovering_player_id),
            lost_by,
        )
    } else {
        (None, None, None, None)
    };

    let mut resolved_duels = vec![pass_phase.pass_duel_outcome];
    resolved_duels.extend(execution_outcome.duels);

    let mut play_ledger = pass_phase.duration_ledger;
    play_ledger.merge(execution_outcome.duration_ledger);

    let possession_control_seconds = if pass_phase.pass_completed {
        Some(play_ledger.total_live().value())
    } else {
        None
    };

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
        lost_by_player_id,
        out_of_bounds,
        arbitral_stoppage,
        last_valid_possession_point: execution_outcome.end_position,
        possession_control_seconds,
        scoring_decision: execution_outcome.scoring_decision,
    };

    let live_seconds = play_ledger.total_live().value().max(1.0);
    let game_state_pressure = analyze_match_state(state);
    let ball_pos = execution_outcome.end_position;

    let runner_id = execution_outcome
        .receiver_id
        .unwrap_or(pass_phase.artrine.id());

    let all_players: Vec<Player> = state
        .home_lineup()
        .players()
        .into_iter()
        .chain(state.away_lineup().players().into_iter())
        .cloned()
        .collect();

    for p in &all_players {
        let pid = p.id();
        let p_fatigue = state.fatigue_for(&pid);
        let is_home = state.home_offensive_position_index().contains_key(&pid);
        let team_id = if is_home {
            state.home_team_id()
        } else {
            state.away_team_id()
        };
        let p_pos = state
            .position_index_for_team(team_id)
            .get(&pid)
            .copied()
            .unwrap_or(DomainPosition::CenterOffense);

        let is_near = state
            .spatial_map()
            .get_position(&pid)
            .map_or(false, |pos| is_player_near_ball(pos, ball_pos, 15.0))
            || pid == runner_id
            || pid == pass_phase.passer.id();

        let pacing_mult = calculate_player_pacing_multiplier(
            p,
            state.attribute_keys(),
            is_near,
            &game_state_pressure,
        );

        let transit_mult = calculate_transit_strain_multiplier(p_pos);
        let eff_speed = calculate_effective_player_speed(
            p,
            state.attribute_keys(),
            &p_fatigue,
        );

        let base_transit_mirim = (eff_speed.value() * 0.10 / MIRIM_TO_METERS) * live_seconds;
        let mut player_dist = base_transit_mirim * transit_mult * pacing_mult;

        if pid == runner_id && execution_outcome.mirins_advanced > 0.0 {
            player_dist += execution_outcome.mirins_advanced;
        }

        if player_dist > 0.0 {
            let (energy, w_bal) = state.record_distance(pid, player_dist);
            let strain_ev = translate_physical_strain_recorded(pid, energy, w_bal, player_dist);
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, strain_ev));
        }
    }

    let previous_down = state.possession().down() as u32;
    let possession_outcome = detailed_outcome.to_possession_outcome();
    let transition_result = transition(state.possession(), &possession_outcome);

    if let Some(new_offense) = detailed_outcome.turnover {
        let turnover_event = translate_turnover(
            offense_team_id,
            new_offense,
            detailed_outcome.recovering_player_id,
            detailed_outcome.lost_by_player_id,
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

    if detailed_outcome.scoring_decision.is_scored() || is_possession_change {
        state.reset_drives();
    }

    let was_goal_point = matches!(
        detailed_outcome.scoring_decision,
        ScoringDecision::GoalPoint { .. }
    );
    let mut next_snapshot = transition_result.snapshot;
    if detailed_outcome.scoring_decision.is_scored() {
        let center_scrimmage = VectorPosition::from_components(
            state.pitch().length().value() / 2.0,
            state.pitch().width().value() / 2.0,
            0.0,
        );
        next_snapshot.series_state_mut().reset(center_scrimmage);
        if was_goal_point {
            next_snapshot.series_state_mut().is_bonus_phase = true;
        }
    }

    if transition_result.countdown_to_size_triggered {
        let next_scrimmage_x_mirim =
            next_snapshot.series_state().scrimmage_point().raw().0 / MIRIM_TO_METERS;
        let (reorg_duration, huddle_duration) =
            derive_and_apply_reorganization(state, next_scrimmage_x_mirim, sink);
        play_ledger.record_dead_ball(DurationComponentKind::Reorganization, reorg_duration);
        play_ledger.record_dead_ball(DurationComponentKind::Huddle, huddle_duration);
    }

    let dead_ball_seconds = play_ledger.total_dead_ball().value();
    if dead_ball_seconds > 0.0 {
        let recoveries = state.apply_dead_ball_recovery(dead_ball_seconds);
        for (pid, recovery_amount, new_w_bal) in recoveries {
            if recovery_amount > 0.0 {
                let rec_ev = translate_recovery_interval_processed(
                    pid,
                    recovery_amount,
                    dead_ball_seconds,
                    new_w_bal,
                );
                let seq = state.next_sequence();
                let clock_inst = state.clock().to_instant();
                sink.record(create_envelope(seq, clock_inst, rec_ev));
            }
        }
    }

    let period_ended = state
        .clock_mut()
        .advance_seconds(play_ledger.total_live().value());
    state.real_time_mut().add(play_ledger.total());
    *state.possession_mut() = next_snapshot;
    if period_ended {
        resolve_period_end(state);
    }

    detailed_outcome
}