#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type ControllerResult<T> = Result<T, ControllerError>;