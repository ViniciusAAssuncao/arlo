use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::human_control::{
    try_apply_human_kick_foul_realignment, try_apply_human_tactical_switch,
    try_apply_human_time_call,
};
use crate::manager_ai::kick_foul::evaluate_kick_foul_realignment;
use crate::manager_ai::tactical_adjustment::{
    execute_tactical_adjustment_by_id, TacticalAdjustmentDecisionEngine,
};
use crate::manager_ai::time_calls::{execute_time_call, TimeCallDecisionEngine};
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::situational::build_situational_context;
use arlo_domain::ManagerControlMode;
use arlo_events::{EventSink, TimeCallReason};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_math::units::Duration;
use rand::Rng;
use uuid::Uuid;

pub fn evaluate_tactical_adjustment_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    period_duration_seconds: f64,
    manager_decision_inbox: &ManagerDecisionInbox,
    rng: &mut R,
) {
    if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
        let context = ManagerDecisionContext::build(publisher.state(), team_id);
        let tac_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.adaptability,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::TacticalAdjustment,
            tac_cooldown,
        ) {
            let is_home = team_id == publisher.state().home_team_id();
            let available_profiles = publisher
                .state()
                .available_profiles_for_team(team_id)
                .to_vec();
            let active_profile_id = publisher.state().tactical_profile_for_team(team_id).id();
            let pitch_length_mirim = publisher.state().pitch().length_mirim();
            let scrimmage_x_mirim = publisher.state().possession().scrimmage_x_mirim();
            let normalized_x_to_goal = if is_home {
                (scrimmage_x_mirim / pitch_length_mirim).clamp(0.0, 1.0)
            } else {
                ((pitch_length_mirim - scrimmage_x_mirim) / pitch_length_mirim).clamp(0.0, 1.0)
            };
            let situational_ctx =
                build_situational_context(publisher.state(), normalized_x_to_goal);

            if let Some(new_profile_id) = TacticalAdjustmentDecisionEngine::evaluate(
                &context,
                &available_profiles,
                active_profile_id,
                &situational_ctx,
                rng,
            ) {
                if execute_tactical_adjustment_by_id(
                    publisher,
                    team_id,
                    new_profile_id,
                    &available_profiles,
                ) {
                    publisher
                        .state_mut()
                        .mark_decision_triggered(team_id, ManagerDecisionKind::TacticalAdjustment);
                }
            }
        }
    } else {
        try_apply_human_tactical_switch(publisher, team_id, manager_decision_inbox);
    }
}

pub fn evaluate_kick_foul_realignment_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    manager_decision_inbox: &ManagerDecisionInbox,
    rng: &mut R,
) -> Duration {
    let mut extra_dead_ball = Duration::new(0.0);
    if let Some(pending) = publisher.state().kick_foul_pending().copied() {
        if pending.awarded_team_id() == team_id {
            if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
                let context = ManagerDecisionContext::build(publisher.state(), team_id);
                let pitch_length_mirim = publisher.state().pitch().length_mirim();
                let spot_x_mirim = pending.spot_x_mirim();
                let normalized_x = if is_home {
                    (spot_x_mirim / pitch_length_mirim).clamp(0.0, 1.0)
                } else {
                    ((pitch_length_mirim - spot_x_mirim) / pitch_length_mirim).clamp(0.0, 1.0)
                };
                if evaluate_kick_foul_realignment(
                    &context,
                    normalized_x,
                    pending.scoring_tier(),
                    rng,
                ) {
                    let mut ledger = DurationLedger::new();
                    if execute_time_call(
                        publisher,
                        team_id,
                        is_home,
                        &mut ledger,
                        TimeCallReason::KickFoulRealignment,
                    ) {
                        extra_dead_ball = extra_dead_ball + ledger.total_dead_ball();
                        publisher.state_mut().clear_kick_foul_pending();
                        publisher
                            .state_mut()
                            .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
                    }
                }
            } else if try_apply_human_kick_foul_realignment(
                publisher,
                team_id,
                is_home,
                manager_decision_inbox,
            ) {
                let duration_seconds = (publisher
                    .state()
                    .format_rules()
                    .time_call_duration_minutes()
                    * 60) as f64;
                extra_dead_ball = Duration::new(duration_seconds);
            }
        }
    }
    extra_dead_ball
}

pub fn evaluate_time_call_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    period_duration_seconds: f64,
    manager_decision_inbox: &ManagerDecisionInbox,
    rng: &mut R,
) -> Duration {
    let mut extra_dead_ball = Duration::new(0.0);
    if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
        let context = ManagerDecisionContext::build(publisher.state(), team_id);
        let time_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.time_call_management,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::TimeCall,
            time_cooldown,
        ) {
            let just_conceded = publisher.state().last_action_score_occurred()
                && publisher.state().last_scoring_team() != Some(team_id);
            if TimeCallDecisionEngine::evaluate(&context, just_conceded, rng) {
                let mut ledger = DurationLedger::new();
                if execute_time_call(
                    publisher,
                    team_id,
                    is_home,
                    &mut ledger,
                    TimeCallReason::Standard,
                ) {
                    extra_dead_ball = extra_dead_ball + ledger.total_dead_ball();
                    publisher
                        .state_mut()
                        .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
                }
            }
        }
    } else if try_apply_human_time_call(publisher, team_id, is_home, manager_decision_inbox) {
        let duration_seconds = (publisher
            .state()
            .format_rules()
            .time_call_duration_minutes()
            * 60) as f64;
        extra_dead_ball = Duration::new(duration_seconds);
    }
    extra_dead_ball
}