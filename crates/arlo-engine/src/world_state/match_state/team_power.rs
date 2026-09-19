use crate::team_strength::TeamMatchPower;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchPowerCache {
    home_power: TeamMatchPower,
    away_power: TeamMatchPower,
    last_update_period: u32,
    last_update_seconds: f64,
    dirty: bool,
}

impl MatchPowerCache {
    pub fn new() -> Self {
        Self {
            home_power: TeamMatchPower::default(),
            away_power: TeamMatchPower::default(),
            last_update_period: 0,
            last_update_seconds: 0.0,
            dirty: true,
        }
    }

    pub fn home_power(&self) -> TeamMatchPower {
        self.home_power
    }

    pub fn away_power(&self) -> TeamMatchPower {
        self.away_power
    }

    pub fn power_for_team(&self, team_id: Uuid, home_team_id: Uuid) -> TeamMatchPower {
        if team_id == home_team_id {
            self.home_power
        } else {
            self.away_power
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn should_refresh(
        &self,
        current_period: u32,
        current_seconds: f64,
        refresh_interval_seconds: f64,
    ) -> bool {
        self.dirty
            || self.last_update_period != current_period
            || (current_seconds - self.last_update_seconds).abs() >= refresh_interval_seconds
    }

    pub fn update(
        &mut self,
        home: TeamMatchPower,
        away: TeamMatchPower,
        period: u32,
        seconds: f64,
    ) {
        self.home_power = home;
        self.away_power = away;
        self.last_update_period = period;
        self.last_update_seconds = seconds;
        self.dirty = false;
    }
}

impl Default for MatchPowerCache {
    fn default() -> Self {
        Self::new()
    }
}
