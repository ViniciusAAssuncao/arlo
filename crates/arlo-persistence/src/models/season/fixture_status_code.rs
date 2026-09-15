use crate::error::{PersistenceError, PersistenceResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixtureStatusCode {
    Scheduled,
    Postponed,
    Completed,
    Cancelled,
}

impl FixtureStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scheduled => "Scheduled",
            Self::Postponed => "Postponed",
            Self::Completed => "Completed",
            Self::Cancelled => "Cancelled",
        }
    }

    pub fn parse(code: &str) -> PersistenceResult<Self> {
        match code {
            "Scheduled" | "scheduled" => Ok(Self::Scheduled),
            "Postponed" | "postponed" => Ok(Self::Postponed),
            "Completed" | "completed" => Ok(Self::Completed),
            "Cancelled" | "cancelled" => Ok(Self::Cancelled),
            _ => Err(PersistenceError::InvalidData(format!(
                "Invalid fixture status: {code}"
            ))),
        }
    }
}
