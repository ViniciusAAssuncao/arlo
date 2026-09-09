pub mod ambiguity;
pub mod resolution;
pub mod reviewable_call;

pub use ambiguity::ambiguity_from_duel_outcome;
pub use resolution::resolve_true_ruling;
pub use reviewable_call::{ReviewableCall, ReviewableCallKind};
