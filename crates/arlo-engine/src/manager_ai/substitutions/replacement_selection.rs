use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::fit_calculator::calculate_fit_for_position;
use arlo_domain::sport_constants::MIN_CURRENT_ABILITY;
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
        let fit_a = calculate_fit_for_position(a.as_ref(), outgoing_position);
        let fit_b = calculate_fit_for_position(b.as_ref(), outgoing_position);

        let table_a = attribute_tables
            .get(&a.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let table_b = attribute_tables
            .get(&b.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

        let ca_a = calculate_player_ca(a.as_ref(), table_a).unwrap_or(MIN_CURRENT_ABILITY) as f64;
        let ca_b = calculate_player_ca(b.as_ref(), table_b).unwrap_or(MIN_CURRENT_ABILITY) as f64;

        let score_a = ca_a * (0.35 + 0.65 * fit_a.efficiency_multiplier());
        let score_b = ca_b * (0.35 + 0.65 * fit_b.efficiency_multiplier());

        score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
    })
}