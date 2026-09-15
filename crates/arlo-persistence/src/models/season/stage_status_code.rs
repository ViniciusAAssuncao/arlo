use crate::error::{PersistenceError, PersistenceResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageStatusCode {
    Pending,
    Active,
    Completed,
}

impl StageStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Active => "Active",
            Self::Completed => "Completed",
        }
    }

    pub fn parse(code: &str) -> PersistenceResult<Self> {
        match code {
            "Pending" | "pending" => Ok(Self::Pending),
            "Active" | "active" => Ok(Self::Active),
            "Completed" | "completed" => Ok(Self::Completed),
            _ => Err(PersistenceError::InvalidData(format!(
                "Invalid stage status: {code}"
            ))),
        }
    }
}
