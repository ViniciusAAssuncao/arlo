use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::human_control::try_apply_human_time_call;
use crate::manager_ai::time_calls::{execute_time_call, TimeCallDecisionEngine};
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::ManagerControlMode;
use arlo_events::{EventSink, TimeCallReason};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_math::units::Duration;
use rand::Rng;
use uuid::Uuid;

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
