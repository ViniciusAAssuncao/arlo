mod decision_dispatch;
pub mod dual_sink;
pub mod error;
pub mod loop_driver;
pub mod manager_ai;
pub mod match_run_result;
pub mod match_run_status;
mod performance_registry;

pub use decision_dispatch::{resolve_segment, resolve_segment_with_registry};
pub use dual_sink::DualEventSink;
pub use error::{MatchRunnerError, MatchRunnerResult};
pub use loop_driver::{
    run_loop, run_loop_with_inbox, run_match, run_match_with_inbox, run_match_with_limit,
    run_match_with_registry, run_match_with_registry_and_play_calls, DEFAULT_MAX_SEGMENTS,
};
pub use match_run_result::MatchRunResult;
pub use match_run_status::MatchRunStatus;
pub use performance_registry::build_default_aggregator_registry;
mod manager_play_call;
