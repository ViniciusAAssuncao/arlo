use crate::domain::MoraleCondition;
use crate::morale_recovery::recovery_rate_model::calculate_morale_reversion_rate;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;

pub fn calculate_morale_recovery(
    current: &MoraleCondition,
    determination: f64,
    composure: f64,
    consistency: f64,
    days: u32,
    is_post_long_injury: bool,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<MoraleCondition> {
    if days == 0 {
        return Ok(*current);
    }

    let rate = calculate_morale_reversion_rate(determination, composure, consistency, tuning);

    let target_baseline = if is_post_long_injury {
        (current.baseline() - 6.0).max(10.0)
    } else {
        current.baseline()
    };

    let mut value = current.current();
    for _ in 0..days {
        let diff = target_baseline - value;
        if diff > 0.0 {
            let rebound = (rate * (0.65 + 0.65 * diff / 100.0)).clamp(0.05, 0.65);
            value += diff * rebound;
        } else {
            value += diff * (rate * 0.55).clamp(0.05, 0.45);
        }
    }

    MoraleCondition::new(value.clamp(0.0, 120.0), current.baseline())
}
