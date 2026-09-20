use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::substitutions::forced_departure_detection::forced_departures_for_team;
use crate::manager_ai::substitutions::{execute_substitutions, SubstitutionPlan};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, SubstitutionReason};
use arlo_manager_control::{ForcedSubstitutionIntent, ManagerDecisionInbox};
use std::collections::HashSet;
use uuid::Uuid;

pub fn try_apply_human_substitutions(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let intents = inbox.take_substitutions(team_id);
    if intents.is_empty() {
        return false;
    }
    let plans: Vec<SubstitutionPlan> = intents
        .into_iter()
        .map(|intent| SubstitutionPlan {
            outgoing_id: intent.outgoing_player_id(),
            incoming_id: intent.incoming_player_id(),
            reason: SubstitutionReason::Tactical,
        })
        .collect();

    match execute_substitutions(publisher, team_id, &plans) {
        Ok(executed) => {
            if executed > 0 {
                publisher
                    .state_mut()
                    .mark_decision_triggered(team_id, ManagerDecisionKind::Substitution);
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub fn apply_forced_substitution_intents(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    intents: Vec<ForcedSubstitutionIntent>,
    eligible_outgoing_ids: &[Uuid],
) -> usize {
    let eligible_set: HashSet<Uuid> = eligible_outgoing_ids.iter().copied().collect();
    let plans: Vec<SubstitutionPlan> = intents
        .into_iter()
        .filter(|intent| eligible_set.contains(&intent.outgoing_player_id()))
        .map(|intent| SubstitutionPlan {
            outgoing_id: intent.outgoing_player_id(),
            incoming_id: intent.incoming_player_id(),
            reason: SubstitutionReason::Injury,
        })
        .collect();

    if plans.is_empty() {
        return 0;
    }

    execute_substitutions(publisher, team_id, &plans).unwrap_or(0)
}

pub fn resolve_forced_substitutions_for_team(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> Vec<Uuid> {
    let departures = forced_departures_for_team(publisher.state(), team_id);
    if departures.is_empty() {
        publisher
            .state_mut()
            .clear_pending_forced_substitutions(team_id);
        return Vec::new();
    }

    let eligible_outgoing_ids: Vec<Uuid> = departures.iter().map(|(id, _)| *id).collect();
    let intents = inbox.take_forced_substitutions(team_id);
    if !intents.is_empty() {
        apply_forced_substitution_intents(publisher, team_id, intents, &eligible_outgoing_ids);
    }

    let remaining_departures = forced_departures_for_team(publisher.state(), team_id);
    let remaining_ids: Vec<Uuid> = remaining_departures
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    publisher
        .state_mut()
        .set_pending_forced_substitutions(team_id, remaining_ids.clone());

    remaining_ids
}