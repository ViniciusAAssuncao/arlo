use crate::physical::state::PhysicalState;
use crate::physical::tuning::EnergyTuningProfile;

pub fn recover_energy(
    current_energy: f64,
    dead_ball_seconds: f64,
    natural_fitness: f64,
    is_time_call: bool,
    profile: &EnergyTuningProfile,
) -> f64 {
    if dead_ball_seconds <= 0.0 || current_energy >= 1.0 {
        return current_energy.clamp(0.0, 1.0);
    }
    let norm_fitness = (natural_fitness.clamp(0.0, 20.0)) / 20.0;
    let mut rate = profile.dead_ball_recovery_rate_per_second() * (0.60 + 0.60 * norm_fitness);
    if is_time_call {
        rate *= profile.time_call_recovery_multiplier();
    }
    let recovered = dead_ball_seconds * rate;
    (current_energy + recovered).clamp(0.0, 1.0)
}

pub fn recover_player_energy(
    state: &mut PhysicalState,
    dead_ball_seconds: f64,
    natural_fitness: f64,
    is_time_call: bool,
    profile: &EnergyTuningProfile,
) {
    let new_energy = recover_energy(
        state.energy(),
        dead_ball_seconds,
        natural_fitness,
        is_time_call,
        profile,
    );
    state.set_energy(new_energy);
}
