use arlo_domain::DomainError;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum RecoveryError {
    #[error("SQL error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Resource not found with ID {0}")]
    NotFound(Uuid),
}

pub type RecoveryResult<T> = Result<T, RecoveryError>;
