use crate::tuning::RecoveryTuningProfile;
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use rand::Rng;

pub fn estimate_injury_recovery_days(
    severity: InjurySeverityGrade,
    region: BodyRegion,
    natural_fitness: f64,
    age_years: f64,
    tuning: &RecoveryTuningProfile,
) -> u32 {
    let base_days = tuning.base_injury_days(severity) * tuning.region_multiplier(region);

    let norm_fitness = (natural_fitness.clamp(1.0, 20.0) - 1.0) / 19.0;
    let fitness_factor = 1.0 - (norm_fitness - 0.5) * 0.20;

    let age_penalty = if age_years > tuning.energy_age_inflection_years {
        ((age_years - tuning.energy_age_inflection_years) * 0.02).min(0.30)
    } else {
        0.0
    };
    let age_factor = 1.0 + age_penalty;

    let jitter: f64 = rand::thread_rng().gen_range(0.88..=1.12);

    let calculated = (base_days * fitness_factor * age_factor * jitter).round();
    calculated.max(1.0) as u32
}