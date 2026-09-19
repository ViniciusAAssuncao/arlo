use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_player_contribution;
use crate::physical::PhysicalState;
use arlo_domain::{Player, Position};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn best_replacement<'a>(
    outgoing_position: Position,
    candidates: &'a [Arc<Player>],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
) -> Option<&'a Arc<Player>> {
    candidates.iter().max_by(|a, b| {
        let table_a = attribute_tables
            .get(&a.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let table_b = attribute_tables
            .get(&b.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

        let initial_state = PhysicalState::initial();
        let contrib_a = calculate_player_contribution(a.as_ref(), table_a, outgoing_position, &initial_state).value();
        let contrib_b = calculate_player_contribution(b.as_ref(), table_b, outgoing_position, &initial_state).value();

        contrib_a.partial_cmp(&contrib_b).unwrap_or(std::cmp::Ordering::Equal)
    })
}
