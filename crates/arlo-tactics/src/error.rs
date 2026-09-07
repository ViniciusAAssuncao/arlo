#[derive(thiserror::Error, Debug)]
pub enum TacticsError {
    #[error(transparent)]
    Domain(#[from] arlo_domain::DomainError),
    #[error(transparent)]
    Db(#[from] arlo_db::DbError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error("Invalid lineup: {0}")]
    InvalidLineup(String),
    #[error("Invalid instruction key: {0}")]
    InvalidInstructionKey(String),
    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
}

pub type TacticsResult<T> = Result<T, TacticsError>;