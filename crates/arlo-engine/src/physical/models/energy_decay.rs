use crate::physical::models::age::calculate_age_degradation;
use crate::physical::state::PhysicalState;
use crate::physical::tuning::EnergyTuningProfile;

pub fn calculate_event_energy_decay(
    live_duration_seconds: f64,
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
    position_workload: f64,
    tempo_mult: f64,
    pressing_mult: f64,
    participated: bool,
    profile: &EnergyTuningProfile,
) -> f64 {
    if live_duration_seconds <= 0.0 {
        return 0.0;
    }
    let norm_stamina = (stamina.clamp(0.0, 20.0)) / 20.0;
    let norm_fitness = (natural_fitness.clamp(0.0, 20.0)) / 20.0;
    let resilience = 0.55 * norm_stamina + 0.45 * norm_fitness;
    let age_penalty = 2.0 - calculate_age_degradation(age_years);

    let tactical_strain = 1.0 + (tempo_mult - 1.0) * profile.tempo_drain_scale()
        + (pressing_mult - 1.0) * profile.pressing_drain_scale();

    let rate = profile.base_drain_per_live_second()
        * position_workload
        * tactical_strain
        * (1.50 - 0.80 * resilience)
        * age_penalty;

    let base_drain = live_duration_seconds * rate;
    let participation_extra = if participated {
        profile.participation_drain_bonus() * (1.20 - 0.40 * resilience)
    } else {
        0.0
    };

    (base_drain + participation_extra).clamp(0.0, 0.08)
}

pub fn apply_event_energy_decay(
    state: &mut PhysicalState,
    decay: f64,
) {
    let new_energy = (state.energy() - decay).clamp(0.0, 1.0);
    state.set_energy(new_energy);
}
