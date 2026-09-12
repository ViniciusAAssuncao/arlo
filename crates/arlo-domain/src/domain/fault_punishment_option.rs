use crate::domain::punishment_kind::PunishmentKind;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultPunishmentOption {
    id: Uuid,
    fault_definition_id: Uuid,
    kind: PunishmentKind,
    magnitude_min: Option<i32>,
    magnitude_max: Option<i32>,
}

impl FaultPunishmentOption {
    pub fn new(
        id: Uuid,
        fault_definition_id: Uuid,
        kind: PunishmentKind,
        magnitude_min: Option<i32>,
        magnitude_max: Option<i32>,
    ) -> DomainResult<Self> {
        if let (Some(min), Some(max)) = (magnitude_min, magnitude_max) {
            validate_integer_range(min, 0, max, "magnitude_min")?;
        } else if let Some(min) = magnitude_min {
            validate_integer_range(min, 0, i32::MAX, "magnitude_min")?;
        } else if let Some(max) = magnitude_max {
            validate_integer_range(max, 0, i32::MAX, "magnitude_max")?;
        }

        Ok(Self {
            id,
            fault_definition_id,
            kind,
            magnitude_min,
            magnitude_max,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn fault_definition_id(&self) -> Uuid {
        self.fault_definition_id
    }

    pub fn kind(&self) -> PunishmentKind {
        self.kind
    }

    pub fn magnitude_min(&self) -> Option<i32> {
        self.magnitude_min
    }

    pub fn magnitude_max(&self) -> Option<i32> {
        self.magnitude_max
    }
}