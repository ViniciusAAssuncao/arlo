use crate::match_run_result::MatchRunResult;
use arlo_manager_control::RequiredManagerDecision;

pub enum MatchRunStatus {
    Finished(MatchRunResult),
    AwaitingDecision {
        decisions: Vec<RequiredManagerDecision>,
        partial: MatchRunResult,
    },
}
