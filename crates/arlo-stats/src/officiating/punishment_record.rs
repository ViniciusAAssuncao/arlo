use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerPunishmentSnapshot};
use arlo_domain::PunishmentKind;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerPunishmentStats {
    pub player_id: Uuid,
    pub punishments_by_kind: HashMap<PunishmentKind, u32>,
    pub total_yardage_loss_mirim: f64,
    pub total_loss_of_down_count: u32,
    pub total_time_penalty_seconds: f64,
    pub total_loss_of_drive_count: u32,
    pub total_expulsions: u32,
    pub total_plays_invalidated: u32,
}

impl PlayerPunishmentStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            punishments_by_kind: HashMap::new(),
            total_yardage_loss_mirim: 0.0,
            total_loss_of_down_count: 0,
            total_time_penalty_seconds: 0.0,
            total_loss_of_drive_count: 0,
            total_expulsions: 0,
            total_plays_invalidated: 0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn punishments_by_kind(&self) -> &HashMap<PunishmentKind, u32> {
        &self.punishments_by_kind
    }

    pub fn count_for_kind(&self, kind: PunishmentKind) -> u32 {
        self.punishments_by_kind.get(&kind).copied().unwrap_or(0)
    }

    pub fn total_yardage_loss_mirim(&self) -> f64 {
        self.total_yardage_loss_mirim
    }

    pub fn total_loss_of_down_count(&self) -> u32 {
        self.total_loss_of_down_count
    }

    pub fn total_time_penalty_seconds(&self) -> f64 {
        self.total_time_penalty_seconds
    }

    pub fn total_loss_of_drive_count(&self) -> u32 {
        self.total_loss_of_drive_count
    }

    pub fn total_expulsions(&self) -> u32 {
        self.total_expulsions
    }

    pub fn total_plays_invalidated(&self) -> u32 {
        self.total_plays_invalidated
    }

    pub fn record_punishment(&mut self, kind: PunishmentKind, magnitude: Option<i32>) {
        *self.punishments_by_kind.entry(kind).or_insert(0) += 1;
        match kind {
            PunishmentKind::YardageLoss => {
                let mirim = magnitude.unwrap_or(5).max(0) as f64;
                self.total_yardage_loss_mirim += mirim;
            }
            PunishmentKind::LossOfDown => {
                let downs = magnitude.unwrap_or(1).max(0) as u32;
                self.total_loss_of_down_count += downs;
            }
            PunishmentKind::LossOfDrive => {
                let drives = magnitude.unwrap_or(1).max(0) as u32;
                self.total_loss_of_drive_count += drives;
            }
            PunishmentKind::TimePenalty => {
                let minutes = magnitude.unwrap_or(2).max(0) as f64;
                self.total_time_penalty_seconds += minutes * 60.0;
            }
            PunishmentKind::Expulsion => {
                self.total_expulsions += 1;
            }
            PunishmentKind::InvalidatePreviousPlay => {
                self.total_plays_invalidated += 1;
            }
            PunishmentKind::KickFoulAwarded => {}
        }
    }
}

impl IntoSnapshot for PlayerPunishmentStats {
    type Snapshot = PlayerPunishmentSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerPunishmentSnapshot {
            player_id: self.player_id,
            yardage_loss_count: self.count_for_kind(PunishmentKind::YardageLoss),
            loss_of_down_count: self.count_for_kind(PunishmentKind::LossOfDown),
            loss_of_drive_count: self.count_for_kind(PunishmentKind::LossOfDrive),
            time_penalty_count: self.count_for_kind(PunishmentKind::TimePenalty),
            expulsion_count: self.total_expulsions,
            invalidate_play_count: self.total_plays_invalidated,
            total_yardage_loss_mirim: self.total_yardage_loss_mirim,
            total_loss_of_down_count: self.total_loss_of_down_count,
            total_time_penalty_seconds: self.total_time_penalty_seconds,
            total_loss_of_drive_count: self.total_loss_of_drive_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerPunishmentAggregator {
    stats: HashMap<Uuid, PlayerPunishmentStats>,
}

impl PlayerPunishmentAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerPunishmentStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerPunishmentStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerPunishmentStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerPunishmentStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerPunishmentStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerPunishmentStats::new(player_id))
    }

    pub fn record_punishment(
        &mut self,
        player_id: Uuid,
        kind: PunishmentKind,
        magnitude: Option<i32>,
    ) {
        let stats = self.get_mut_or_create(player_id);
        stats.record_punishment(kind, magnitude);
    }
}

impl IntoSnapshot for PlayerPunishmentAggregator {
    type Snapshot = HashMap<Uuid, PlayerPunishmentSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerPunishmentAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::FoulRaised(e) = event {
            if let Some(kind) = e.punishment_kind() {
                self.record_punishment(
                    e.offending_player_id(),
                    kind,
                    e.punishment_magnitude(),
                );
            }
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}