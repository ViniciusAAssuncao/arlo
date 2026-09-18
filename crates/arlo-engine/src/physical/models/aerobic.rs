use crate::physical::state::PhysicalState;
use arlo_domain::Player;

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

pub fn calculate_event_energy_decay(
    live_duration_seconds: f64,
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
) -> f64 {
    if live_duration_seconds <= 0.0 {
        return 0.0;
    }
    let norm_stamina = (stamina.clamp(0.0, 20.0)) / 20.0;
    let norm_fitness = (natural_fitness.clamp(0.0, 20.0)) / 20.0;
    let resilience = 0.55 * norm_stamina + 0.45 * norm_fitness;
    let age_penalty = 2.0 - calculate_age_degradation(age_years);
    let base_decay_rate = 0.0012;
    (live_duration_seconds * base_decay_rate * (1.6 - resilience) * age_penalty).clamp(0.0, 0.15)
}

pub fn apply_event_energy_decay(
    state: &mut PhysicalState,
    live_duration_seconds: f64,
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
) {
    let decay = calculate_event_energy_decay(live_duration_seconds, stamina, natural_fitness, age_years);
    let new_energy = (state.energy() - decay).clamp(0.0, 1.0);
    state.set_energy(new_energy);
}