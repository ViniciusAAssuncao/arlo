#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Domain(#[from] arlo_domain::DomainError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type DbResult<T> = Result<T, DbError>;