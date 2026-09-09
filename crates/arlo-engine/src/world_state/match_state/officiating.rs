use crate::officiating::ReviewableCall;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OfficiatingTracker {
    last_reviewable_call: Option<(Uuid, ReviewableCall)>,
}

impl OfficiatingTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn last_reviewable_call(&self) -> Option<&(Uuid, ReviewableCall)> {
        self.last_reviewable_call.as_ref()
    }

    pub fn set_last_reviewable_call(&mut self, team_id: Uuid, call: ReviewableCall) {
        self.last_reviewable_call = Some((team_id, call));
    }

    pub fn clear_last_reviewable_call(&mut self) {
        self.last_reviewable_call = None;
    }
}
