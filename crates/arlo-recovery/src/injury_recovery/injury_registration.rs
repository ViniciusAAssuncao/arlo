use crate::domain::InjuryRecord;
use crate::injury_recovery::recovery_duration_estimator::estimate_injury_recovery_days;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use uuid::Uuid;

pub fn register_injury(
    id: Uuid,
    injury_definition_id: Uuid,
    body_region: BodyRegion,
    severity_grade: InjurySeverityGrade,
    natural_fitness: f64,
    age_years: f64,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<InjuryRecord> {
    let expected_days = estimate_injury_recovery_days(
        severity_grade,
        body_region,
        natural_fitness,
        age_years,
        tuning,
    );

    InjuryRecord::new(
        id,
        injury_definition_id,
        body_region,
        severity_grade,
        expected_days,
        0,
        false,
        None,
    )
}