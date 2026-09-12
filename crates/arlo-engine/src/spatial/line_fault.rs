use crate::attributes::PlayerAttributeTable;
use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::steering::radii::derive_player_physical_radius_from_table;
use arlo_domain::Player;
use arlo_math::units::Position as VectorPosition;

pub fn identify_last_defender<'a>(
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attacking_positive_x: bool,
) -> Option<&'a Player> {
    if defenders.is_empty() {
        return None;
    }
    if attacking_positive_x {
        defenders.iter().copied().max_by(|a, b| {
            let pos_a = spatial_map
                .get_position(&a.id())
                .map(|p| p.raw().0)
                .unwrap_or(f64::NEG_INFINITY);
            let pos_b = spatial_map
                .get_position(&b.id())
                .map(|p| p.raw().0)
                .unwrap_or(f64::NEG_INFINITY);
            pos_a
                .partial_cmp(&pos_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    } else {
        defenders.iter().copied().min_by(|a, b| {
            let pos_a = spatial_map
                .get_position(&a.id())
                .map(|p| p.raw().0)
                .unwrap_or(f64::INFINITY);
            let pos_b = spatial_map
                .get_position(&b.id())
                .map(|p| p.raw().0)
                .unwrap_or(f64::INFINITY);
            pos_a
                .partial_cmp(&pos_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

pub fn calculate_offside_margin_meters(
    receiver_pos: VectorPosition,
    receiver_body_radius: f64,
    defender_pos: VectorPosition,
    defender_body_radius: f64,
    attacking_positive_x: bool,
) -> f64 {
    if attacking_positive_x {
        (receiver_pos.raw().0 - receiver_body_radius)
            - (defender_pos.raw().0 + defender_body_radius)
    } else {
        (defender_pos.raw().0 - defender_body_radius)
            - (receiver_pos.raw().0 + receiver_body_radius)
    }
}

pub fn is_line_fault(
    receiver: &Player,
    receiver_table: &PlayerAttributeTable,
    receiver_pos: VectorPosition,
    defender: &Player,
    defender_table: &PlayerAttributeTable,
    defender_pos: VectorPosition,
    attacking_positive_x: bool,
) -> (bool, f64) {
    let receiver_radius = derive_player_physical_radius_from_table(receiver, receiver_table);
    let defender_radius = derive_player_physical_radius_from_table(defender, defender_table);
    let margin = calculate_offside_margin_meters(
        receiver_pos,
        receiver_radius,
        defender_pos,
        defender_radius,
        attacking_positive_x,
    );
    (margin > 0.0, margin)
}