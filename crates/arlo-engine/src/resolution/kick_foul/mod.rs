mod attempt;
mod award;
mod model;

pub use attempt::resolve_kick_foul_segment;
pub(super) use attempt::resolve_kick_foul_segment_inner;
pub use award::award_kick_foul_segment;
