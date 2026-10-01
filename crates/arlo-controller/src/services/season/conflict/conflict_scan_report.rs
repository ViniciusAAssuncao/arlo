use crate::domain::season::PostponementRecord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictScanReport {
    pub competition_id: Uuid,
    pub conflicts_detected: usize,
    pub postponements_applied: usize,
    pub postponement_records: Vec<PostponementRecord>,
}

impl ConflictScanReport {
    pub fn new(
        competition_id: Uuid,
        conflicts_detected: usize,
        postponements_applied: usize,
        postponement_records: Vec<PostponementRecord>,
    ) -> Self {
        Self {
            competition_id,
            conflicts_detected,
            postponements_applied,
            postponement_records,
        }
    }

    pub fn empty(competition_id: Uuid) -> Self {
        Self {
            competition_id,
            conflicts_detected: 0,
            postponements_applied: 0,
            postponement_records: Vec::new(),
        }
    }

    pub fn competition_id(&self) -> Uuid {
        self.competition_id
    }

    pub fn conflicts_detected(&self) -> usize {
        self.conflicts_detected
    }

    pub fn postponements_applied(&self) -> usize {
        self.postponements_applied
    }

    pub fn postponement_records(&self) -> &[PostponementRecord] {
        &self.postponement_records
    }
}
