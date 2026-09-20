use crate::domain::InjuryRecord;
use crate::injury_recovery::recovery_duration_estimator::estimate_injury_recovery_days;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;
use arlo_domain::InjurySeverityGrade;
use rand::Rng;
use uuid::Uuid;

pub fn calculate_daily_reinjury_probability(
    observed_injury: &InjuryRecord,
    conditioning_score: f64,
    natural_fitness: f64,
    tuning: &RecoveryTuningProfile,
) -> f64 {
    let base_prob = tuning.relapse_base_daily_probability;
    let grade_mult = tuning.relapse_grade_multiplier(observed_injury.severity_grade());

    let cond_clamped = conditioning_score.clamp(0.0, 1.0);
    let cond_factor = 1.0 - (cond_clamped * tuning.relapse_conditioning_mitigation_factor);

    let norm_fitness = (natural_fitness.clamp(1.0, 20.0) - 1.0) / 19.0;
    let fitness_factor = 1.0 - (norm_fitness * tuning.relapse_natural_fitness_weight);

    (base_prob * grade_mult * cond_factor * fitness_factor).clamp(0.001, 0.50)
}

pub fn evaluate_reinjury_risk(
    observed_injury: &InjuryRecord,
    conditioning_score: f64,
    natural_fitness: f64,
    age_years: f64,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<Option<InjuryRecord>> {
    if observed_injury.days_remaining() > 0 || observed_injury.observation_days_remaining() == 0 {
        return Ok(None);
    }

    let prob = calculate_daily_reinjury_probability(
        observed_injury,
        conditioning_score,
        natural_fitness,
        tuning,
    );

    let roll: f64 = rand::thread_rng().gen();
    if roll < prob {
        let relapse_grade = match observed_injury.severity_grade() {
            InjurySeverityGrade::Grade3 => InjurySeverityGrade::Grade2,
            InjurySeverityGrade::Grade2 => InjurySeverityGrade::Grade2,
            InjurySeverityGrade::Grade1 => InjurySeverityGrade::Grade1,
        };

        let duration = estimate_injury_recovery_days(
            relapse_grade,
            observed_injury.body_region(),
            natural_fitness,
            age_years,
            tuning,
        );

        let original_id = observed_injury
            .original_injury_id()
            .unwrap_or_else(|| observed_injury.id());

        let relapse = InjuryRecord::new(
            Uuid::new_v4(),
            observed_injury.injury_definition_id(),
            observed_injury.body_region(),
            relapse_grade,
            duration,
            0,
            true,
            Some(original_id),
        )?;

        Ok(Some(relapse))
    } else {
        Ok(None)
    }
}