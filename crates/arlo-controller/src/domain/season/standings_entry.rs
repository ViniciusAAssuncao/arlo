use crate::domain::season::standings_home_away_record::HomeAwayRecord;
use crate::domain::season::standings_spa_metrics::SpaMetrics;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StandingsEntry {
    team_id: Uuid,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    goal_points_for: u32,
    goal_points_against: u32,
    home_away: HomeAwayRecord,
    spa_metrics: SpaMetrics,
    qta: f64,
}

impl StandingsEntry {
    pub fn new(
        team_id: Uuid,
        played: u32,
        won: u32,
        drawn: u32,
        lost: u32,
        goal_points_for: u32,
        goal_points_against: u32,
        home_away: HomeAwayRecord,
        spa_metrics: SpaMetrics,
        qta: f64,
    ) -> Self {
        Self {
            team_id,
            played,
            won,
            drawn,
            lost,
            goal_points_for,
            goal_points_against,
            home_away,
            spa_metrics,
            qta,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn played(&self) -> u32 {
        self.played
    }

    pub fn won(&self) -> u32 {
        self.won
    }

    pub fn drawn(&self) -> u32 {
        self.drawn
    }

    pub fn lost(&self) -> u32 {
        self.lost
    }

    pub fn goal_points_for(&self) -> u32 {
        self.goal_points_for
    }

    pub fn goal_points_against(&self) -> u32 {
        self.goal_points_against
    }

    pub fn home_away(&self) -> HomeAwayRecord {
        self.home_away
    }

    pub fn spa_metrics(&self) -> SpaMetrics {
        self.spa_metrics
    }

    pub fn qta(&self) -> f64 {
        self.qta
    }

    pub fn with_spa_metrics(mut self, spa_metrics: SpaMetrics) -> Self {
        self.spa_metrics = spa_metrics;
        self
    }

    pub fn with_qta(mut self, qta: f64) -> Self {
        self.qta = qta;
        self
    }
}
