use arlo_engine::EngineError;
use arlo_manager_control::RequiredManagerDecision;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum MatchRunnerError {
    #[error("Maximum segment limit reached ({max_segments}) without match finishing")]
    SegmentLimitExceeded { max_segments: usize },
    #[error("Match is awaiting a manager decision: {decisions:?}")]
    AwaitingManagerDecision {
        decisions: Vec<RequiredManagerDecision>,
    },
    #[error("PlayCall {play_call_id} is missing from the supplied playbook")]
    UnknownPlayCall { play_call_id: Uuid },
    #[error(transparent)]
    Engine(#[from] EngineError),
}

pub type MatchRunnerResult<T> = Result<T, MatchRunnerError>;
