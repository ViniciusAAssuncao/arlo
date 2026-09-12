use arlo_engine::EngineError;

#[derive(thiserror::Error, Debug)]
pub enum MatchRunnerError {
    #[error("Maximum iteration limit reached ({max_iterations}) without match finishing")]
    MaxIterationsExceeded { max_iterations: usize },
    #[error(transparent)]
    Engine(#[from] EngineError),
}

pub type MatchRunnerResult<T> = Result<T, MatchRunnerError>;