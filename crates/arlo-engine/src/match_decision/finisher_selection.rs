use crate::match_decision::target_selection::{
    calculate_player_target_weight, player_base_reception_weight, select_target, ReceptionRole,
};
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Pitch, Player, Position};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn player_base_finishing_weight(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    player_base_reception_weight(player, attribute_keys, ReceptionRole::Finisher)
}

pub fn calculate_player_finishing_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
) -> f64 {
    calculate_player_target_weight(
        player,
        spatial_map,
        pitch,
        position_index,
        attribute_keys,
        attacking_positive_x,
        ReceptionRole::Finisher,
    )
}

pub fn select_finisher<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    rng: &mut R,
) -> Option<Uuid> {
    select_target(
        candidates,
        spatial_map,
        pitch,
        position_index,
        attribute_keys,
        attacking_positive_x,
        ReceptionRole::Finisher,
        rng,
    )
}