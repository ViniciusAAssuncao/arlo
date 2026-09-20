use crate::ai::gravity::{
    calculate_team_max_finishing_gravity_with_fatigue_from_tables, OffensiveGravity,
};
use crate::world_state::match_state::state::MatchState;
use arlo_domain::Player;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchGravityCache {
    home_gravity: OffensiveGravity,
    away_gravity: OffensiveGravity,
    last_update_period: u32,
    last_update_seconds: f64,
    dirty: bool,
}

impl MatchGravityCache {
    pub fn new() -> Self {
        Self {
            home_gravity: OffensiveGravity::default(),
            away_gravity: OffensiveGravity::default(),
            last_update_period: 0,
            last_update_seconds: 0.0,
            dirty: true,
        }
    }

    pub fn home_gravity(&self) -> OffensiveGravity {
        self.home_gravity
    }

    pub fn away_gravity(&self) -> OffensiveGravity {
        self.away_gravity
    }

    pub fn gravity_for_team(&self, team_id: Uuid, home_team_id: Uuid) -> OffensiveGravity {
        if team_id == home_team_id {
            self.home_gravity
        } else {
            self.away_gravity
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
        home: OffensiveGravity,
        away: OffensiveGravity,
        period: u32,
        seconds: f64,
    ) {
        self.home_gravity = home;
        self.away_gravity = away;
        self.last_update_period = period;
        self.last_update_seconds = seconds;
        self.dirty = false;
    }
}

impl Default for MatchGravityCache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_team_offensive_gravity(state: &MatchState, team_id: Uuid) -> OffensiveGravity {
    let is_home = team_id == state.home_team_id();
    let lineup = if is_home {
        state.home_lineup()
    } else {
        state.away_lineup()
    };
    let players: Vec<&Player> = lineup.assignments().iter().map(|a| a.player()).collect();
    let pos_index = state.offensive_position_index_for_team(team_id);
    let tables = state.teams.player_attribute_tables();
    let fatigue_lookup = state.fatigue_lookup();
    calculate_team_max_finishing_gravity_with_fatigue_from_tables(
        &players,
        tables,
        pos_index,
        state.pitch(),
        is_home,
        &|id| fatigue_lookup.get(id),
    )
}
