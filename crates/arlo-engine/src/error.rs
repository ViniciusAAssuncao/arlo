use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid match input: {0}")]
    InvalidInput(String),
}

pub type EngineResult<T> = Result<T, EngineError>;
