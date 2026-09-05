use crate::match_decision::target_selection::{
    calculate_player_target_weight, player_base_reception_weight, position_reception_bias,
    select_target, ReceptionRole,
};
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{Pitch, Player, Position};
use rand::Rng;
use uuid::Uuid;

pub fn position_finishing_bias(position: Position) -> f64 {
    position_reception_bias(position, ReceptionRole::Finisher)
}

pub fn player_base_finishing_weight(player: &Player) -> f64 {
    player_base_reception_weight(player, ReceptionRole::Finisher)
}

pub fn calculate_player_finishing_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    calculate_player_target_weight(
        player,
        spatial_map,
        pitch,
        attacking_positive_x,
        ReceptionRole::Finisher,
    )
}

pub fn select_finisher<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attacking_positive_x: bool,
    rng: &mut R,
) -> Option<Uuid> {
    select_target(
        candidates,
        spatial_map,
        pitch,
        attacking_positive_x,
        ReceptionRole::Finisher,
        rng,
    )
}