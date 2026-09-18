use crate::artrine::ArtrineExecutionOutcome;
use crate::error::EngineResult;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::carry_resolver::resolve_carry;
use crate::world_state::step::open_play_loop::cross_resolver::resolve_cross;
use crate::world_state::step::open_play_loop::decision_selection::select_carrier_decision;
use crate::world_state::step::open_play_loop::distribution_resolver::resolve_distribution;
use crate::world_state::step::open_play_loop::finish_resolver::resolve_finish;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;

pub fn run_open_play_loop(
    state: &mut MatchState,
    context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    offense_players: &[&Player],
    defense_players: &[&Player],
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let current_carrier = pass_phase.artrine;
    let is_true_artrine = current_carrier.id() == pass_phase.artrine.id();

    let iter_ctx = OpenPlayIterationContext::build(
        state,
        context,
        pass_phase,
        current_carrier,
        offense_players,
        defense_players,
    );

    let seq = state.event_sequence();
    let mut rng = state
        .rng_provider()
        .iteration_rng(RngStream::DuelResolution, seq, 1);

    let chosen_decision = select_carrier_decision(
        state,
        context,
        &iter_ctx,
        pass_phase,
        current_carrier,
        &mut rng,
        sink,
    );

    let outcome = match chosen_decision {
        ArtrineDecisionKind::SelfCarry => resolve_carry(
            state,
            context,
            &iter_ctx,
            pass_phase,
            current_carrier,
            defense_players,
            is_true_artrine,
            &mut rng,
        ),
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => resolve_distribution(
            state,
            context,
            &iter_ctx,
            pass_phase,
            current_carrier,
            defense_players,
            chosen_decision,
            &mut rng,
        ),
        ArtrineDecisionKind::Cross => resolve_cross(
            state,
            context,
            &iter_ctx,
            pass_phase,
            current_carrier,
            defense_players,
            &mut rng,
        ),
        ArtrineDecisionKind::SelfFinish => resolve_finish(
            state,
            context,
            &iter_ctx,
            pass_phase,
            current_carrier,
            defense_players,
            &mut rng,
        ),
    };

    Ok((chosen_decision, outcome))
}
