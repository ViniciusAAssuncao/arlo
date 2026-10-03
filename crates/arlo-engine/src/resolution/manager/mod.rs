mod prepared_plan;
mod realignment;
pub use prepared_plan::{preview_prepared_plan, resolve_prepared_plan_segment};
mod substitution;
pub use realignment::resolve_tactical_realignment_segment;
mod challenge;
mod tactics;
mod time_call;
mod valuation;

pub(super) use challenge::review_decision;
pub use substitution::resolve_substitution_segment;
pub use tactics::{resolve_tactical_switch_segment, select_tactical_profile};
pub use time_call::should_use_time_call;
