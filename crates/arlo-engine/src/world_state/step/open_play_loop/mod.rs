pub mod action_context;
pub mod carry_action;
pub mod decision_selection;
pub mod distribution_action;
pub mod finish_action;
pub mod loop_state;

pub use action_context::OpenPlayIterationContext;
pub use carry_action::execute_carry_action;
pub use decision_selection::select_carrier_decision;
pub use distribution_action::execute_distribution_action;
pub use finish_action::{execute_cross_action, execute_self_finish_action, find_defense_goalguard};
pub use loop_state::OpenPlayLoopState;

use crate::artrine::ArtrineExecutionOutcome;
use crate::error::EngineResult;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;

const MAX_LIVE_ACTION_ITERATIONS: usize = 16;

pub fn run_open_play_loop(
    state: &mut MatchState,
    context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    offense_players: &[&Player],
    defense_players: &[&Player],
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let mut loop_state = OpenPlayLoopState::new(pass_phase.artrine.id(), pass_phase.reception_point);

    while loop_state.ball_in_play
        && loop_state.loop_iteration < MAX_LIVE_ACTION_ITERATIONS
        && !state.is_match_finished()
    {
        loop_state.loop_iteration += 1;

        let current_carrier = match offense_players
            .iter()
            .copied()
            .find(|p| p.id() == loop_state.current_carrier_id)
        {
            Some(p) => p,
            None => break,
        };

        let is_true_artrine = loop_state.current_carrier_id == pass_phase.artrine.id();

        let iter_ctx = OpenPlayIterationContext::build(
            state,
            context,
            pass_phase,
            &loop_state,
            current_carrier,
            offense_players,
            defense_players,
        );

        let chosen_decision = select_carrier_decision(
            state,
            context,
            &iter_ctx,
            pass_phase,
            &loop_state,
            current_carrier,
            sink,
        );

        if loop_state.loop_iteration == 1 {
            loop_state.primary_decision_kind = chosen_decision;
        }

        match chosen_decision {
            ArtrineDecisionKind::SelfCarry => {
                execute_carry_action(
                    state,
                    context,
                    &iter_ctx,
                    &mut loop_state,
                    current_carrier,
                    defense_players,
                    is_true_artrine,
                );
            }
            ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => {
                execute_distribution_action(
                    state,
                    context,
                    &iter_ctx,
                    pass_phase,
                    &mut loop_state,
                    current_carrier,
                    defense_players,
                    chosen_decision,
                );
            }
            ArtrineDecisionKind::Cross => {
                execute_cross_action(
                    state,
                    context,
                    &iter_ctx,
                    pass_phase,
                    &mut loop_state,
                    current_carrier,
                    defense_players,
                );
            }
            ArtrineDecisionKind::SelfFinish => {
                execute_self_finish_action(
                    state,
                    context,
                    &iter_ctx,
                    pass_phase,
                    &mut loop_state,
                    current_carrier,
                    defense_players,
                );
            }
        }
    }

    Ok(loop_state.into_outcome())
}