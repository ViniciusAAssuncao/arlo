use crate::domain::conditioning_profile::ConditioningProfile;
use crate::domain::fatigue_condition::FatigueCondition;
use crate::domain::impulse_condition::ImpulseCondition;
use crate::domain::injury_record::InjuryRecord;
use crate::domain::injury_status_kind::InjuryStatusKind;
use arlo_domain::domain::invariant_violation::InvariantViolation;
use arlo_domain::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerCondition {
    player_id: Uuid,
    fatigue: FatigueCondition,
    impulse: ImpulseCondition,
    conditioning: ConditioningProfile,
    active_injury: Option<InjuryRecord>,
    last_update_year: u32,
    last_update_day: u32,
    last_match_year: Option<u32>,
    last_match_day: Option<u32>,
}

impl PlayerCondition {
    pub fn new(
        player_id: Uuid,
        fatigue: FatigueCondition,
        impulse: ImpulseCondition,
        conditioning: ConditioningProfile,
        active_injury: Option<InjuryRecord>,
        last_update_year: u32,
        last_update_day: u32,
        last_match_year: Option<u32>,
        last_match_day: Option<u32>,
    ) -> DomainResult<Self> {
        match (last_match_year, last_match_day) {
            (Some(_), None) | (None, Some(_)) => {
                return Err(DomainError::InvalidInvariant {
                    field: "last_match_timestamp".to_string(),
                    violation: InvariantViolation::MissingRequiredValue,
                });
            }
            _ => {}
        }

        Ok(Self {
            player_id,
            fatigue,
            impulse,
            conditioning,
            active_injury,
            last_update_year,
            last_update_day,
            last_match_year,
            last_match_day,
        })
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn fatigue(&self) -> &FatigueCondition {
        &self.fatigue
    }

    pub fn impulse(&self) -> &ImpulseCondition {
        &self.impulse
    }

    pub fn conditioning(&self) -> &ConditioningProfile {
        &self.conditioning
    }

    pub fn active_injury(&self) -> Option<&InjuryRecord> {
        self.active_injury.as_ref()
    }

    pub fn last_update_year(&self) -> u32 {
        self.last_update_year
    }

    pub fn last_update_day(&self) -> u32 {
        self.last_update_day
    }

    pub fn last_match_year(&self) -> Option<u32> {
        self.last_match_year
    }

    pub fn last_match_day(&self) -> Option<u32> {
        self.last_match_day
    }

    pub fn status(&self) -> InjuryStatusKind {
        match &self.active_injury {
            Some(injury) if injury.days_remaining() > 0 => InjuryStatusKind::Injured,
            Some(injury) if injury.observation_days_remaining() > 0 => {
                InjuryStatusKind::Observation
            }
            _ => InjuryStatusKind::Healthy,
        }
    }
}
