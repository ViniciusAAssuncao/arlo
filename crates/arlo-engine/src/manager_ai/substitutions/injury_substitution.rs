use crate::manager_ai::substitutions::decision::SubstitutionPlan;
use crate::manager_ai::substitutions::execution::execute_substitutions;
use crate::manager_ai::substitutions::forced_departure_detection::forced_departures_for_team;
use crate::manager_ai::substitutions::replacement_selection::best_replacement_from_tables;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, SubstitutionReason};
use arlo_manager_control::{ForcedSubstitutionIntent, ManagerDecisionInbox};
use std::collections::HashSet;
use uuid::Uuid;

pub fn execute_forced_injury_substitutions(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
) -> usize {
    let injured_players = forced_departures_for_team(publisher.state(), team_id);

    if injured_players.is_empty() {
        return 0;
    }

    let mut executed = 0;
    for (outgoing_id, position) in injured_players {
        let available_replacements: Vec<_> = {
            let squad = publisher.state().squad_for_team(team_id);
            squad.available_replacements().cloned().collect()
        };

        if available_replacements.is_empty() {
            continue;
        }

        let tables = publisher.state().teams.player_attribute_tables();
        if let Some(replacement) =
            best_replacement_from_tables(position, &available_replacements, tables)
        {
            let incoming = replacement.clone();
            let incoming_id = incoming.id();
            if publisher
                .state_mut()
                .apply_substitution(team_id, outgoing_id, incoming)
                .is_ok()
            {
                let clock_inst = publisher.state().clock().to_instant();
                publisher.emit_substitution_made(
                    team_id,
                    outgoing_id,
                    incoming_id,
                    clock_inst,
                    SubstitutionReason::Injury,
                );
                executed += 1;
            }
        }
    }

    executed
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