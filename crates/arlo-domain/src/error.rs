use crate::domain::InvariantViolation;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Invalid invariant on {field}: {violation}")]
    InvalidInvariant {
        field: String,
        violation: InvariantViolation,
    },
}

pub type DomainResult<T> = Result<T, DomainError>;
