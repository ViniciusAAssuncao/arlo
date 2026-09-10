use crate::attributes::PlayerAttributeTable;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn derive_braking_deceleration(agility: f64, balance: f64, fatigue_multiplier: f64) -> f64 {
    let ag = agility.clamp(0.0, 20.0);
    let bal = balance.clamp(0.0, 20.0);
    let base_decel = 3.2 + (ag * 0.18) + (bal * 0.14);
    (base_decel * fatigue_multiplier.clamp(0.5, 1.0)).clamp(1.5, 12.0)
}

pub fn derive_arrival_slowing_radius(
    current_speed: f64,
    agility: f64,
    balance: f64,
    fatigue_multiplier: f64,
) -> f64 {
    let decel = derive_braking_deceleration(agility, balance, fatigue_multiplier);
    let speed = current_speed.max(0.0);
    let stopping_dist = (speed * speed) / (2.0 * decel);
    stopping_dist.max(0.35)
}

pub fn derive_player_arrival_radius_from_table(
    _player: &Player,
    table: &PlayerAttributeTable,
    current_speed: f64,
    fatigue_multiplier: f64,
) -> f64 {
    let agility = extract_attribute_value(table, AttributeKey::Agility);
    let balance = extract_attribute_value(table, AttributeKey::Balance);
    derive_arrival_slowing_radius(current_speed, agility, balance, fatigue_multiplier)
}

pub fn derive_player_arrival_radius(
    player: &Player,
    current_speed: f64,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    derive_player_arrival_radius_from_table(player, &table, current_speed, fatigue_multiplier)
}

pub fn derive_player_physical_radius_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
) -> f64 {
    let height = player.height_m().clamp(1.4, 2.3);
    let strength =
        extract_attribute_value(table, AttributeKey::Strength).clamp(0.0, 20.0);
    height * (0.22 + 0.008 * strength)
}

pub fn derive_player_physical_radius(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    derive_player_physical_radius_from_table(player, &table)
}

pub fn derive_dynamic_separation_radius(
    self_physical_radius: f64,
    neighbor_physical_radius: f64,
    is_teammate: bool,
    approach_speed: f64,
) -> f64 {
    let combined_radius = self_physical_radius + neighbor_physical_radius;
    let relational_factor = if is_teammate { 1.35 } else { 2.40 };
    let base_separation = combined_radius * relational_factor;
    let velocity_expansion = 1.0 + (approach_speed.max(0.0) * 0.15);
    base_separation * velocity_expansion
}

pub fn max_turn_radians_per_tick(agility: f64) -> f64 {
    let clamped_agility = agility.clamp(0.0, 20.0);
    let base_turn = 0.10 + clamped_agility * 0.025;
    apply_saturation(base_turn, 0.45, 0.5)
}
