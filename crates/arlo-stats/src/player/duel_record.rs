use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerDuelSnapshot};
use arlo_events::{DuelKind, MatchEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DuelKindStats {
    pub total: u32,
    pub wins: u32,
    pub losses: u32,
    pub as_attacker_wins: u32,
    pub as_attacker_losses: u32,
    pub as_defender_wins: u32,
    pub as_defender_losses: u32,
}

impl DuelKindStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn win_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.wins as f64) / (self.total as f64)
        }
    }

    pub fn attacker_total(&self) -> u32 {
        self.as_attacker_wins + self.as_attacker_losses
    }

    pub fn defender_total(&self) -> u32 {
        self.as_defender_wins + self.as_defender_losses
    }

    pub fn attacker_win_rate(&self) -> f64 {
        let att_total = self.attacker_total();
        if att_total == 0 {
            0.0
        } else {
            (self.as_attacker_wins as f64) / (att_total as f64)
        }
    }

    pub fn defender_win_rate(&self) -> f64 {
        let def_total = self.defender_total();
        if def_total == 0 {
            0.0
        } else {
            (self.as_defender_wins as f64) / (def_total as f64)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDuelStats {
    pub player_id: Uuid,
    pub total_duels: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub attacker_duels: u32,
    pub attacker_wins: u32,
    pub attacker_losses: u32,
    pub defender_duels: u32,
    pub defender_wins: u32,
    pub defender_losses: u32,
    pub by_kind: HashMap<DuelKind, DuelKindStats>,
}

impl PlayerDuelStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_duels: 0,
            total_wins: 0,
            total_losses: 0,
            attacker_duels: 0,
            attacker_wins: 0,
            attacker_losses: 0,
            defender_duels: 0,
            defender_wins: 0,
            defender_losses: 0,
            by_kind: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_duels(&self) -> u32 {
        self.total_duels
    }

    pub fn total_wins(&self) -> u32 {
        self.total_wins
    }

    pub fn total_losses(&self) -> u32 {
        self.total_losses
    }

    pub fn win_rate(&self) -> f64 {
        if self.total_duels == 0 {
            0.0
        } else {
            (self.total_wins as f64) / (self.total_duels as f64)
        }
    }

    pub fn attacker_duels(&self) -> u32 {
        self.attacker_duels
    }

    pub fn attacker_wins(&self) -> u32 {
        self.attacker_wins
    }

    pub fn attacker_losses(&self) -> u32 {
        self.attacker_losses
    }

    pub fn attacker_win_rate(&self) -> f64 {
        if self.attacker_duels == 0 {
            0.0
        } else {
            (self.attacker_wins as f64) / (self.attacker_duels as f64)
        }
    }

    pub fn defender_duels(&self) -> u32 {
        self.defender_duels
    }

    pub fn defender_wins(&self) -> u32 {
        self.defender_wins
    }

    pub fn defender_losses(&self) -> u32 {
        self.defender_losses
    }

    pub fn defender_win_rate(&self) -> f64 {
        if self.defender_duels == 0 {
            0.0
        } else {
            (self.defender_wins as f64) / (self.defender_duels as f64)
        }
    }

    pub fn by_kind(&self) -> &HashMap<DuelKind, DuelKindStats> {
        &self.by_kind
    }

    pub fn stats_for_kind(&self, kind: DuelKind) -> Option<&DuelKindStats> {
        self.by_kind.get(&kind)
    }
}

impl IntoSnapshot for PlayerDuelStats {
    type Snapshot = PlayerDuelSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerDuelSnapshot {
            player_id: self.player_id,
            total_duels: self.total_duels,
            total_wins: self.total_wins,
            total_losses: self.total_losses,
            win_rate: self.win_rate(),
            attacker_duels: self.attacker_duels,
            attacker_wins: self.attacker_wins,
            attacker_losses: self.attacker_losses,
            attacker_win_rate: self.attacker_win_rate(),
            defender_duels: self.defender_duels,
            defender_wins: self.defender_wins,
            defender_losses: self.defender_losses,
            defender_win_rate: self.defender_win_rate(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerDuelAggregator {
    stats: HashMap<Uuid, PlayerDuelStats>,
}

impl PlayerDuelAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerDuelStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerDuelStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerDuelStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerDuelStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerDuelStats {
        self.stats.entry(player_id).or_insert_with(|| PlayerDuelStats::new(player_id))
    }

    pub fn record_attacker_duel(&mut self, player_id: Uuid, kind: DuelKind, won: bool) {
        let stats = self.get_mut_or_create(player_id);
        stats.total_duels += 1;
        stats.attacker_duels += 1;

        if won {
            stats.total_wins += 1;
            stats.attacker_wins += 1;
        } else {
            stats.total_losses += 1;
            stats.attacker_losses += 1;
        }

        let kind_stats = stats.by_kind.entry(kind).or_default();
        kind_stats.total += 1;
        if won {
            kind_stats.wins += 1;
            kind_stats.as_attacker_wins += 1;
        } else {
            kind_stats.losses += 1;
            kind_stats.as_attacker_losses += 1;
        }
    }

    pub fn record_defender_duel(&mut self, player_id: Uuid, kind: DuelKind, won: bool) {
        let stats = self.get_mut_or_create(player_id);
        stats.total_duels += 1;
        stats.defender_duels += 1;

        if won {
            stats.total_wins += 1;
            stats.defender_wins += 1;
        } else {
            stats.total_losses += 1;
            stats.defender_losses += 1;
        }

        let kind_stats = stats.by_kind.entry(kind).or_default();
        kind_stats.total += 1;
        if won {
            kind_stats.wins += 1;
            kind_stats.as_defender_wins += 1;
        } else {
            kind_stats.losses += 1;
            kind_stats.as_defender_losses += 1;
        }
    }
}

impl IntoSnapshot for PlayerDuelAggregator {
    type Snapshot = HashMap<Uuid, PlayerDuelSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerDuelAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::DuelResolved(e) = event {
            let kind = e.kind();
            let attacker_won = e.attacker_won();

            for &attacker_id in e.attacker_ids() {
                self.record_attacker_duel(attacker_id, kind, attacker_won);
            }

            for &defender_id in e.defender_ids() {
                self.record_defender_duel(defender_id, kind, !attacker_won);
            }
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}