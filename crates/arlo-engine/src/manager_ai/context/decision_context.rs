use crate::manager_ai::cognition::SituationalAwareness;
use crate::manager_ai::context::manager_snapshot::ManagerSnapshot;
use crate::manager_ai::context::squad_fatigue_summary::SquadFatigueSummary;
use crate::world_state::context_analyzer::{analyze_match_state, GameStatePressure};
use crate::world_state::match_state::MatchState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagerDecisionContext {
    pub team_id: Uuid,
    pub is_home: bool,
    pub game_state_pressure: GameStatePressure,
    pub manager_snapshot: ManagerSnapshot,
    pub squad_fatigue_summary: SquadFatigueSummary,
    pub remaining_time_calls: u32,
    pub remaining_challenges: u32,
    pub situational_awareness: SituationalAwareness,
    pub is_bonus_phase: bool,
}

impl ManagerDecisionContext {
    pub fn new(
        team_id: Uuid,
        is_home: bool,
        game_state_pressure: GameStatePressure,
        manager_snapshot: ManagerSnapshot,
        squad_fatigue_summary: SquadFatigueSummary,
        remaining_time_calls: u32,
        remaining_challenges: u32,
    ) -> Self {
        Self {
            team_id,
            is_home,
            game_state_pressure,
            manager_snapshot,
            squad_fatigue_summary,
            remaining_time_calls,
            remaining_challenges,
            situational_awareness: SituationalAwareness::default(),
            is_bonus_phase: false,
        }
    }

    pub fn with_situational_awareness(
        team_id: Uuid,
        is_home: bool,
        game_state_pressure: GameStatePressure,
        manager_snapshot: ManagerSnapshot,
        squad_fatigue_summary: SquadFatigueSummary,
        remaining_time_calls: u32,
        remaining_challenges: u32,
        situational_awareness: SituationalAwareness,
    ) -> Self {
        Self {
            team_id,
            is_home,
            game_state_pressure,
            manager_snapshot,
            squad_fatigue_summary,
            remaining_time_calls,
            remaining_challenges,
            situational_awareness,
            is_bonus_phase: false,
        }
    }

    pub fn build(state: &MatchState, team_id: Uuid) -> Self {
        let is_home = team_id == state.home_team_id();
        let game_state_pressure = analyze_match_state(state);
        let manager = state.manager_for_team(team_id);
        let manager_table = state.manager_attribute_table_for(team_id);
        let manager_snapshot = ManagerSnapshot::from_table(manager, manager_table);
        let lineup = if is_home {
            state.home_lineup()
        } else {
            state.away_lineup()
        };
        let squad_fatigue_summary =
            SquadFatigueSummary::from_lineup_and_fatigue(lineup, &state.fatigue);
        let remaining_time_calls = if is_home {
            state.clock().home_time_calls()
        } else {
            state.clock().away_time_calls()
        };
        let remaining_challenges = if is_home {
            state.clock().home_challenges()
        } else {
            state.clock().away_challenges()
        };
        let situational_awareness = SituationalAwareness::build(state, team_id);
        let is_bonus_phase = state.possession().is_bonus_phase();

        Self {
            team_id,
            is_home,
            game_state_pressure,
            manager_snapshot,
            squad_fatigue_summary,
            remaining_time_calls,
            remaining_challenges,
            situational_awareness,
            is_bonus_phase,
        }
    }
}