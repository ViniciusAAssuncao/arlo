use crate::domain::season::StandingsEntry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupRankedStandingsEntry {
    entry: StandingsEntry,
    group_id: Uuid,
    rank_in_group: u32,
}

impl GroupRankedStandingsEntry {
    pub fn new(entry: StandingsEntry, group_id: Uuid, rank_in_group: u32) -> Self {
        Self {
            entry,
            group_id,
            rank_in_group,
        }
    }

    pub fn entry(&self) -> &StandingsEntry {
        &self.entry
    }

    pub fn standings_entry(&self) -> StandingsEntry {
        self.entry
    }

    pub fn group_id(&self) -> Uuid {
        self.group_id
    }

    pub fn rank_in_group(&self) -> u32 {
        self.rank_in_group
    }
}
