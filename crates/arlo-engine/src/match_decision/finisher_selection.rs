use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::{
    calculate_player_target_weight, calculate_player_target_weight_from_table,
    player_base_reception_weight, player_base_reception_weight_from_table,
    select_target_from_tables, ReceptionRole,
};
use crate::physical::PhysicalState;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Pitch, Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn player_base_finishing_weight_from_table(
    table: &PlayerAttributeTable,
    state: Option<&PhysicalState>,
) -> f64 {
    player_base_reception_weight_from_table(table, ReceptionRole::Finisher, state)
}

pub fn player_base_finishing_weight(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: Option<&PhysicalState>,
) -> f64 {
    player_base_reception_weight(player, attribute_keys, ReceptionRole::Finisher, state)
}

pub fn calculate_player_finishing_weight_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    state: Option<&PhysicalState>,
) -> f64 {
    calculate_player_target_weight_from_table(
        player,
        table,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
        state,
    )
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
    state: Option<&PhysicalState>,
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
        state,
    )
}

pub fn select_finisher_from_tables<F, R>(
    candidates: &[&Player],
    _role_index_for_play: Option<&HashMap<Uuid, SlotRole>>,
    _is_bonus_phase: bool,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    select_target_from_tables(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_tables,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
        fatigue_for,
        rng,
    )
}
