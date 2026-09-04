#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid invariant on {field}: {reason}")]
    InvalidInvariant { field: String, reason: String },
}

pub type DomainResult<T> = Result<T, DomainError>;