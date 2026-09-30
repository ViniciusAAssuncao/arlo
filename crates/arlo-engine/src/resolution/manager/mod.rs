mod substitution;
mod challenge;
mod time_call;
mod valuation;
mod tactics;

pub use substitution::{resolve_substitution_segment, select_substitution};
pub use time_call::should_use_time_call;
pub use tactics::{resolve_tactical_switch_segment, select_tactical_profile};
pub(super) use challenge::review_decision;
