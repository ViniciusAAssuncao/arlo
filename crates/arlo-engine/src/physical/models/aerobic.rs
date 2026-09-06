use crate::physical::state::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub const SECONDS_PER_YEAR: f64 = 31557600.0;
pub const AGE_DEGRADATION_THRESHOLD: f64 = 30.0;

pub fn calculate_player_age(player: &Player, current_time_unix_seconds: i64) -> f64 {
    if current_time_unix_seconds <= player.birthdate_unix_seconds() {
        if current_time_unix_seconds <= 0 {
            25.0
        } else {
            18.0
        }
    } else {
        ((current_time_unix_seconds - player.birthdate_unix_seconds()) as f64) / SECONDS_PER_YEAR
    }
}

pub fn calculate_age_degradation(age_years: f64) -> f64 {
    if age_years <= AGE_DEGRADATION_THRESHOLD {
        1.0
    } else {
        let excess = age_years - AGE_DEGRADATION_THRESHOLD;
        let factor = 2.0 / (1.0 + (0.075 * excess).exp());
        factor.clamp(0.20, 1.0)
    }
}

pub fn calculate_aerobic_capacity(stamina: f64, natural_fitness: f64, age_years: f64) -> f64 {
    let norm_stamina = stamina.clamp(0.0, 20.0) / 20.0;
    let norm_fitness = natural_fitness.clamp(0.0, 20.0) / 20.0;
    let age_factor = calculate_age_degradation(age_years);
    let base_capacity = 8000.0 + (norm_stamina * 6000.0) + (norm_fitness * 4000.0);
    base_capacity * age_factor
}

pub fn calculate_aerobic_energy_decay(
    distance_mirim: f64,
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
) -> f64 {
    let capacity = calculate_aerobic_capacity(stamina, natural_fitness, age_years);
    let distance_ratio = distance_mirim.max(0.0) / capacity.max(1.0);
    let decay = 1.0 / (1.0 + (distance_ratio.powf(2.4) * 2.2));
    decay.clamp(0.0, 1.0)
}

pub fn calculate_player_aerobic_energy(
    player: &Player,
    cumulative_distance_mirim: f64,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    current_time_unix_seconds: i64,
) -> f64 {
    let stamina = extract_attribute_value(player, attribute_keys, AttributeKey::Stamina);
    let natural_fitness =
        extract_attribute_value(player, attribute_keys, AttributeKey::NaturalFitness);
    let age_years = calculate_player_age(player, current_time_unix_seconds);
    calculate_aerobic_energy_decay(
        cumulative_distance_mirim,
        stamina,
        natural_fitness,
        age_years,
    )
}

pub fn update_physical_state_aerobic(
    state: &mut PhysicalState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    current_time_unix_seconds: i64,
) {
    let energy = calculate_player_aerobic_energy(
        player,
        state.cumulative_distance_mirim(),
        attribute_keys,
        current_time_unix_seconds,
    );
    state.set_energy(energy);
}