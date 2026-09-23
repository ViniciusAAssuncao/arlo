mod decision_dispatch;
pub mod dual_sink;
pub mod error;
pub mod loop_driver;
pub mod match_run_result;
pub mod match_run_status;

pub use dual_sink::DualEventSink;
pub use error::{MatchRunnerError, MatchRunnerResult};
pub use loop_driver::{
    run_loop, run_loop_with_inbox, run_match, run_match_with_inbox, run_match_with_limit,
    run_match_with_registry, DEFAULT_MAX_SEGMENTS,
};
pub use match_run_result::MatchRunResult;
pub use match_run_status::MatchRunStatus;
