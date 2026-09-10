use crate::attributes::PlayerAttributeTable;
use crate::physical::models::aerobic::calculate_age_degradation;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Speed, ATHLETIC_BMI_REFERENCE, ENERGY_COST_OF_RUNNING_J_PER_KG_M};
use std::collections::HashMap;
use uuid::Uuid;

pub fn estimate_body_mass(height_m: f64, strength: f64) -> f64 {
    let h = height_m.clamp(1.50, 2.30);
    let norm_str = (strength.clamp(0.0, 20.0)) / 20.0;
    let bmi = ATHLETIC_BMI_REFERENCE + (norm_str - 0.5) * 4.0;
    bmi * h * h
}

pub fn calculate_max_sprint_speed(
    pace: f64,
    acceleration: f64,
    height_m: f64,
    mass_kg: f64,
) -> f64 {
    let norm_pace = (pace.clamp(0.0, 20.0)) / 20.0;
    let norm_accel = (acceleration.clamp(0.0, 20.0)) / 20.0;
    let power_to_weight = (norm_accel * 0.6 + norm_pace * 0.4) / (mass_kg / 75.0).sqrt().max(0.7);
    let base_speed = 4.8 + (norm_pace * 4.2) + (power_to_weight * 1.5) + (height_m - 1.80) * 0.4;
    base_speed.clamp(4.0, 11.5)
}

pub fn calculate_critical_speed(
    stamina: f64,
    natural_fitness: f64,
    pace: f64,
    age_years: f64,
) -> f64 {
    let norm_stamina = (stamina.clamp(0.0, 20.0)) / 20.0;
    let norm_fitness = (natural_fitness.clamp(0.0, 20.0)) / 20.0;
    let norm_pace = (pace.clamp(0.0, 20.0)) / 20.0;
    let age_factor = calculate_age_degradation(age_years);
    let v_crit =
        (2.6 + (norm_stamina * 2.0) + (norm_fitness * 1.4) + (norm_pace * 0.6)) * age_factor;
    v_crit.clamp(2.0, 6.8)
}

pub fn calculate_max_w_prime(strength: f64, acceleration: f64, mass_kg: f64) -> f64 {
    let norm_str = (strength.clamp(0.0, 20.0)) / 20.0;
    let norm_acc = (acceleration.clamp(0.0, 20.0)) / 20.0;
    let joules_per_kg = 140.0 + (norm_str * 130.0) + (norm_acc * 170.0);
    mass_kg * joules_per_kg
}

pub fn calculate_desired_cruise_speed(
    critical_speed: f64,
    work_rate: f64,
    positioning: f64,
) -> f64 {
    let norm_wr = (work_rate.clamp(0.0, 20.0)) / 20.0;
    let norm_pos = (positioning.clamp(0.0, 20.0)) / 20.0;
    let factor = 0.38 + (norm_wr * 0.34) + (norm_pos * 0.16);
    (critical_speed * factor).clamp(0.8, critical_speed)
}

pub fn calculate_max_acceleration(
    acceleration: f64,
    agility: f64,
    strength: f64,
    mass_kg: f64,
    fatigue_multiplier: f64,
) -> f64 {
    let norm_acc = (acceleration.clamp(0.0, 20.0)) / 20.0;
    let norm_ag = (agility.clamp(0.0, 20.0)) / 20.0;
    let norm_str = (strength.clamp(0.0, 20.0)) / 20.0;
    let power_factor =
        (norm_acc * 0.5 + norm_str * 0.3 + norm_ag * 0.2) / (mass_kg / 75.0).sqrt().max(0.7);
    let base_accel = 2.4 + power_factor * 4.8;
    (base_accel * fatigue_multiplier.clamp(0.3, 1.0)).clamp(1.0, 8.5)
}

pub fn calculate_player_body_mass(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let strength = extract_attribute_value(&table, AttributeKey::Strength);
    estimate_body_mass(player.height_m(), strength)
}

pub fn calculate_player_max_sprint_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let pace = extract_attribute_value(&table, AttributeKey::Pace);
    let accel = extract_attribute_value(&table, AttributeKey::Acceleration);
    let mass = calculate_player_body_mass(player, attribute_keys);
    let val = calculate_max_sprint_speed(pace, accel, player.height_m(), mass);
    Speed::new(val)
}

pub fn calculate_player_critical_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    current_time_unix_seconds: i64,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let stamina = extract_attribute_value(&table, AttributeKey::Stamina);
    let fitness = extract_attribute_value(&table, AttributeKey::NaturalFitness);
    let pace = extract_attribute_value(&table, AttributeKey::Pace);
    let age =
        crate::physical::models::aerobic::calculate_player_age(player, current_time_unix_seconds);
    let val = calculate_critical_speed(stamina, fitness, pace, age);
    Speed::new(val)
}

pub fn calculate_player_max_w_prime(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let strength = extract_attribute_value(&table, AttributeKey::Strength);
    let accel = extract_attribute_value(&table, AttributeKey::Acceleration);
    let mass = calculate_player_body_mass(player, attribute_keys);
    calculate_max_w_prime(strength, accel, mass)
}

pub fn calculate_player_desired_cruise_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    current_time_unix_seconds: i64,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let v_crit =
        calculate_player_critical_speed(player, attribute_keys, current_time_unix_seconds).value();
    let work_rate = extract_attribute_value(&table, AttributeKey::WorkRate);
    let positioning = extract_attribute_value(&table, AttributeKey::Positioning);
    let val = calculate_desired_cruise_speed(v_crit, work_rate, positioning);
    Speed::new(val)
}

pub fn calculate_player_max_acceleration(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let accel = extract_attribute_value(&table, AttributeKey::Acceleration);
    let agility = extract_attribute_value(&table, AttributeKey::Agility);
    let strength = extract_attribute_value(&table, AttributeKey::Strength);
    let mass = calculate_player_body_mass(player, attribute_keys);
    calculate_max_acceleration(accel, agility, strength, mass, fatigue_multiplier)
}

pub fn calculate_metabolic_work_rate(
    speed_m_s: f64,
    critical_speed_m_s: f64,
    mass_kg: f64,
    intensity_multiplier: f64,
) -> f64 {
    let speed_excess = (speed_m_s - critical_speed_m_s).max(0.0);
    let running_cost_rate = speed_excess * mass_kg * ENERGY_COST_OF_RUNNING_J_PER_KG_M;
    let duel_intensity_rate = if intensity_multiplier > 1.0 {
        (intensity_multiplier - 1.0) * mass_kg * 1.5
    } else {
        0.0
    };
    (running_cost_rate + duel_intensity_rate) * intensity_multiplier.max(1.0)
}
