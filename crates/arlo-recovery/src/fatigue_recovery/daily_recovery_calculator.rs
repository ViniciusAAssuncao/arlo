use crate::domain::FatigueCondition;
use crate::fatigue_recovery::recovery_rate_model::calculate_daily_fatigue_recovery_rates;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;

pub fn calculate_fatigue_recovery(
    current: &FatigueCondition,
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
    conditioning_score: f64,
    days: u32,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<FatigueCondition> {
    if days == 0 {
        return Ok(*current);
    }

    let rates = calculate_daily_fatigue_recovery_rates(
        stamina,
        natural_fitness,
        age_years,
        conditioning_score,
        tuning,
    );

    let energy_deficit = 1.0 - current.energy();
    let new_energy = 1.0 - energy_deficit * (1.0 - rates.energy_rate).powi(days as i32);

    let w_prime_deficit = 1.0 - current.w_prime();
    let new_w_prime = 1.0 - w_prime_deficit * (1.0 - rates.anaerobic_rate).powi(days as i32);

    FatigueCondition::new(new_energy.clamp(0.0, 1.0), new_w_prime.clamp(0.0, 1.0))
}