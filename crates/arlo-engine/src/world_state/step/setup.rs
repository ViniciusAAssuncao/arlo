use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::MatchState;
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use arlo_tactics::PlayerInstructions;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CallToActionContext {
    pub is_home_offense: bool,
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub offense_lineup: Lineup,
    pub defense_lineup: Lineup,
    pub offense_pos_index: HashMap<Uuid, DomainPosition>,
    pub defense_pos_index: HashMap<Uuid, DomainPosition>,
    pub offense_role_index: HashMap<Uuid, SlotRole>,
    pub offense_instructions_index: HashMap<Uuid, PlayerInstructions>,
    pub defense_instructions_index: HashMap<Uuid, PlayerInstructions>,
}

impl CallToActionContext {
    pub fn offense_players(&self) -> Vec<&Player> {
        self.offense_lineup.players()
    }

    pub fn defense_players(&self) -> Vec<&Player> {
        self.defense_lineup.players()
    }
}

pub fn setup_call_to_action_context(state: &MatchState) -> CallToActionContext {
    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
    let (offense_team_id, defense_team_id) = if is_home_offense {
        (state.home_team_id(), state.away_team_id())
    } else {
        (state.away_team_id(), state.home_team_id())
    };

    let (offense_lineup, defense_lineup) = if is_home_offense {
        (state.home_lineup().clone(), state.away_lineup().clone())
    } else {
        (state.away_lineup().clone(), state.home_lineup().clone())
    };

    let offense_pos_index = state
        .offensive_position_index_for_team(offense_team_id)
        .clone();
    let defense_pos_index = state
        .defensive_position_index_for_team(defense_team_id)
        .clone();
    let offense_role_index = state
        .role_index_for_team(offense_team_id)
        .clone();
    let offense_instructions_index = state
        .instructions_index_for_team(offense_team_id)
        .clone();
    let defense_instructions_index = state
        .instructions_index_for_team(defense_team_id)
        .clone();

    CallToActionContext {
        is_home_offense,
        offense_team_id,
        defense_team_id,
        offense_lineup,
        defense_lineup,
        offense_pos_index,
        defense_pos_index,
        offense_role_index,
        offense_instructions_index,
        defense_instructions_index,
    }
}