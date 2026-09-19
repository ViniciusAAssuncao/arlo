use crate::psychology::state::ImpulseState;
use arlo_domain::sport_constants::{impulse_floor_for_baseline, IMPULSE_SCALE_MAX};

pub fn fatigue_depression(exhaustion: f64) -> f64 {
    (1.0 - 0.20 * exhaustion.clamp(0.0, 1.0)).clamp(0.70, 1.0)
}

pub fn calculate_impulse_recovery_tau(stamina: f64, natural_fitness: f64) -> f64 {
    let norm_fitness =
        (natural_fitness.clamp(0.0, 20.0) * 0.6 + stamina.clamp(0.0, 20.0) * 0.4) / 20.0;
    let tau = 180.0 - 130.0 * norm_fitness;
    tau.clamp(40.0, 240.0)
}

pub fn update_impulse(state: &mut ImpulseState, exhaustion: f64, dt_seconds: f64, tau: f64) {
    if dt_seconds <= 0.0 {
        return;
    }
    let baseline = state.baseline();
    let floor = impulse_floor_for_baseline(baseline) * fatigue_depression(exhaustion);
    let target = baseline * (1.0 - 0.15 * exhaustion.clamp(0.0, 1.0)).max(floor);
    let decay_factor = (-dt_seconds / tau.max(1.0)).exp();
    let current_acc = state.accumulator();
    let new_acc = (target + (current_acc - target) * decay_factor)
        .max(floor)
        .clamp(0.0, IMPULSE_SCALE_MAX as f64);
    state.set_accumulator(new_acc);
}