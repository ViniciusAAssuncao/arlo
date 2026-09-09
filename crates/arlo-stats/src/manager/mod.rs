pub mod aggregator;
pub mod decision_log;

pub use aggregator::{aggregate_match, ManagerDecisionAggregator};
pub use decision_log::ManagerDecisionLog;