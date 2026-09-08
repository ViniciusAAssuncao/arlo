use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerDriveSnapshot};
use arlo_domain::pitch::ArtroPlacement;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerDriveStats {
    pub player_id: Uuid,
    pub total_drives: u32,
    pub central_drives: u32,
    pub left_lateral_drives: u32,
    pub right_lateral_drives: u32,
    pub max_drives_in_series: u32,
    pub drives_by_row: HashMap<usize, u32>,
}

impl PlayerDriveStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_drives: 0,
            central_drives: 0,
            left_lateral_drives: 0,
            right_lateral_drives: 0,
            max_drives_in_series: 0,
            drives_by_row: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_drives(&self) -> u32 {
        self.total_drives
    }

    pub fn central_drives(&self) -> u32 {
        self.central_drives
    }

    pub fn left_lateral_drives(&self) -> u32 {
        self.left_lateral_drives
    }

    pub fn right_lateral_drives(&self) -> u32 {
        self.right_lateral_drives
    }

    pub fn lateral_drives(&self) -> u32 {
        self.left_lateral_drives + self.right_lateral_drives
    }

    pub fn max_drives_in_series(&self) -> u32 {
        self.max_drives_in_series
    }

    pub fn drives_by_row(&self) -> &HashMap<usize, u32> {
        &self.drives_by_row
    }
}

impl IntoSnapshot for PlayerDriveStats {
    type Snapshot = PlayerDriveSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerDriveSnapshot {
            player_id: self.player_id,
            total_drives: self.total_drives,
            central_drives: self.central_drives,
            left_lateral_drives: self.left_lateral_drives,
            right_lateral_drives: self.right_lateral_drives,
            lateral_drives: self.lateral_drives(),
            max_drives_in_series: self.max_drives_in_series,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerDrivesAggregator {
    stats: HashMap<Uuid, PlayerDriveStats>,
}

impl PlayerDrivesAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerDriveStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerDriveStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerDriveStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerDriveStats> {
        &self.stats
    }

    pub fn record_drive(
        &mut self,
        player_id: Uuid,
        artro_row_index: usize,
        placement: ArtroPlacement,
        drives_in_series: u32,
    ) {
        let stats = self
            .stats
            .entry(player_id)
            .or_insert_with(|| PlayerDriveStats::new(player_id));

        stats.total_drives += 1;
        match placement {
            ArtroPlacement::Central => stats.central_drives += 1,
            ArtroPlacement::LeftLateral => stats.left_lateral_drives += 1,
            ArtroPlacement::RightLateral => stats.right_lateral_drives += 1,
        }

        if drives_in_series > stats.max_drives_in_series {
            stats.max_drives_in_series = drives_in_series;
        }

        *stats.drives_by_row.entry(artro_row_index).or_insert(0) += 1;
    }
}

impl IntoSnapshot for PlayerDrivesAggregator {
    type Snapshot = HashMap<Uuid, PlayerDriveSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerDrivesAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::DriveRecorded(e) = event {
            self.record_drive(
                e.artrine_id(),
                e.artro_row_index(),
                e.placement(),
                e.drives_in_series(),
            );
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}
