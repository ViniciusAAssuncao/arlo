use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{Player, Position};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_man_marking_target_position(
    target: Position,
    opposing_players: &[&Player],
    opposing_position_index: &HashMap<Uuid, Position>,
    defender_pos: VectorPosition,
) -> Option<VectorPosition> {
    let mut matching_candidates: Vec<&Player> = Vec::new();

    for &player in opposing_players {
        if opposing_position_index.get(&player.id()) == Some(&target) {
            matching_candidates.push(player);
        }
    }

    if matching_candidates.is_empty() {
        return None;
    }

    let offset_m = PROXIMITY_CONTEST_RADIUS_MIRIM * 0.5 * MIRIM_TO_METERS;
    let shifted_x = defender_pos.raw().0 + offset_m;

    Some(VectorPosition::from_components(
        shifted_x,
        defender_pos.raw().1,
        defender_pos.raw().2,
    ))
}