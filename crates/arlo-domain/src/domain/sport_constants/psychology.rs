pub const IMPULSE_SCALE_MAX: u8 = 100;
pub const HOME_IMPULSE_BASELINE_BOOST: f64 = 2.0;
pub const HOME_MOMENTUM_RESILIENCE_BOOST: f64 = 0.10;
pub const MAX_CAPTAINCY_BASELINE_BOOST: f64 = 3.5;
pub const CAPTAINCY_LOSS_AVERSION_BUFFER: f64 = 0.25;

pub fn impulse_floor_for_baseline(baseline: f64) -> f64 {
    let b = baseline.clamp(0.0, 100.0);
    let floor_curve = 0.45 + (0.10 / (1.0 + (-0.05 * (b - 50.0)).exp()));
    (b * floor_curve).clamp(0.0, 100.0)
}
