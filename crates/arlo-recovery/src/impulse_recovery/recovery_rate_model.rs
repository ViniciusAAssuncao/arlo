use crate::tuning::RecoveryTuningProfile;

pub fn calculate_impulse_reversion_rate(
    determination: f64,
    composure: f64,
    consistency: f64,
    tuning: &RecoveryTuningProfile,
) -> f64 {
    let norm_det = (determination.clamp(1.0, 20.0) - 1.0) / 19.0;
    let norm_comp = (composure.clamp(1.0, 20.0) - 1.0) / 19.0;
    let norm_cons = (consistency.clamp(1.0, 20.0) - 1.0) / 19.0;

    let attr_contribution = (norm_det * tuning.impulse_determination_weight)
        + (norm_comp * tuning.impulse_composure_weight)
        + (norm_cons * tuning.impulse_consistency_weight);

    let rate = tuning.impulse_base_daily_reversion_rate + attr_contribution * 0.25;
    rate.clamp(0.05, 0.85)
}
