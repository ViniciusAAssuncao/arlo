pub mod decision_phase;
pub mod play_resolution;
pub mod setup;

pub use decision_phase::{run_decision_phase, DecisionPhaseResult};
pub use play_resolution::*;
pub use setup::{setup_call_to_action_context, CallToActionContext};

use crate::error::EngineResult;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::world_state::cta_pass::resolve_pass_phase;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::apply_play_transition;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

fn build_finished_match_outcome(state: &MatchState) -> DetailedPlayOutcome {
    let scrimmage = state.possession().scrimmage_point();
    DetailedPlayOutcome {
        offense_team_id: state.possession().offense(),
        defense_team_id: state.possession().defense(),
        passer_id: Uuid::nil(),
        artrine_id: Uuid::nil(),
        down_number: state.possession().down() as u32,
        scrimmage_x_mirim: scrimmage.raw().0 / MIRIM_TO_METERS,
        pass_completed: false,
        pass_is_aerial: false,
        reception_point: scrimmage,
        drives_recorded: 0,
        mirins_advanced: 0.0,
        duels: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        lost_by_player_id: None,
        out_of_bounds: false,
        arbitral_stoppage: true,
        last_valid_possession_point: scrimmage,
        possession_control_seconds: None,
        scoring_decision: ScoringDecision::NoOpportunity,
    }
}

pub fn step_call_to_action(
    state: &mut MatchState,
    sink: &mut impl EventSink,
) -> EngineResult<DetailedPlayOutcome> {
    if state.is_match_finished() {
        return Ok(build_finished_match_outcome(state));
    }

    let context = setup_call_to_action_context(state);
    let offense_players = context.offense_players();
    let defense_players = context.defense_players();

    let pass_phase = resolve_pass_phase(
        state,
        &offense_players,
        &context.offense_pos_index,
        &context.offense_role_index,
        &defense_players,
        context.is_home_offense,
        context.offense_team_id,
        context.defense_team_id,
        sink,
    )?;

    let decision_result = run_decision_phase(
        state,
        &context,
        &pass_phase,
        &offense_players,
        &defense_players,
        sink,
    )?;

    let detailed_outcome = apply_play_transition(
        state,
        pass_phase,
        decision_result.chosen_decision,
        decision_result.execution_outcome,
        context.offense_team_id,
        context.defense_team_id,
        sink,
    );

    Ok(detailed_outcome)
}