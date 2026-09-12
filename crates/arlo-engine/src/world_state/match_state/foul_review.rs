use crate::officiating::punishment::PunishmentLedgerEntry;
use crate::world_state::match_state::review_slot::ReviewSlot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoulReviewRecord {
    pub offending_player_id: Uuid,
    pub punishment: PunishmentLedgerEntry,
    pub original_call_correct: bool,
}

impl FoulReviewRecord {
    pub fn new(
        offending_player_id: Uuid,
        punishment: PunishmentLedgerEntry,
        original_call_correct: bool,
    ) -> Self {
        Self {
            offending_player_id,
            punishment,
            original_call_correct,
        }
    }

    pub fn offending_player_id(&self) -> Uuid {
        self.offending_player_id
    }

    pub fn punishment(&self) -> &PunishmentLedgerEntry {
        &self.punishment
    }

    pub fn original_call_correct(&self) -> bool {
        self.original_call_correct
    }
}

pub type FoulReviewTracker = ReviewSlot<FoulReviewRecord>;