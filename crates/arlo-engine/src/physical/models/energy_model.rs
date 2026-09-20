use crate::physical::state::PhysicalState;
use crate::physical::systems::degradation::physical_attribute_modifier;

pub fn physical_multiplier(state: &PhysicalState) -> f64 {
    physical_attribute_modifier(state)
}

pub fn fatigue_multiplier(state: &PhysicalState) -> f64 {
    physical_multiplier(state)
}