use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForcedSubstitutionTracker {
    home_pending: Vec<Uuid>,
    away_pending: Vec<Uuid>,
}

impl ForcedSubstitutionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self, is_home: bool) -> &[Uuid] {
        if is_home {
            &self.home_pending
        } else {
            &self.away_pending
        }
    }

    pub fn set_pending(&mut self, is_home: bool, ids: Vec<Uuid>) {
        if is_home {
            self.home_pending = ids;
        } else {
            self.away_pending = ids;
        }
    }

    pub fn clear(&mut self, is_home: bool) {
        if is_home {
            self.home_pending.clear();
        } else {
            self.away_pending.clear();
        }
    }
}