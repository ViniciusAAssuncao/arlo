use crate::domain::InjuryRecord;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::error::DomainResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InjuryProgressionOutcome {
    StillInjured(InjuryRecord),
    TransitionedToObservation(InjuryRecord),
    ObservationProgressed(InjuryRecord),
    FullyRecovered,
}

pub fn advance_injury_days(
    injury: &InjuryRecord,
    days: u32,
    tuning: &RecoveryTuningProfile,
) -> DomainResult<InjuryProgressionOutcome> {
    if injury.days_remaining() > 0 {
        if days >= injury.days_remaining() {
            let base_days = tuning.base_injury_days(injury.severity_grade()) as u32;
            let obs_days = tuning.calculate_observation_days(base_days);
            let updated = InjuryRecord::new(
                injury.id(),
                injury.injury_definition_id(),
                injury.body_region(),
                injury.severity_grade(),
                0,
                obs_days,
                injury.is_relapse(),
                injury.original_injury_id(),
            )?;
            Ok(InjuryProgressionOutcome::TransitionedToObservation(updated))
        } else {
            let remaining = injury.days_remaining() - days;
            let updated = InjuryRecord::new(
                injury.id(),
                injury.injury_definition_id(),
                injury.body_region(),
                injury.severity_grade(),
                remaining,
                0,
                injury.is_relapse(),
                injury.original_injury_id(),
            )?;
            Ok(InjuryProgressionOutcome::StillInjured(updated))
        }
    } else if injury.observation_days_remaining() > 0 {
        if days >= injury.observation_days_remaining() {
            Ok(InjuryProgressionOutcome::FullyRecovered)
        } else {
            let remaining = injury.observation_days_remaining() - days;
            let updated = InjuryRecord::new(
                injury.id(),
                injury.injury_definition_id(),
                injury.body_region(),
                injury.severity_grade(),
                0,
                remaining,
                injury.is_relapse(),
                injury.original_injury_id(),
            )?;
            Ok(InjuryProgressionOutcome::ObservationProgressed(updated))
        }
    } else {
        Ok(InjuryProgressionOutcome::FullyRecovered)
    }
}
