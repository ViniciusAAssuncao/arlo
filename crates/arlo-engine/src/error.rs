use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid match input: {0}")]
    InvalidInput(String),
    #[error("invalid match transition: {0}")]
    InvalidTransition(String),
}

pub type EngineResult<T> = Result<T, EngineError>;
