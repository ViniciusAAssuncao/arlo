use crate::domain::InjuryRecord;
use crate::injury_recovery::recovery_duration_estimator::estimate_injury_recovery_days;
use crate::injury_recovery::recovery_profile::{
    choose_treatment, profile_for, sample_profile_days, RecoveryProfiles, TreatmentKind,
};
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
    profiles: &RecoveryProfiles,
) -> DomainResult<(InjuryRecord, TreatmentKind)> {
    let treatment = choose_treatment(
        profiles,
        injury_definition_id,
        severity_grade,
        age_years,
        natural_fitness,
    );
    let expected_days = profile_for(profiles, injury_definition_id, severity_grade, treatment)
        .map(|profile| sample_profile_days(profile, age_years, natural_fitness))
        .unwrap_or_else(|| estimate_injury_recovery_days(
            severity_grade,
            body_region,
            natural_fitness,
            age_years,
            tuning,
        ));

    let record = InjuryRecord::new(
        id,
        injury_definition_id,
        body_region,
        severity_grade,
        expected_days,
        0,
        false,
        None,
    )?;
    Ok((record, treatment))
}
