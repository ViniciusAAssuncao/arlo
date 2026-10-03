use crate::domain::validation::{validate_hex_color, validate_integer_range, validate_not_empty};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    id: Uuid,
    name: String,
    country_id: Uuid,
    league_id: Option<Uuid>,
    founded_at_unix_seconds: i64,
    prestige: i32,
    min_attendance: Option<i32>,
    max_attendance: Option<i32>,
    primary_color_hex: Option<String>,
    secondary_color_hex: Option<String>,
    home_venue_id: Option<Uuid>,
}

impl Team {
    pub fn builder(
        id: Uuid,
        name: impl Into<String>,
        country_id: Uuid,
        founded_at_unix_seconds: i64,
        prestige: i32,
    ) -> TeamBuilder {
        TeamBuilder::new(
            id,
            name.into(),
            country_id,
            founded_at_unix_seconds,
            prestige,
        )
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn country_id(&self) -> Uuid {
        self.country_id
    }

    pub fn league_id(&self) -> Option<Uuid> {
        self.league_id
    }

    pub fn founded_at_unix_seconds(&self) -> i64 {
        self.founded_at_unix_seconds
    }

    pub fn prestige(&self) -> i32 {
        self.prestige
    }

    pub fn min_attendance(&self) -> Option<i32> {
        self.min_attendance
    }

    pub fn max_attendance(&self) -> Option<i32> {
        self.max_attendance
    }

    pub fn primary_color_hex(&self) -> Option<&str> {
        self.primary_color_hex.as_deref()
    }

    pub fn secondary_color_hex(&self) -> Option<&str> {
        self.secondary_color_hex.as_deref()
    }

    pub fn home_venue_id(&self) -> Option<Uuid> {
        self.home_venue_id
    }
}

#[derive(Debug, Clone)]
pub struct TeamBuilder {
    id: Uuid,
    name: String,
    country_id: Uuid,
    league_id: Option<Uuid>,
    founded_at_unix_seconds: i64,
    prestige: i32,
    min_attendance: Option<i32>,
    max_attendance: Option<i32>,
    primary_color_hex: Option<String>,
    secondary_color_hex: Option<String>,
    home_venue_id: Option<Uuid>,
}

impl TeamBuilder {
    pub fn new(
        id: Uuid,
        name: String,
        country_id: Uuid,
        founded_at_unix_seconds: i64,
        prestige: i32,
    ) -> Self {
        Self {
            id,
            name,
            country_id,
            league_id: None,
            founded_at_unix_seconds,
            prestige,
            min_attendance: None,
            max_attendance: None,
            primary_color_hex: None,
            secondary_color_hex: None,
            home_venue_id: None,
        }
    }

    pub fn with_league_id(mut self, league_id: Option<Uuid>) -> Self {
        self.league_id = league_id;
        self
    }

    pub fn with_primary_color_hex(mut self, primary_color_hex: Option<String>) -> Self {
        self.primary_color_hex = primary_color_hex;
        self
    }

    pub fn with_secondary_color_hex(mut self, secondary_color_hex: Option<String>) -> Self {
        self.secondary_color_hex = secondary_color_hex;
        self
    }

    pub fn with_home_venue_id(mut self, home_venue_id: Option<Uuid>) -> Self {
        self.home_venue_id = home_venue_id;
        self
    }

    pub fn with_attendance_references(
        mut self,
        min_attendance: Option<i32>,
        max_attendance: Option<i32>,
    ) -> Self {
        self.min_attendance = min_attendance;
        self.max_attendance = max_attendance;
        self
    }

    pub fn build(self) -> DomainResult<Team> {
        validate_not_empty(&self.name, "name")?;
        validate_integer_range(self.prestige, 0, 1000, "prestige")?;

        if let Some(value) = self.min_attendance {
            validate_integer_range(value, 0, i32::MAX, "min_attendance")?;
        }
        if let Some(value) = self.max_attendance {
            validate_integer_range(value, 0, i32::MAX, "max_attendance")?;
        }
        if self.min_attendance.is_some() != self.max_attendance.is_some() {
            return Err(crate::error::DomainError::InvalidInvariant {
                field: "attendance_references".to_string(),
                violation:
                    crate::domain::invariant_violation::InvariantViolation::MissingRequiredValue,
            });
        }
        if let (Some(minimum), Some(maximum)) = (self.min_attendance, self.max_attendance) {
            if minimum > maximum {
                return Err(crate::error::DomainError::InvalidInvariant {
                    field: "min_attendance".to_string(),
                    violation:
                        crate::domain::invariant_violation::InvariantViolation::OutOfIntegerRange {
                            min: 0,
                            max: maximum,
                        },
                });
            }
        }

        if let Some(ref color) = self.primary_color_hex {
            validate_hex_color(color, "primary_color_hex")?;
        }
        if let Some(ref color) = self.secondary_color_hex {
            validate_hex_color(color, "secondary_color_hex")?;
        }

        Ok(Team {
            id: self.id,
            name: self.name,
            country_id: self.country_id,
            league_id: self.league_id,
            founded_at_unix_seconds: self.founded_at_unix_seconds,
            prestige: self.prestige,
            min_attendance: self.min_attendance,
            max_attendance: self.max_attendance,
            primary_color_hex: self.primary_color_hex,
            secondary_color_hex: self.secondary_color_hex,
            home_venue_id: self.home_venue_id,
        })
    }
}
