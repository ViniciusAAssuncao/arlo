use crate::manager_ai::substitutions::replacement_selection::best_replacement_from_tables;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, SubstitutionReason};
use uuid::Uuid;

pub fn execute_forced_injury_substitutions(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
) -> usize {
    let is_home = team_id == publisher.state().home_team_id();
    let current_lineup = if is_home {
        publisher.state().home_lineup().clone()
    } else {
        publisher.state().away_lineup().clone()
    };

    let injured_players: Vec<(Uuid, arlo_domain::Position)> = current_lineup
        .assignments()
        .iter()
        .filter(|a| {
            let pid = a.player().id();
            publisher.state().availability_for(&pid).is_injured()
        })
        .map(|a| (a.player().id(), a.slot().position()))
        .collect();

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