use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::time::DurationComponentKind;
use crate::world_state::constants::IMMEDIATE_CONTROL_THRESHOLD_SECONDS;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::period_resolution::resolve_period_end;
use crate::world_state::play_transition::fatigue_applier::{
    apply_dead_ball_recovery, apply_duel_strain, apply_kinematic_movement_strain,
};
use crate::world_state::play_transition::possession_resolver::{
    build_detailed_play_outcome, classify_play_outcome, determine_countdown_reason,
    resolve_possession_transition,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::{
    apply_match_score, post_transition_score_reset,
};
use crate::world_state::reorganization::derive_and_apply_reorganization;
use arlo_domain::{ArtrineDecisionKind, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
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
    let mut publisher = EventPublisher::new(state, sink);

    publisher.emit_drives(pass_phase.artrine.id(), &execution_outcome.drive_row_indices);

    if let Some(flight_info) = &execution_outcome.distribution_flight {
        publisher.emit_distribution_flight(flight_info);
    }

    let mut play_duels = Vec::with_capacity(1 + execution_outcome.duels.len());
    play_duels.push(pass_phase.pass_duel_outcome.clone());
    play_duels.extend(execution_outcome.duels.iter().cloned());

    apply_duel_strain(&mut publisher, &play_duels);
    publisher.emit_duel_events(&execution_outcome.duels, pass_phase.artrine.id());

    apply_match_score(publisher.state_mut(), offense_team_id, &execution_outcome.scoring_decision);
    publisher.emit_scoring_event(&execution_outcome.scoring_decision);

    let classification = classify_play_outcome(&pass_phase, &execution_outcome, defense_team_id);

    let mut resolved_duels = vec![pass_phase.pass_duel_outcome.clone()];
    resolved_duels.extend(execution_outcome.duels.clone());

    let mut play_ledger = pass_phase.duration_ledger.clone();
    play_ledger.merge(execution_outcome.duration_ledger.clone());

    let possession_control_seconds = if pass_phase.pass_completed {
        Some(play_ledger.total_live().value())
    } else {
        None
    };

    let detailed_outcome = build_detailed_play_outcome(
        &pass_phase,
        &execution_outcome,
        &classification,
        resolved_duels,
        possession_control_seconds,
        offense_team_id,
        defense_team_id,
    );

    apply_kinematic_movement_strain(&mut publisher, &execution_outcome.kinematic_trajectories);

    let offense_lineup = if offense_team_id == publisher.state().home_team_id() {
        publisher.state().home_lineup().clone()
    } else {
        publisher.state().away_lineup().clone()
    };
    let defense_lineup = if defense_team_id == publisher.state().home_team_id() {
        publisher.state().home_lineup().clone()
    } else {
        publisher.state().away_lineup().clone()
    };

    let offense_players = offense_lineup.players();
    let defense_players = defense_lineup.players();

    for duel in &play_duels {
        publisher.state_mut().impulse_bus_mut().publish_attributed_duel(
            duel,
            &offense_players,
            &defense_players,
        );
    }

    if execution_outcome.scoring_decision.is_scored()
        || matches!(execution_outcome.scoring_decision, ScoringDecision::Missed { .. })
    {
        let goalguard = defense_players
            .iter()
            .copied()
            .find(|p| {
                p.positions()
                    .iter()
                    .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
            })
            .unwrap_or(defense_players[0]);

        let finisher_id = execution_outcome
            .receiver_id
            .unwrap_or(pass_phase.artrine.id());

        publisher.state_mut().impulse_bus_mut().publish_scoring_decision(
            &execution_outcome.scoring_decision,
            finisher_id,
            goalguard.id(),
            0.5,
            &offense_players,
            &defense_players,
        );
    }

    let previous_down = publisher.state().possession().down() as u32;
    let transition_result = resolve_possession_transition(publisher.state(), &detailed_outcome);

    publisher
        .state_mut()
        .impulse_bus_mut()
        .publish_events(transition_result.impulse_events.clone());

    let current_period_seconds = publisher.state().clock().seconds_in_period();
    let shifts = publisher.state_mut().process_impulse_bus(current_period_seconds);
    for (pid, shift, ev) in shifts {
        publisher.emit_impulse_shift(pid, &shift, &ev);
    }

    if let Some(new_offense) = detailed_outcome.turnover {
        publisher.emit_turnover_event(
            offense_team_id,
            new_offense,
            detailed_outcome.recovering_player_id,
            detailed_outcome.lost_by_player_id,
            !detailed_outcome.out_of_bounds,
            execution_outcome.end_position,
        );
    }

    if detailed_outcome.out_of_bounds {
        let was_immediate = detailed_outcome
            .possession_control_seconds
            .map(|s| s < IMMEDIATE_CONTROL_THRESHOLD_SECONDS)
            .unwrap_or(false);
        publisher.emit_out_of_bounds_event(
            offense_team_id,
            Some(pass_phase.artrine.id()),
            execution_outcome.end_position,
            was_immediate,
        );
    }

    let end_x_mirim = execution_outcome.end_position.raw().0 / MIRIM_TO_METERS;
    let new_down = transition_result.snapshot.down() as u32;
    let is_possession_change = transition_result.snapshot.role().offense() != offense_team_id;
    let is_first_down = (transition_result.snapshot.down() == 1 && previous_down > 1)
        || is_possession_change
        || transition_result.snapshot.down() == 1;

    publisher.emit_down_advanced_event(
        previous_down,
        new_down,
        execution_outcome.mirins_advanced,
        transition_result.snapshot.advanced_mirins(),
        is_first_down,
        end_x_mirim,
    );

    if transition_result.countdown_to_size_triggered {
        let reason = determine_countdown_reason(&detailed_outcome, is_possession_change);
        publisher.emit_countdown_event(transition_result.snapshot.offense(), end_x_mirim, reason);
    }

    let next_snapshot = post_transition_score_reset(
        publisher.state_mut(),
        &detailed_outcome.scoring_decision,
        is_possession_change,
        transition_result.snapshot,
    );

    let is_post_turnover = detailed_outcome.turnover.is_some();

    if transition_result.countdown_to_size_triggered {
        let next_scrimmage_x_mirim =
            next_snapshot.series_state().scrimmage_point().raw().0 / MIRIM_TO_METERS;
        let (reorg_duration, huddle_duration) = derive_and_apply_reorganization(
            &mut publisher,
            next_scrimmage_x_mirim,
            is_post_turnover,
            detailed_outcome.recovering_player_id,
        );
        play_ledger.record_dead_ball(DurationComponentKind::Reorganization, reorg_duration);
        play_ledger.record_dead_ball(DurationComponentKind::Huddle, huddle_duration);
    }

    let dead_ball_seconds = play_ledger.total_dead_ball().value();
    apply_dead_ball_recovery(&mut publisher, dead_ball_seconds);

    let live_seconds = play_ledger.total_live().value();
    if live_seconds > 0.0 {
        publisher.state_mut().advance_impulse_dynamics(live_seconds);
    }
    if dead_ball_seconds > 0.0 {
        publisher.state_mut().advance_impulse_dynamics(dead_ball_seconds);
    }

    let period_ended = publisher
        .state_mut()
        .clock_mut()
        .advance_seconds(live_seconds);
    publisher.state_mut().real_time_mut().add(play_ledger.total());
    *publisher.state_mut().possession_mut() = next_snapshot;

    if period_ended {
        resolve_period_end(publisher.state_mut());
    }

    detailed_outcome
}