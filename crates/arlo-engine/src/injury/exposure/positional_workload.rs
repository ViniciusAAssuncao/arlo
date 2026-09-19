use crate::physical::models::metabolic_power::positional_strain_multiplier;
use arlo_domain::Position;

pub fn position_workload_multiplier(position: Position) -> f64 {
    positional_strain_multiplier(position)
}