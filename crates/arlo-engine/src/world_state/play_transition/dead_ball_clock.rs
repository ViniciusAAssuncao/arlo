use crate::manager_ai::orchestrator::ManagerAiEngine;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::possession::TransitionResult;
use crate::rng::RngStream;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::period_resolution::resolve_period_end;
use crate::world_state::play_transition::added_time_handler::evaluate_and_apply_added_time;
use crate::world_state::play_transition::fatigue_applier::apply_dead_ball_recovery;
use crate::world_state::play_transition::kick_foul_handler::resolve_and_apply_kick_foul;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::post_transition_score_reset;
use crate::world_state::reorganization::derive_and_apply_reorganization;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;

pub fn handle_dead_ball_and_clock(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    detailed_outcome: &DetailedPlayOutcome,
    transition_result: TransitionResult,
    play_ledger: &mut DurationLedger,
) {
    let is_possession_change =
        transition_result.snapshot.role().offense() != detailed_outcome.offense_team_id;

    let next_snapshot = post_transition_score_reset(
        publisher.state_mut(),
        &detailed_outcome.scoring_decision,
        is_possession_change,
        transition_result.snapshot,
    );

    *publisher.state_mut().possession_mut() = next_snapshot;

    let is_post_turnover = detailed_outcome.turnover.is_some();

    if transition_result.countdown_to_size_triggered {
        let seq = publisher.state_mut().next_sequence();
        let mut ai_rng = publisher
            .state()
            .rng_provider()
            .indexed_rng_for(RngStream::PlayCallSelection, seq);

        let extra_offense =
            ManagerAiEngine::on_stoppage(publisher, detailed_outcome.offense_team_id, &mut ai_rng);
        let extra_defense =
            ManagerAiEngine::on_stoppage(publisher, detailed_outcome.defense_team_id, &mut ai_rng);
        let extra_total = extra_offense + extra_defense;
        if extra_total.value() > 0.0 {
            play_ledger.record_dead_ball(DurationComponentKind::Huddle, extra_total);
        }

        if let Some(pending) = publisher.state().kick_foul_pending().copied() {
            let mut kick_foul_rng = publisher
                .state()
                .rng_provider()
                .indexed_rng_for(RngStream::KickFoulResolution, seq);
            resolve_and_apply_kick_foul(publisher, &pending, &mut kick_foul_rng);
        }

        let next_scrimmage_x_mirim =
            publisher.state().possession().scrimmage_point().raw().0 / MIRIM_TO_METERS;
        let (reorg_duration, huddle_duration) = derive_and_apply_reorganization(
            publisher,
            next_scrimmage_x_mirim,
            is_post_turnover,
            detailed_outcome.recovering_player_id,
        );
        play_ledger.record_dead_ball(DurationComponentKind::Reorganization, reorg_duration);
        play_ledger.record_dead_ball(DurationComponentKind::Huddle, huddle_duration);
    }

    let dead_ball_seconds = play_ledger.total_dead_ball().value();
    publisher
        .state_mut()
        .record_period_dead_ball_seconds(dead_ball_seconds);
    apply_dead_ball_recovery(publisher, dead_ball_seconds);

    let live_seconds = play_ledger.total_live().value();
    if live_seconds > 0.0 {
        publisher.state_mut().advance_impulse_dynamics(live_seconds);
    }
    if dead_ball_seconds > 0.0 {
        publisher
            .state_mut()
            .advance_impulse_dynamics(dead_ball_seconds);
    }

    let mut period_ended = publisher
        .state_mut()
        .clock_mut()
        .advance_seconds(live_seconds);

    if period_ended {
        let seq = publisher.state_mut().next_sequence();
        let mut rng = publisher
            .state()
            .rng_provider()
            .indexed_rng_for(RngStream::AddedTimeDecision, seq);
        period_ended = evaluate_and_apply_added_time(publisher, &mut rng);
    }

    publisher
        .state_mut()
        .real_time_mut()
        .add(play_ledger.total());

    let transitions = publisher
        .state_mut()
        .tick_player_availability(live_seconds);
    for (player_id, is_home, prev, new) in transitions {
        let team_id = if is_home {
            publisher.state().home_team_id()
        } else {
            publisher.state().away_team_id()
        };
        publisher.emit_player_availability_changed(player_id, team_id, prev, new);
    }

    if period_ended {
        resolve_period_end(publisher.state_mut());
    }
}
