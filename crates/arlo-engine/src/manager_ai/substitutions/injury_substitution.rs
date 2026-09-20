use crate::manager_ai::substitutions::forced_departure_detection::forced_departures_for_team;
use crate::manager_ai::substitutions::replacement_selection::best_replacement;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, SubstitutionReason};
use uuid::Uuid;

pub use crate::manager_ai::human_control::{
    apply_forced_substitution_intents, resolve_forced_substitutions_for_team,
};

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
            squad
                .available_replacements()
                .filter(|p| publisher.state().is_player_available(&p.id()))
                .cloned()
                .collect()
        };

        if available_replacements.is_empty() {
            continue;
        }

        let tables = publisher.state().teams.player_attribute_tables();
        if let Some(replacement) =
            best_replacement(position, &available_replacements, tables)
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