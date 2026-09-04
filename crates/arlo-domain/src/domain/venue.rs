use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::sport_constants::{
    PITCH_LENGTH_MIRIM_MAX, PITCH_LENGTH_MIRIM_MIN, PITCH_WIDTH_MIRIM_MAX, PITCH_WIDTH_MIRIM_MIN,
};
use crate::domain::validation::{validate_float_range, validate_not_empty};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VenueKind {
    MatchStadium,
    TrainingCenter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    id: Uuid,
    name: String,
    kind: VenueKind,
    owner_team_id: Option<Uuid>,
    country_id: Uuid,
    capacity: Option<i32>,
    pitch_length_mirim: Option<f64>,
    pitch_width_mirim: Option<f64>,
}

impl Venue {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        kind: VenueKind,
        owner_team_id: Option<Uuid>,
        country_id: Uuid,
        capacity: Option<i32>,
        pitch_length_mirim: Option<f64>,
        pitch_width_mirim: Option<f64>,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;

        match kind {
            VenueKind::MatchStadium => {
                let length = pitch_length_mirim.ok_or_else(|| DomainError::InvalidInvariant {
                    field: "pitch_length_mirim".to_string(),
                    violation: InvariantViolation::MissingRequiredValue,
                })?;
                let width = pitch_width_mirim.ok_or_else(|| DomainError::InvalidInvariant {
                    field: "pitch_width_mirim".to_string(),
                    violation: InvariantViolation::MissingRequiredValue,
                })?;

                validate_float_range(
                    length,
                    PITCH_LENGTH_MIRIM_MIN,
                    PITCH_LENGTH_MIRIM_MAX,
                    "pitch_length_mirim",
                )?;
                validate_float_range(
                    width,
                    PITCH_WIDTH_MIRIM_MIN,
                    PITCH_WIDTH_MIRIM_MAX,
                    "pitch_width_mirim",
                )?;
            }
            VenueKind::TrainingCenter => {
                if pitch_length_mirim.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "pitch_length_mirim".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
                if pitch_width_mirim.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "pitch_width_mirim".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
            }
        }

        Ok(Self {
            id,
            name,
            kind,
            owner_team_id,
            country_id,
            capacity,
            pitch_length_mirim,
            pitch_width_mirim,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> VenueKind {
        self.kind
    }

    pub fn owner_team_id(&self) -> Option<Uuid> {
        self.owner_team_id
    }

    pub fn country_id(&self) -> Uuid {
        self.country_id
    }

    pub fn capacity(&self) -> Option<i32> {
        self.capacity
    }

    pub fn pitch_length_mirim(&self) -> Option<f64> {
        self.pitch_length_mirim
    }

    pub fn pitch_width_mirim(&self) -> Option<f64> {
        self.pitch_width_mirim
    }
}
