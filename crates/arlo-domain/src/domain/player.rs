use crate::domain::captaincy_role::CaptaincyRole;
use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::player_attribute_value::PlayerAttributeValue;
use crate::domain::player_position::PlayerPosition;
use crate::domain::validation::{
    validate_integer_range, validate_no_duplicate_keys, validate_not_empty,
    validate_positive_finite,
};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    id: Uuid,
    name: String,
    height_m: f64,
    birthdate_unix_seconds: i64,
    nationality_id: Uuid,
    team_id: Option<Uuid>,
    squad_number: Option<i32>,
    captaincy_role: Option<CaptaincyRole>,
    positions: Vec<PlayerPosition>,
    attributes: Vec<PlayerAttributeValue>,
}

impl Player {
    pub fn builder(
        id: Uuid,
        name: impl Into<String>,
        height_m: f64,
        birthdate_unix_seconds: i64,
        nationality_id: Uuid,
    ) -> PlayerBuilder {
        PlayerBuilder::new(
            id,
            name.into(),
            height_m,
            birthdate_unix_seconds,
            nationality_id,
        )
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn height_m(&self) -> f64 {
        self.height_m
    }

    pub fn birthdate_unix_seconds(&self) -> i64 {
        self.birthdate_unix_seconds
    }

    pub fn nationality_id(&self) -> Uuid {
        self.nationality_id
    }

    pub fn team_id(&self) -> Option<Uuid> {
        self.team_id
    }

    pub fn squad_number(&self) -> Option<i32> {
        self.squad_number
    }

    pub fn captaincy_role(&self) -> Option<CaptaincyRole> {
        self.captaincy_role
    }

    pub fn positions(&self) -> &[PlayerPosition] {
        &self.positions
    }

    pub fn attributes(&self) -> &[PlayerAttributeValue] {
        &self.attributes
    }
}

#[derive(Debug, Clone)]
pub struct PlayerBuilder {
    id: Uuid,
    name: String,
    height_m: f64,
    birthdate_unix_seconds: i64,
    nationality_id: Uuid,
    team_id: Option<Uuid>,
    squad_number: Option<i32>,
    captaincy_role: Option<CaptaincyRole>,
    positions: Vec<PlayerPosition>,
    attributes: Vec<PlayerAttributeValue>,
}

impl PlayerBuilder {
    pub fn new(
        id: Uuid,
        name: String,
        height_m: f64,
        birthdate_unix_seconds: i64,
        nationality_id: Uuid,
    ) -> Self {
        Self {
            id,
            name,
            height_m,
            birthdate_unix_seconds,
            nationality_id,
            team_id: None,
            squad_number: None,
            captaincy_role: None,
            positions: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn with_team_id(mut self, team_id: Option<Uuid>) -> Self {
        self.team_id = team_id;
        self
    }

    pub fn with_squad_number(mut self, squad_number: Option<i32>) -> Self {
        self.squad_number = squad_number;
        self
    }

    pub fn with_captaincy_role(mut self, captaincy_role: Option<CaptaincyRole>) -> Self {
        self.captaincy_role = captaincy_role;
        self
    }

    pub fn with_positions(mut self, positions: Vec<PlayerPosition>) -> Self {
        self.positions = positions;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<PlayerAttributeValue>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn build(self) -> DomainResult<Player> {
        validate_not_empty(&self.name, "name")?;
        validate_positive_finite(self.height_m, "height_m")?;

        if let Some(squad_number) = self.squad_number {
            validate_integer_range(squad_number, 0, 100, "squad_number")?;
        }

        if self.captaincy_role.is_some() && self.team_id.is_none() {
            return Err(DomainError::InvalidInvariant {
                field: "captaincy_role".to_string(),
                violation: InvariantViolation::MissingRequiredValue,
            });
        }

        validate_no_duplicate_keys(
            &self.positions,
            |p| p.position(),
            "positions",
            "position",
        )?;

        validate_no_duplicate_keys(
            &self.attributes,
            |a| a.attribute_definition_id(),
            "attributes",
            "attribute_definition_id",
        )?;

        Ok(Player {
            id: self.id,
            name: self.name,
            height_m: self.height_m,
            birthdate_unix_seconds: self.birthdate_unix_seconds,
            nationality_id: self.nationality_id,
            team_id: self.team_id,
            squad_number: self.squad_number,
            captaincy_role: self.captaincy_role,
            positions: self.positions,
            attributes: self.attributes,
        })
    }
}
