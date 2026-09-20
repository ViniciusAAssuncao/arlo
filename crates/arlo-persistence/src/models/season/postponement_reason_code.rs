use crate::error::{PersistenceError, PersistenceResult};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostponementReasonCode {
    GamesPerWeekConflict,
    ManualOverride,
    InsufficientRestGap,
    CollectiveAgreementBlackout,
}

impl PostponementReasonCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GamesPerWeekConflict => "GamesPerWeekConflict",
            Self::ManualOverride => "ManualOverride",
            Self::InsufficientRestGap => "InsufficientRestGap",
            Self::CollectiveAgreementBlackout => "CollectiveAgreementBlackout",
        }
    }

    pub fn parse(code: &str) -> PersistenceResult<Self> {
        match code {
            "GamesPerWeekConflict" | "games_per_week_conflict" => Ok(Self::GamesPerWeekConflict),
            "ManualOverride" | "manual_override" => Ok(Self::ManualOverride),
            "InsufficientRestGap" | "insufficient_rest_gap" => Ok(Self::InsufficientRestGap),
            "CollectiveAgreementBlackout" | "collective_agreement_blackout" => {
                Ok(Self::CollectiveAgreementBlackout)
            }
            _ => Err(PersistenceError::InvalidData(format!(
                "Invalid postponement reason: {code}"
            ))),
        }
    }
}

impl fmt::Display for PostponementReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}