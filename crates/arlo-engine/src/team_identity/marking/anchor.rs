use crate::spatial::proximity::calculate_distance;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{Player, Position};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_man_marking_target_position(
    target: Position,
    opposing_players: &[&Player],
    opposing_position_index: &HashMap<Uuid, Position>,
    spatial_map: &DynamicSpatialMap,
    defender_pos: VectorPosition,
) -> Option<VectorPosition> {
    let mut matching_candidates: Vec<(&Player, VectorPosition)> = Vec::new();

    for &player in opposing_players {
        if opposing_position_index.get(&player.id()) == Some(&target) {
            if let Some(pos) = spatial_map.get_position(&player.id()) {
                matching_candidates.push((player, pos));
            }
        }
    }

    if matching_candidates.is_empty() {
        return None;
    }

    let (best_player, target_live_pos) =
        matching_candidates
            .into_iter()
            .min_by(|(_, pos_a), (_, pos_b)| {
                let dist_a = calculate_distance(defender_pos, *pos_a).value();
                let dist_b = calculate_distance(defender_pos, *pos_b).value();
                dist_a
                    .partial_cmp(&dist_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

    let offset_m = PROXIMITY_CONTEST_RADIUS_MIRIM * 0.5 * MIRIM_TO_METERS;
    let is_target_home = spatial_map.is_home_player(&best_player.id());
    let shifted_x = if is_target_home {
        target_live_pos.raw().0 + offset_m
    } else {
        target_live_pos.raw().0 - offset_m
    };

    Some(VectorPosition::from_components(
        shifted_x,
        target_live_pos.raw().1,
        target_live_pos.raw().2,
    ))
}
