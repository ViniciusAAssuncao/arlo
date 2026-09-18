use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::human_control::try_apply_human_tactical_switch;
use crate::manager_ai::tactical_adjustment::{
    execute_tactical_adjustment_by_id, TacticalAdjustmentDecisionEngine,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::situational::build_situational_context;
use arlo_domain::ManagerControlMode;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
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
