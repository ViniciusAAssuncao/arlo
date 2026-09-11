use arlo_math::Probability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewableCallKind {
    TurnoverClassification,
    OutOfBoundsClassification,
    DriveValidity,
    FoulClassification,
}

impl ReviewableCallKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TurnoverClassification => "TurnoverClassification",
            Self::OutOfBoundsClassification => "OutOfBoundsClassification",
            Self::DriveValidity => "DriveValidity",
            Self::FoulClassification => "FoulClassification",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewableCall {
    kind: ReviewableCallKind,
    ambiguity: Probability,
    on_field_favors_offense: bool,
    true_favors_offense: bool,
}

impl ReviewableCall {
    pub fn new(
        kind: ReviewableCallKind,
        ambiguity: Probability,
        on_field_favors_offense: bool,
        true_favors_offense: bool,
    ) -> Self {
        Self {
            kind,
            ambiguity,
            on_field_favors_offense,
            true_favors_offense,
        }
    }

    pub fn kind(&self) -> ReviewableCallKind {
        self.kind
    }

    pub fn ambiguity(&self) -> Probability {
        self.ambiguity
    }

    pub fn on_field_favors_offense(&self) -> bool {
        self.on_field_favors_offense
    }

    pub fn true_favors_offense(&self) -> bool {
        self.true_favors_offense
    }
}