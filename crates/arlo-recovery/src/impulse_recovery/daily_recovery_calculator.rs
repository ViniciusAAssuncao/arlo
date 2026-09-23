use crate::domain::ImpulseCondition;
use crate::impulse_recovery::recovery_rate_model::calculate_impulse_reversion_rate;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;

pub fn calculate_impulse_recovery(
    current: &ImpulseCondition,
    determination: f64,
    composure: f64,
    consistency: f64,
    days: u32,
    is_post_long_injury: bool,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<ImpulseCondition> {
    if days == 0 {
        return Ok(*current);
    }

    let rate = calculate_impulse_reversion_rate(determination, composure, consistency, tuning);

    let target_baseline = if is_post_long_injury {
        (current.baseline() - 6.0).max(10.0)
    } else {
        current.baseline()
    };

    let diff = current.current() - target_baseline;
    let new_current = target_baseline + diff * (1.0 - rate).powi(days as i32);

    ImpulseCondition::new(new_current.clamp(0.0, 100.0), current.baseline())
}
