use crate::error::{PersistenceError, PersistenceResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostponementReasonCode {
    GamesPerWeekConflict,
    ManualOverride,
}

impl PostponementReasonCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GamesPerWeekConflict => "GamesPerWeekConflict",
            Self::ManualOverride => "ManualOverride",
        }
    }

    pub fn parse(code: &str) -> PersistenceResult<Self> {
        match code {
            "GamesPerWeekConflict" | "games_per_week_conflict" => Ok(Self::GamesPerWeekConflict),
            "ManualOverride" | "manual_override" => Ok(Self::ManualOverride),
            _ => Err(PersistenceError::InvalidData(format!(
                "Invalid postponement reason: {code}"
            ))),
        }
    }
}
