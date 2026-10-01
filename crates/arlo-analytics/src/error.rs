#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum AnalyticsError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type AnalyticsResult<T> = Result<T, AnalyticsError>;