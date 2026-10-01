use uuid::Uuid;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum AnalyticsError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Out of range: {field} must be between {min} and {max}, got {value}")]
    OutOfRange {
        field: String,
        min: f64,
        max: f64,
        value: f64,
    },
    #[error("Non-finite number encountered: {field}")]
    NonFinite { field: String },
    #[error("Player not found in match context: {0}")]
    PlayerNotFound(Uuid),
    #[error("Slot index {slot_index} out of bounds (max {max_slots})")]
    SlotOutOfBounds {
        slot_index: usize,
        max_slots: usize,
    },
    #[error("Duplicate player registered: {0}")]
    DuplicatePlayer(Uuid),
}

pub type AnalyticsResult<T> = Result<T, AnalyticsError>;