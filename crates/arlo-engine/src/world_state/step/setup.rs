use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::play_resolution::{
    resolve_decision_emphasis_for_play, resolve_role_index_for_play, resolve_route_index_for_play,
};
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use arlo_tactics::{DecisionEmphasis, PlayCall, PlayerInstructions, RouteAssignment};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct CallToActionContext {
    pub is_home_offense: bool,
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub offense_lineup: Arc<Lineup>,
    pub defense_lineup: Arc<Lineup>,
    pub offense_pos_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub defense_pos_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub offense_role_index: Arc<HashMap<Uuid, SlotRole>>,
    pub offense_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    pub defense_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    pub active_play_call: Option<PlayCall>,
    pub offense_route_index: Arc<HashMap<Uuid, RouteAssignment>>,
    pub decision_emphasis: DecisionEmphasis,
}

impl CallToActionContext {
    pub fn offense_players(&self) -> Vec<&Player> {
        self.offense_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect()
    }

    pub fn defense_players(&self) -> Vec<&Player> {
        self.defense_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect()
    }
}

pub fn setup_call_to_action_context(state: &mut MatchState) -> CallToActionContext {
    let active_play_call = state.resolve_active_play_call_for_offense();

    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
    let (offense_team_id, defense_team_id) = if is_home_offense {
        (state.home_team_id(), state.away_team_id())
    } else {
        (state.away_team_id(), state.home_team_id())
    };

    let (offense_lineup, defense_lineup) = if is_home_offense {
        (state.home_lineup_arc(), state.away_lineup_arc())
    } else {
        (state.away_lineup_arc(), state.home_lineup_arc())
    };

    let offense_pos_index = state.offensive_position_index_for_team_arc(offense_team_id);
    let defense_pos_index = state.defensive_position_index_for_team_arc(defense_team_id);
    let base_offense_role_index = state.role_index_for_team_arc(offense_team_id);
    let offense_instructions_index = state.instructions_index_for_team_arc(offense_team_id);
    let defense_instructions_index = state.instructions_index_for_team_arc(defense_team_id);

    let offense_role_index = match &active_play_call {
        Some(play_call) => Arc::new(resolve_role_index_for_play(
            &base_offense_role_index,
            &offense_lineup,
            play_call.role_overrides(),
        )),
        None => base_offense_role_index,
    };

    let offense_route_index = match &active_play_call {
        Some(play_call) => Arc::new(resolve_route_index_for_play(
            &offense_lineup,
            play_call.routes(),
        )),
        None => Arc::new(HashMap::new()),
    };

    let offense_instructions = state.instructions_for_team(offense_team_id);
    let decision_emphasis =
        resolve_decision_emphasis_for_play(active_play_call.as_ref(), offense_instructions);

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
        active_play_call,
        offense_route_index,
        decision_emphasis,
    }
}