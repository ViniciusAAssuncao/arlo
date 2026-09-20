use arlo_domain::domain::invariant_violation::InvariantViolation;
use arlo_domain::error::{DomainError, DomainResult};
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InjuryRecord {
    id: Uuid,
    injury_definition_id: Uuid,
    body_region: BodyRegion,
    severity_grade: InjurySeverityGrade,
    days_remaining: u32,
    observation_days_remaining: u32,
    is_relapse: bool,
    original_injury_id: Option<Uuid>,
}

impl InjuryRecord {
    pub fn new(
        id: Uuid,
        injury_definition_id: Uuid,
        body_region: BodyRegion,
        severity_grade: InjurySeverityGrade,
        days_remaining: u32,
        observation_days_remaining: u32,
        is_relapse: bool,
        original_injury_id: Option<Uuid>,
    ) -> DomainResult<Self> {
        if is_relapse && original_injury_id.is_none() {
            return Err(DomainError::InvalidInvariant {
                field: "original_injury_id".to_string(),
                violation: InvariantViolation::MissingRequiredValue,
            });
        }

        if !is_relapse && original_injury_id.is_some() {
            return Err(DomainError::InvalidInvariant {
                field: "original_injury_id".to_string(),
                violation: InvariantViolation::UnexpectedValue,
            });
        }

        Ok(Self {
            id,
            injury_definition_id,
            body_region,
            severity_grade,
            days_remaining,
            observation_days_remaining,
            is_relapse,
            original_injury_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn injury_definition_id(&self) -> Uuid {
        self.injury_definition_id
    }

    pub fn body_region(&self) -> BodyRegion {
        self.body_region
    }

    pub fn severity_grade(&self) -> InjurySeverityGrade {
        self.severity_grade
    }

    pub fn days_remaining(&self) -> u32 {
        self.days_remaining
    }

    pub fn observation_days_remaining(&self) -> u32 {
        self.observation_days_remaining
    }

    pub fn is_relapse(&self) -> bool {
        self.is_relapse
    }

    pub fn original_injury_id(&self) -> Option<Uuid> {
        self.original_injury_id
    }
}
