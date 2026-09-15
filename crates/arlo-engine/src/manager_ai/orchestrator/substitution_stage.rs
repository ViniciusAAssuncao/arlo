use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::human_control::try_apply_human_substitutions;
use crate::manager_ai::substitutions::{execute_substitutions, SubstitutionDecisionEngine};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::ManagerControlMode;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use rand::Rng;
use uuid::Uuid;

pub fn evaluate_substitution_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    period_duration_seconds: f64,
    manager_decision_inbox: &ManagerDecisionInbox,
    rng: &mut R,
) {
    if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
        let context = ManagerDecisionContext::build(publisher.state(), team_id);
        let sub_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.in_game_adjustments,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::Substitution,
            sub_cooldown,
        ) {
            let is_home = team_id == publisher.state().home_team_id();
            let lineup = if is_home {
                publisher.state().home_lineup().clone()
            } else {
                publisher.state().away_lineup().clone()
            };
            let squad = if is_home {
                publisher.state().home_squad().clone()
            } else {
                publisher.state().away_squad().clone()
            };
            let fatigue_lookup = publisher.state().fatigue_lookup();
            let availability_lookup = |id: &Uuid| publisher.state().availability_for(id);

            let plans = SubstitutionDecisionEngine::evaluate_plans_from_tables(
                &context,
                &context.squad_fatigue_summary,
                &lineup,
                &squad,
                publisher.state().teams.player_attribute_tables(),
                |id| fatigue_lookup.get(id),
                &availability_lookup,
                rng,
            );

            if !plans.is_empty() {
                if let Ok(executed) = execute_substitutions(publisher, team_id, &plans) {
                    if executed > 0 {
                        publisher
                            .state_mut()
                            .mark_decision_triggered(team_id, ManagerDecisionKind::Substitution);
                    }
                }
            }
        }
    } else {
        try_apply_human_substitutions(publisher, team_id, manager_decision_inbox);
    }
}
