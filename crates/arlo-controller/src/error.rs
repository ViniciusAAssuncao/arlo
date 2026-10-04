#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    Persistence(#[from] arlo_persistence::PersistenceError),
    #[error(transparent)]
    Analytics(#[from] arlo_analytics::AnalyticsError),
    #[error(transparent)]
    Award(#[from] arlo_awards::AwardError),
    #[error(transparent)]
    AwardCatalog(#[from] arlo_catalog::AwardCatalogError),
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
