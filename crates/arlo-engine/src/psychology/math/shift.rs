use arlo_domain::sport_constants::{impulse_floor_for_baseline, IMPULSE_SCALE_MAX};

pub fn calculate_shift_magnitude(
    base_magnitude: f64,
    effective_lambda: f64,
    momentum_multiplier: f64,
    desperation_dampener: f64,
    is_positive: bool,
    is_home: bool,
    is_turnover_or_series_success: bool,
) -> f64 {
    let crowd_boost = if is_home && is_turnover_or_series_success {
        1.40
    } else {
        1.0
    };

    if is_positive {
        base_magnitude * momentum_multiplier * crowd_boost
    } else {
        base_magnitude * effective_lambda * momentum_multiplier * desperation_dampener
    }
}

pub fn calculate_effective_floor(
    baseline: f64,
    floor_offset: f64,
    fatigue_depression: f64,
) -> f64 {
    let base_floor = impulse_floor_for_baseline(baseline);
    ((base_floor + floor_offset) * fatigue_depression).clamp(0.0, baseline * 0.95)
}

pub fn calculate_effective_ceiling(baseline: f64, effective_det: f64) -> f64 {
    let norm_det = effective_det / 10.0;
    (baseline
        + ((IMPULSE_SCALE_MAX as f64) - baseline) * (0.35 + 0.3 * (norm_det / 2.0)))
        .clamp(baseline, IMPULSE_SCALE_MAX as f64)
}

pub fn calculate_impulse_delta(is_positive: bool, magnitude: f64) -> f64 {
    if is_positive {
        magnitude
    } else {
        -magnitude
    }
}