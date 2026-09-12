use crate::officiating::ReviewableCall;
use crate::world_state::match_state::review_slot::ReviewSlot;

pub type OfficiatingTracker = ReviewSlot<ReviewableCall>;