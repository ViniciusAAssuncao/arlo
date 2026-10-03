use crate::input::{MatchInput, TeamInput};
use crate::state::{MatchState, TeamState};
use arlo_domain::{AttributeKey, Manager};
use uuid::Uuid;

pub(super) fn team_pair<'a>(
    input: &'a MatchInput,
    state: &'a MatchState,
    team_id: Uuid,
) -> Option<(&'a TeamInput, &'a TeamState)> {
    if team_id == input.home().team_id() {
        Some((input.home(), state.home()))
    } else if team_id == input.away().team_id() {
        Some((input.away(), state.away()))
    } else {
        None
    }
}

pub(super) fn manager_value(input: &MatchInput, manager: &Manager, key: AttributeKey) -> f64 {
    manager
        .attributes()
        .iter()
        .find(|entry| {
            input
                .manager_attribute_keys()
                .get(&entry.attribute_definition_id())
                == Some(&key)
        })
        .map_or(10.0, |entry| f64::from(entry.value()))
}
