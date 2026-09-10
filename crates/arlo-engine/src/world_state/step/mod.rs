pub mod open_play_loop;
pub mod play_resolution;
pub mod setup;
pub mod target_weighting;

pub use open_play_loop::run_open_play_loop;
pub use play_resolution::*;
pub use setup::{setup_call_to_action_context, CallToActionContext};
pub use target_weighting::resolve_decision_target_weights;

use crate::error::EngineResult;
use crate::manager_ai::orchestrator::ManagerAiEngine;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::rng::RngStream;
use crate::time::DurationLedger;
use crate::world_state::cta_pass::resolve_pass_phase;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::{apply_play_transition, EventPublisher};
use arlo_domain::ArtrineDecisionKind;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use std::collections::HashMap;
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

    let (last_play_call_id, last_play_failed) = state
        .last_play_outcome_summary()
        .map(|(id, failed)| (Some(id), failed))
        .unwrap_or((None, false));

    let offense_id = state.possession().offense();
    let seq = state.next_sequence();
    let mut ai_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::PlayCallSelection, seq);

    {
        let mut publisher = EventPublisher::new(state, sink);
        ManagerAiEngine::on_down_start(
            &mut publisher,
            offense_id,
            last_play_call_id,
            last_play_failed,
            &mut ai_rng,
        );
    }

    let context = setup_call_to_action_context(state);
    let active_play_call_id = context.active_play_call.as_ref().map(|pc| pc.id());
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

    let (chosen_decision, execution_outcome) = if !pass_phase.pass_completed {
        (
            ArtrineDecisionKind::SelfCarry,
            crate::artrine::ArtrineExecutionOutcome {
                mirins_advanced: 0.0,
                drives_recorded: 0,
                drive_row_indices: Vec::new(),
                turnover: None,
                recovering_player_id: None,
                scoring_decision: ScoringDecision::NoOpportunity,
                duration_ledger: DurationLedger::new(),
                end_position: pass_phase.scrimmage_point,
                duels: Vec::new(),
                receiver_id: None,
                distribution_flight: None,
                kinematic_trajectories: HashMap::new(),
            },
        )
    } else {
        open_play_loop::run_open_play_loop(
            state,
            &context,
            &pass_phase,
            &offense_players,
            &defense_players,
            sink,
        )?
    };

    let detailed_outcome = apply_play_transition(
        state,
        pass_phase,
        chosen_decision,
        execution_outcome,
        context.offense_team_id,
        context.defense_team_id,
        active_play_call_id,
        sink,
    );

    Ok(detailed_outcome)
}