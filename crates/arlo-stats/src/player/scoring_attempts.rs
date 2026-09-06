use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerScoringAttemptSnapshot};
use arlo_events::{MatchEvent, ScoringPost};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerScoringAttemptStats {
    pub player_id: Uuid,
    pub attempts: u32,
    pub converted: u32,
    pub missed: u32,
    pub goal_points_scored: u32,
    pub field_points_scored: u32,
    pub field_goals_scored: u32,
    pub total_points_scored: u32,
    pub by_post: HashMap<ScoringPost, (u32, u32)>,
}

impl PlayerScoringAttemptStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            attempts: 0,
            converted: 0,
            missed: 0,
            goal_points_scored: 0,
            field_points_scored: 0,
            field_goals_scored: 0,
            total_points_scored: 0,
            by_post: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn converted(&self) -> u32 {
        self.converted
    }

    pub fn missed(&self) -> u32 {
        self.missed
    }

    pub fn goal_points_scored(&self) -> u32 {
        self.goal_points_scored
    }

    pub fn field_points_scored(&self) -> u32 {
        self.field_points_scored
    }

    pub fn field_goals_scored(&self) -> u32 {
        self.field_goals_scored
    }

    pub fn total_points_scored(&self) -> u32 {
        self.total_points_scored
    }

    pub fn by_post(&self) -> &HashMap<ScoringPost, (u32, u32)> {
        &self.by_post
    }

    pub fn conversion_rate(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            (self.converted as f64) / (self.attempts as f64)
        }
    }

    pub fn miss_rate(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            (self.missed as f64) / (self.attempts as f64)
        }
    }

    pub fn attempts_for_post(&self, post: ScoringPost) -> u32 {
        self.by_post.get(&post).map(|(att, _)| *att).unwrap_or(0)
    }

    pub fn converted_for_post(&self, post: ScoringPost) -> u32 {
        self.by_post.get(&post).map(|(_, conv)| *conv).unwrap_or(0)
    }

    pub fn missed_for_post(&self, post: ScoringPost) -> u32 {
        self.attempts_for_post(post).saturating_sub(self.converted_for_post(post))
    }

    pub fn conversion_rate_for_post(&self, post: ScoringPost) -> f64 {
        let att = self.attempts_for_post(post);
        if att == 0 {
            0.0
        } else {
            (self.converted_for_post(post) as f64) / (att as f64)
        }
    }
}

impl IntoSnapshot for PlayerScoringAttemptStats {
    type Snapshot = PlayerScoringAttemptSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerScoringAttemptSnapshot {
            player_id: self.player_id,
            attempts: self.attempts,
            converted: self.converted,
            missed: self.missed,
            goal_points_scored: self.goal_points_scored,
            field_points_scored: self.field_points_scored,
            field_goals_scored: self.field_goals_scored,
            total_points_scored: self.total_points_scored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerScoringAttemptsAggregator {
    stats: HashMap<Uuid, PlayerScoringAttemptStats>,
}

pub type PlayerScoringAttemptAggregator = PlayerScoringAttemptsAggregator;

impl PlayerScoringAttemptsAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerScoringAttemptStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerScoringAttemptStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerScoringAttemptStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerScoringAttemptStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerScoringAttemptStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerScoringAttemptStats::new(player_id))
    }

    pub fn record_conversion(&mut self, player_id: Uuid, post: ScoringPost) {
        let stats = self.get_mut_or_create(player_id);
        stats.attempts += 1;
        stats.converted += 1;
        let entry = stats.by_post.entry(post).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += 1;
    }

    pub fn record_miss(&mut self, player_id: Uuid, post: ScoringPost) {
        let stats = self.get_mut_or_create(player_id);
        stats.attempts += 1;
        stats.missed += 1;
        let entry = stats.by_post.entry(post).or_insert((0, 0));
        entry.0 += 1;
    }
}

impl IntoSnapshot for PlayerScoringAttemptsAggregator {
    type Snapshot = HashMap<Uuid, PlayerScoringAttemptSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerScoringAttemptsAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::GoalPoint(e) => {
                let stats = self.get_mut_or_create(e.scorer_id());
                stats.attempts += 1;
                stats.converted += 1;
                stats.goal_points_scored += 1;
                stats.total_points_scored += e.points();
                let entry = stats.by_post.entry(e.post()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += 1;
            }
            MatchEvent::FieldPoint(e) => {
                let stats = self.get_mut_or_create(e.scorer_id());
                stats.attempts += 1;
                stats.converted += 1;
                stats.field_points_scored += 1;
                stats.total_points_scored += e.points();
                let entry = stats.by_post.entry(e.post()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += 1;
            }
            MatchEvent::FieldGoal(e) => {
                let stats = self.get_mut_or_create(e.scorer_id());
                stats.attempts += 1;
                stats.converted += 1;
                stats.field_goals_scored += 1;
                stats.total_points_scored += e.points();
                let entry = stats.by_post.entry(e.post()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += 1;
            }
            MatchEvent::ScoringAttemptMissed(e) => {
                self.record_miss(e.scorer_id(), e.attempted_post());
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}