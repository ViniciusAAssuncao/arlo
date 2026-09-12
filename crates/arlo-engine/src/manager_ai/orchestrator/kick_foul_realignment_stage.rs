use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::kick_foul::evaluate_kick_foul_realignment;
use crate::manager_ai::time_calls::execute_time_call;
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, TimeCallReason};
use arlo_math::units::Duration;
use rand::Rng;
use uuid::Uuid;

pub fn evaluate_kick_foul_realignment_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    rng: &mut R,
) -> Duration {
    let mut extra_dead_ball = Duration::new(0.0);
    if let Some(pending) = publisher.state().kick_foul_pending().copied() {
        if pending.awarded_team_id() == team_id {
            let context = ManagerDecisionContext::build(publisher.state(), team_id);
            let pitch_length_m = publisher.state().pitch().length().value();
            let spot_x_m = pending.spot().raw().0;
            let normalized_x = if is_home {
                (spot_x_m / pitch_length_m).clamp(0.0, 1.0)
            } else {
                ((pitch_length_m - spot_x_m) / pitch_length_m).clamp(0.0, 1.0)
            };
            if evaluate_kick_foul_realignment(&context, normalized_x, pending.scoring_tier(), rng) {
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
        }
    }
    extra_dead_ball
}