use super::TeamInput;
use arlo_domain::{AttributeDefinition, AttributeKey};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct PlayerAttributeIndex {
    defined: [bool; AttributeKey::COUNT],
    players: HashMap<Uuid, (Uuid, [Option<f64>; AttributeKey::COUNT])>,
}

impl PlayerAttributeIndex {
    pub(crate) fn new(definitions: &[AttributeDefinition], teams: [&TeamInput; 2]) -> Self {
        let keys: HashMap<_, _> = definitions
            .iter()
            .map(|definition| (definition.id(), definition.key()))
            .collect();
        let mut defined = [false; AttributeKey::COUNT];
        for definition in definitions {
            defined[definition.key().index()] = true;
        }
        let players = teams
            .into_iter()
            .flat_map(|team| {
                team.roster()
                    .iter()
                    .map(move |player| (team.team_id(), player))
            })
            .map(|(team_id, player)| {
                let mut values = [None; AttributeKey::COUNT];
                for attribute in player.attributes() {
                    if let Some(key) = keys.get(&attribute.attribute_definition_id()) {
                        values[key.index()].get_or_insert(f64::from(attribute.value()));
                    }
                }
                (player.id(), (team_id, values))
            })
            .collect();
        Self { defined, players }
    }

    pub(crate) fn is_defined(&self, key: AttributeKey) -> bool {
        self.defined[key.index()]
    }

    pub(crate) fn player(&self, id: Uuid) -> Option<&[Option<f64>; AttributeKey::COUNT]> {
        self.players.get(&id).map(|(_, values)| values)
    }

    pub(crate) fn player_for_team(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Option<&[Option<f64>; AttributeKey::COUNT]> {
        self.players
            .get(&player_id)
            .filter(|(id, _)| *id == team_id)
            .map(|(_, values)| values)
    }
}
