use crate::kick_foul::pending::KickFoulPending;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct KickFoulTracker {
    pending: Option<KickFoulPending>,
}

impl KickFoulTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self) -> Option<&KickFoulPending> {
        self.pending.as_ref()
    }

    pub fn set(&mut self, pending: KickFoulPending) {
        self.pending = Some(pending);
    }

    pub fn take(&mut self) -> Option<KickFoulPending> {
        self.pending.take()
    }

    pub fn clear(&mut self) {
        self.pending = None;
    }
}
