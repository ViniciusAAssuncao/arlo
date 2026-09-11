pub mod ambiguity;
pub mod assignment;
pub mod event_translation;
pub mod foul;
pub mod heads_or_tails;
pub mod resolution;
pub mod reviewable_call;

pub use ambiguity::ambiguity_from_duel_outcome;
pub use assignment::draw_match_referees;
pub use event_translation::translate_foul_raised;
pub use foul::*;
pub use heads_or_tails::flip_officiating_coin;
pub use resolution::resolve_true_ruling;
pub use reviewable_call::{ReviewableCall, ReviewableCallKind};
