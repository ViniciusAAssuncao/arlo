use crate::officiating::punishment::PunishmentLedgerEntry;
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FoulReviewTracker {
    last_reviewable_foul: Option<(Uuid, FoulReviewRecord)>,
}

impl FoulReviewTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn last_reviewable_foul(&self) -> Option<&(Uuid, FoulReviewRecord)> {
        self.last_reviewable_foul.as_ref()
    }

    pub fn set_last_reviewable_foul(&mut self, team_id: Uuid, record: FoulReviewRecord) {
        self.last_reviewable_foul = Some((team_id, record));
    }

    pub fn clear_last_reviewable_foul(&mut self) {
        self.last_reviewable_foul = None;
    }
}
