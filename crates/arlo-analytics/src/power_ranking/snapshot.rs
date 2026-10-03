use crate::power_ranking::rating::PowerRating;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const POWER_RANKING_MODEL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PowerRankingEntry {
    rank: u32,
    team_id: Uuid,
    rating: PowerRating,
    games_rated: u32,
}

impl PowerRankingEntry {
    pub(crate) fn new(rank: u32, team_id: Uuid, rating: PowerRating, games_rated: u32) -> Self {
        Self {
            rank,
            team_id,
            rating,
            games_rated,
        }
    }

    pub fn rank(self) -> u32 {
        self.rank
    }

    pub fn team_id(self) -> Uuid {
        self.team_id
    }

    pub fn rating(self) -> PowerRating {
        self.rating
    }

    pub fn games_rated(self) -> u32 {
        self.games_rated
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerRankingSnapshot {
    model_version: u32,
    entries: Vec<PowerRankingEntry>,
}

impl PowerRankingSnapshot {
    pub(crate) fn new(entries: Vec<PowerRankingEntry>) -> Self {
        Self {
            model_version: POWER_RANKING_MODEL_VERSION,
            entries,
        }
    }

    pub fn model_version(&self) -> u32 {
        self.model_version
    }

    pub fn entries(&self) -> &[PowerRankingEntry] {
        &self.entries
    }

    pub fn entry(&self, team_id: Uuid) -> Option<&PowerRankingEntry> {
        self.entries.iter().find(|entry| entry.team_id == team_id)
    }
}
