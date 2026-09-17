pub mod action_context;
pub mod carry_action;
pub mod carry_collision;
pub mod decision_selection;
pub mod distribution_action;
pub mod distribution_reception;
pub mod distribution_scoring;
pub mod finish_action;
pub mod loop_state;

pub use action_context::OpenPlayIterationContext;
pub use carry_action::execute_carry_action;
pub use carry_collision::{resolve_carry_collision, CarryCollisionResult};
pub use decision_selection::select_carrier_decision;
pub use distribution_action::execute_distribution_action;
pub use distribution_reception::{resolve_distribution_reception, DistributionReceptionResult};
pub use distribution_scoring::check_distribution_scoring_opportunity;
pub use finish_action::{execute_cross_action, execute_self_finish_action};
pub use loop_state::OpenPlayLoopState;

use crate::artrine::ArtrineExecutionOutcome;
use crate::error::EngineResult;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::formation_snapshot::build_role_zone_map;
use crate::rng::RngStream;
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
    let mut loop_state =
        OpenPlayLoopState::new(pass_phase.artrine.id(), pass_phase.reception_point);

    let pitch_length_mirim = state.pitch().length_mirim();
    let norm_prox = (loop_state.current_carrier_pos.raw().0
        / state.pitch().length().value())
    .clamp(0.0, 1.0);

    let mut pitch_state = PitchState::new(
        state.possession().down(),
        state
            .possession()
            .series_state()
            .remaining_mirins_to_target(),
        PitchState::determine_zone_from_proximity(norm_prox),
        arlo_domain::ArtroPlacement::Central,
        norm_prox,
        state.drives_in_current_series(),
        state.possession().is_bonus_phase(),
    );

    let offense_lineup = if context.is_home_offense {
        state.home_lineup_arc()
    } else {
        state.away_lineup_arc()
    };
    let defense_lineup = if context.is_home_offense {
        state.away_lineup_arc()
    } else {
        state.home_lineup_arc()
    };

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let defense_instructions = *state.instructions_for_team(context.defense_team_id);

    let _role_zone_map = build_role_zone_map(
        &offense_lineup,
        context.offense_team_id,
        &offense_instructions,
        &defense_lineup,
        context.defense_team_id,
        &defense_instructions,
        &pitch_state,
    );

    while loop_state.ball_in_play
        && loop_state.loop_iteration < MAX_LIVE_ACTION_ITERATIONS
        && !state.is_match_finished()
    {
        loop_state.loop_iteration += 1;

        let seq = state.event_sequence();
        let mut iteration_rng = state.rng_provider().iteration_rng(
            RngStream::DuelResolution,
            seq,
            loop_state.loop_iteration,
        );

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
            &mut iteration_rng,
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
                    &mut iteration_rng,
                );
                pitch_state = pitch_state.with_advance(
                    loop_state.accumulated_mirins_advanced,
                    pitch_length_mirim,
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
                    &mut iteration_rng,
                );
                pitch_state = pitch_state.with_advance(
                    loop_state.accumulated_mirins_advanced,
                    pitch_length_mirim,
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
                    &mut iteration_rng,
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
                    &mut iteration_rng,
                );
            }
        }
    }

    Ok(loop_state.into_outcome())
}
