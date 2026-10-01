use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InvariantViolation {
    #[error("must not be empty")]
    Empty,
    #[error("must be positive and finite")]
    NotPositiveFinite,
    #[error("must be between {min} and {max}")]
    OutOfIntegerRange { min: i32, max: i32 },
    #[error("must be between {min} and {max}")]
    OutOfFloatRange { min: f64, max: f64 },
    #[error("must be a valid 6-digit hex color starting with #")]
    InvalidHexColor,
    #[error("duplicate {key_name} found")]
    DuplicateKey { key_name: String },
    #[error("missing required value")]
    MissingRequiredValue,
    #[error("unexpected value")]
    UnexpectedValue,
    #[error("cannot reference itself")]
    SelfReference,
    #[error("expected {expected}, found {actual}")]
    CountMismatch { expected: usize, actual: usize },
}
