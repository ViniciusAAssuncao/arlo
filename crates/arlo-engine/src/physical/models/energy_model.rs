use crate::attributes::PlayerAttributeTable;
use crate::physical::state::PhysicalState;
use crate::physical::systems::degradation::physical_attribute_modifier;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn physical_multiplier(state: &PhysicalState, _stamina: f64, _natural_fitness: f64) -> f64 {
    physical_attribute_modifier(state)
}

pub fn compute_player_physical_multiplier(
    _player: &Player,
    physical_state: &PhysicalState,
    _attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    physical_attribute_modifier(physical_state)
}

pub fn fatigue_multiplier(state: &PhysicalState, stamina: f64, natural_fitness: f64) -> f64 {
    physical_multiplier(state, stamina, natural_fitness)
}

pub fn compute_player_fatigue_multiplier(
    player: &Player,
    physical_state: &PhysicalState,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let stamina = extract_attribute_value(&table, AttributeKey::Stamina);
    let natural_fitness =
        extract_attribute_value(&table, AttributeKey::NaturalFitness);
    fatigue_multiplier(physical_state, stamina, natural_fitness)
}
