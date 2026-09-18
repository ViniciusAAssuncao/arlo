use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::calculate_fit_for_position;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::MIN_CURRENT_ABILITY;
use arlo_domain::{AttributeKey, Position, PositionLine};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMatchPower {
    offensive_power: f64,
    defensive_power: f64,
    control_power: f64,
}

impl TeamMatchPower {
    pub fn new(offensive_power: f64, defensive_power: f64, control_power: f64) -> Self {
        Self {
            offensive_power,
            defensive_power,
            control_power,
        }
    }

    pub fn offensive_power(&self) -> f64 {
        self.offensive_power
    }

    pub fn defensive_power(&self) -> f64 {
        self.defensive_power
    }

    pub fn control_power(&self) -> f64 {
        self.control_power
    }
}

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

pub fn calculate_team_match_power(state: &MatchState, team_id: Uuid) -> TeamMatchPower {
    let is_home = team_id == state.home_team_id();
    let lineup = if is_home {
        state.home_lineup()
    } else {
        state.away_lineup()
    };
    let offense_pos_index = state.offensive_position_index_for_team(team_id);
    let defense_pos_index = state.defensive_position_index_for_team(team_id);
    let manager_table = state.manager_attribute_table_for(team_id);
    let fatigue_lookup = state.fatigue_lookup();
    let availability = state.availability();
    let tables = state.teams.player_attribute_tables();

    let mut total_off_ca = 0.0;
    let mut total_def_ca = 0.0;
    let mut total_ctrl_ca = 0.0;

    for assignment in lineup.assignments() {
        let player = assignment.player();
        let pid = player.id();
        if !availability.availability_for(&pid).is_active() {
            continue;
        }

        let table = tables.get(&pid).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let ca = calculate_player_ca(player, table).unwrap_or(MIN_CURRENT_ABILITY) as f64;

        let off_pos = offense_pos_index
            .get(&pid)
            .copied()
            .unwrap_or(Position::CenterOffense);
        let def_pos = defense_pos_index
            .get(&pid)
            .copied()
            .unwrap_or(Position::Centerback);

        let off_fit = calculate_fit_for_position(player, off_pos).efficiency_multiplier();
        let def_fit = calculate_fit_for_position(player, def_pos).efficiency_multiplier();

        let fatigue = fatigue_lookup.get(&pid);
        let fatigue_factor =
            (0.70 + 0.20 * fatigue.energy() + 0.10 * fatigue.w_prime_balance()).clamp(0.40, 1.00);

        let (off_weight, ctrl_weight) = match off_pos {
            Position::Artrine => (1.1, 1.4),
            Position::Passer => (1.0, 1.2),
            _ => match off_pos.line() {
                PositionLine::OffensiveLine => (1.0, 0.6),
                PositionLine::BackLine => (0.8, 1.0),
                PositionLine::DefenseLine => (0.2, 0.3),
                PositionLine::Goalguard => (0.05, 0.1),
            },
        };

        let def_weight = match def_pos {
            Position::Goalguard => 1.3,
            _ => match def_pos.line() {
                PositionLine::DefenseLine => 1.0,
                PositionLine::BackLine => 0.7,
                PositionLine::OffensiveLine => 0.2,
                PositionLine::Goalguard => 1.1,
            },
        };

        total_off_ca += ca * off_fit * fatigue_factor * off_weight;
        total_def_ca += ca * def_fit * fatigue_factor * def_weight;
        total_ctrl_ca += ca * off_fit * fatigue_factor * ctrl_weight;
    }

    let off_tactical_bonus = (manager_table.get(AttributeKey::OffensePlanning) * 0.06)
        + (manager_table.get(AttributeKey::TacticalKnowledge) * 0.04);
    let def_tactical_bonus = (manager_table.get(AttributeKey::DefenseOrganization) * 0.06)
        + (manager_table.get(AttributeKey::TacticalKnowledge) * 0.04);
    let ctrl_tactical_bonus = (manager_table.get(AttributeKey::ArtroStrategy) * 0.05)
        + (manager_table.get(AttributeKey::TacticalKnowledge) * 0.05);

    let home_factor = if is_home { 0.5 } else { 0.0 };

    let offensive_power = (total_off_ca / 70.0) + off_tactical_bonus + home_factor;
    let defensive_power = (total_def_ca / 70.0) + def_tactical_bonus + home_factor;
    let control_power = (total_ctrl_ca / 70.0) + ctrl_tactical_bonus + home_factor;

    TeamMatchPower::new(offensive_power, defensive_power, control_power)
}
