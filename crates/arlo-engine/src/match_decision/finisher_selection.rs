use crate::match_decision::target_selection::{
    calculate_player_target_weight, player_base_reception_weight, select_target_with_fatigue,
    ReceptionRole,
};
use crate::physical::PhysicalState;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Pitch, Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
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
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
) -> f64 {
    calculate_player_target_weight(
        player,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
    )
}

pub fn select_finisher_or_kicker<F, R>(
    candidates: &[&Player],
    role_index_for_play: &HashMap<Uuid, SlotRole>,
    is_bonus_phase: bool,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: &F,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    if is_bonus_phase {
        if let Some(kicker) = candidates
            .iter()
            .find(|p| role_index_for_play.get(&p.id()) == Some(&SlotRole::Kicker))
        {
            return Some(kicker.id());
        }
    }

    select_finisher_with_fatigue(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        openness_by_player,
        fatigue_for,
        rng,
    )
}

pub fn select_finisher_with_fatigue<F, R>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: &F,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    select_target_with_fatigue(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
        fatigue_for,
        rng,
    )
}

pub fn select_finisher<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    rng: &mut R,
) -> Option<Uuid> {
    select_finisher_with_fatigue(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        openness_by_player,
        &|_| PhysicalState::initial(),
        rng,
    )
}