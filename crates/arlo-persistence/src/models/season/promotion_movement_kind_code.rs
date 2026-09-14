use crate::error::{PersistenceError, PersistenceResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromotionMovementKindCode {
    Promoted,
    Relegated,
}

impl PromotionMovementKindCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Promoted => "Promoted",
            Self::Relegated => "Relegated",
        }
    }

    pub fn parse(code: &str) -> PersistenceResult<Self> {
        match code {
            "Promoted" | "promoted" => Ok(Self::Promoted),
            "Relegated" | "relegated" => Ok(Self::Relegated),
            _ => Err(PersistenceError::InvalidData(format!(
                "Invalid promotion movement kind: {code}"
            ))),
        }
    }
}
