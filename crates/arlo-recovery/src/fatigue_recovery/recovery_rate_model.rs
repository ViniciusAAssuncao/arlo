use crate::tuning::RecoveryTuningProfile;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DailyFatigueRecoveryRates {
    pub energy_rate: f64,
    pub anaerobic_rate: f64,
}

pub fn calculate_daily_fatigue_recovery_rates(
    stamina: f64,
    natural_fitness: f64,
    age_years: f64,
    conditioning_score: f64,
    tuning: &RecoveryTuningProfile,
) -> DailyFatigueRecoveryRates {
    let norm_stamina = (stamina.clamp(1.0, 20.0) - 1.0) / 19.0;
    let norm_natural_fitness = (natural_fitness.clamp(1.0, 20.0) - 1.0) / 19.0;

    let age_penalty = if age_years > tuning.energy_age_inflection_years {
        ((age_years - tuning.energy_age_inflection_years) * tuning.energy_age_penalty_slope)
            .min(tuning.energy_max_age_penalty)
    } else {
        0.0
    };
    let age_factor = (1.0 - age_penalty).max(0.1);

    let cond_mult = tuning.conditioning_fatigue_multiplier(conditioning_score);

    let energy_attr_contrib = (norm_stamina * tuning.energy_stamina_weight)
        + (norm_natural_fitness * tuning.energy_natural_fitness_weight);
    let energy_rate =
        (tuning.energy_base_daily_recovery + energy_attr_contrib) * age_factor * cond_mult;

    let anaerobic_attr_contrib = norm_natural_fitness * tuning.anaerobic_natural_fitness_weight;
    let anaerobic_rate =
        (tuning.anaerobic_base_daily_recovery + anaerobic_attr_contrib) * cond_mult;

    DailyFatigueRecoveryRates {
        energy_rate: energy_rate.clamp(0.05, 0.95),
        anaerobic_rate: anaerobic_rate.clamp(0.10, 1.00),
    }
}
