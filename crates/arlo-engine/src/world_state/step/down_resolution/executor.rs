use crate::artrine::ArtrineExecutionOutcome;
use crate::error::EngineResult;
use crate::possession_flow::chain_orchestrator::orchestrate_chain;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::context::DownStaticContext;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;
use rand::Rng;

pub fn resolve_down<'a, R: Rng + ?Sized>(
    state: &mut MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'a>,
    carrier: &'a Player,
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    rng: &mut R,
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let static_ctx = DownStaticContext::build(
        state,
        call_context,
        offense_players.to_vec(),
        defense_players.to_vec(),
    );

    let chain_result = orchestrate_chain(
        state,
        &static_ctx,
        call_context,
        pass_phase,
        carrier,
        rng,
        sink,
    );

    let outcome = ArtrineExecutionOutcome {
        mirins_advanced: chain_result.mirins_advanced,
        drives_recorded: chain_result.drives_recorded,
        turnover: chain_result.turnover,
        recovering_player_id: chain_result.recovering_player_id,
        scoring_decision: chain_result.scoring_decision,
        duration_ledger: chain_result.duration_ledger,
        end_x_mirim: chain_result.end_x_mirim,
        end_y_mirim: chain_result.end_y_mirim,
        duels: chain_result.duels,
        fouls: chain_result.fouls,
        injuries: chain_result.injuries,
        receiver_id: chain_result.receiver_id,
        distribution_flight: chain_result.distribution_flight,
    };

    Ok((chain_result.first_decision.unwrap_or(ArtrineDecisionKind::SelfCarry), outcome))
}
