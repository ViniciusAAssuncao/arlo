use arlo_engine::EngineError;
use arlo_manager_control::RequiredManagerDecision;

#[derive(thiserror::Error, Debug)]
pub enum MatchRunnerError {
    #[error("Maximum iteration limit reached ({max_iterations}) without match finishing")]
    MaxIterationsExceeded { max_iterations: usize },
    #[error("Match is awaiting a manager decision: {decisions:?}")]
    AwaitingManagerDecision {
        decisions: Vec<RequiredManagerDecision>,
    },
    #[error(transparent)]
    Engine(#[from] EngineError),
}

pub type MatchRunnerResult<T> = Result<T, MatchRunnerError>;
