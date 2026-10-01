use crate::domain::InjuryRecord;
use crate::error::{RecoveryError, RecoveryResult};
use crate::injury_recovery::recovery_profile::{
    choose_treatment, profile_for, sample_profile_days, RecoveryProfiles, TreatmentKind,
};
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use uuid::Uuid;

pub fn register_injury(
    id: Uuid,
    injury_definition_id: Uuid,
    body_region: BodyRegion,
    severity_grade: InjurySeverityGrade,
    natural_fitness: f64,
    age_years: f64,
    profiles: &RecoveryProfiles,
) -> RecoveryResult<(InjuryRecord, TreatmentKind)> {
    let treatment = choose_treatment(
        profiles,
        injury_definition_id,
        severity_grade,
        age_years,
        natural_fitness,
    );
    let profile = profile_for(profiles, injury_definition_id, severity_grade, treatment)
        .ok_or_else(|| RecoveryError::InvalidData(format!(
            "Missing recovery profile for {injury_definition_id} {severity_grade:?} {}",
            treatment.as_str(),
        )))?;
    let expected_days = sample_profile_days(profile, age_years, natural_fitness);

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
