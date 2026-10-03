use super::PlayerPerformanceAggregator;
use crate::performance::live::snapshot::LivePerformanceSnapshotRecord;
use crate::performance::rating::PlayerPerformanceSnapshot;
use crate::performance::PlayerMatchRating;
use arlo_events::MatchClockInstant;
use std::collections::HashMap;
use uuid::Uuid;

impl PlayerPerformanceAggregator {
    pub fn capture_snapshot(
        &mut self,
        sequence_number: u64,
        clock: MatchClockInstant,
    ) -> LivePerformanceSnapshotRecord {
        let snapshot = self.create_snapshot(sequence_number, clock);
        self.history.push(snapshot.clone());
        snapshot
    }

    pub fn create_snapshot(
        &self,
        sequence_number: u64,
        clock: MatchClockInstant,
    ) -> LivePerformanceSnapshotRecord {
        LivePerformanceSnapshotRecord::new(
            sequence_number,
            clock,
            self.player_snapshots_map(),
            self.all_team_average_ratings(),
        )
    }

    pub fn current_player_snapshot(&self, player_id: &Uuid) -> Option<PlayerPerformanceSnapshot> {
        self.players.get(player_id).map(|state| state.to_snapshot())
    }

    pub fn all_player_snapshots(&self) -> Vec<PlayerPerformanceSnapshot> {
        let mut snapshots: Vec<PlayerPerformanceSnapshot> =
            self.players.values().map(|state| state.to_snapshot()).collect();
        snapshots.sort_by_key(|snapshot| snapshot.player_id());
        snapshots
    }

    pub fn current_player_ratings(&self) -> Vec<PlayerMatchRating> {
        self.all_player_snapshots()
            .iter()
            .map(PlayerMatchRating::from_snapshot)
            .collect()
    }

    pub fn player_snapshots_map(&self) -> HashMap<Uuid, PlayerPerformanceSnapshot> {
        self.players
            .iter()
            .map(|(&player_id, state)| (player_id, state.to_snapshot()))
            .collect()
    }

    pub fn team_average_rating(&self, team_id: &Uuid) -> Option<f64> {
        let mut players: Vec<_> = self
            .players
            .values()
            .filter(|player| player.team_id() == *team_id)
            .collect();

        if players.is_empty() {
            return None;
        }

        players.sort_by_key(|player| player.player_id());
        let sum: f64 = players
            .iter()
            .map(|player| player.performance_rating().value())
            .sum();
        Some(sum / players.len() as f64)
    }

    pub fn all_team_average_ratings(&self) -> HashMap<Uuid, f64> {
        let mut players: Vec<_> = self.players.values().collect();
        players.sort_by_key(|player| player.player_id());

        let mut sums: HashMap<Uuid, (f64, usize)> = HashMap::new();
        for player in players {
            let entry = sums.entry(player.team_id()).or_insert((0.0, 0));
            entry.0 += player.performance_rating().value();
            entry.1 += 1;
        }

        sums.into_iter()
            .map(|(team_id, (sum, count))| (team_id, sum / count.max(1) as f64))
            .collect()
    }

    pub fn snapshot_history(&self) -> &[LivePerformanceSnapshotRecord] {
        &self.history
    }

    pub fn snapshot_at_sequence(
        &self,
        sequence_number: u64,
    ) -> Option<&LivePerformanceSnapshotRecord> {
        self.history
            .iter()
            .find(|snapshot| snapshot.sequence_number() == sequence_number)
    }

    pub fn latest_snapshot(&self) -> Option<&LivePerformanceSnapshotRecord> {
        self.history.last()
    }
}
