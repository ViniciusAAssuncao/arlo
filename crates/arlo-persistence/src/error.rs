#[derive(thiserror::Error, Debug)]
pub enum PersistenceError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Domain(#[from] arlo_domain::DomainError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;
