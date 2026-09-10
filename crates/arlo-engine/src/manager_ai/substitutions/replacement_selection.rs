use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::fit_calculator::calculate_fit_for_position;
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn best_replacement<'a>(
    outgoing_position: Position,
    candidates: &'a [Arc<Player>],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Option<&'a Arc<Player>> {
    candidates.iter().max_by(|a, b| {
        let fit_a = calculate_fit_for_position(a.as_ref(), outgoing_position);
        let fit_b = calculate_fit_for_position(b.as_ref(), outgoing_position);

        let prof_cmp = fit_a
            .effective_proficiency()
            .partial_cmp(&fit_b.effective_proficiency())
            .unwrap_or(std::cmp::Ordering::Equal);

        if prof_cmp != std::cmp::Ordering::Equal {
            prof_cmp
        } else {
            let ca_a = calculate_player_ca(a.as_ref(), attribute_keys);
            let ca_b = calculate_player_ca(b.as_ref(), attribute_keys);
            ca_a.partial_cmp(&ca_b).unwrap_or(std::cmp::Ordering::Equal)
        }
    })
}