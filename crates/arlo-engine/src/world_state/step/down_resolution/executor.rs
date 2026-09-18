use crate::artrine::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::error::EngineResult;
use crate::possession::TouchActionType;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::collateral_stage::resolve_collateral_events;
use crate::world_state::step::down_resolution::contest_stage::resolve_contest;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use crate::world_state::step::down_resolution::decision_stage::resolve_decision;
use crate::world_state::step::down_resolution::progression_stage::resolve_progression;
use crate::world_state::step::down_resolution::scoring_stage::resolve_scoring;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;

pub fn resolve_down<R: Rng + ?Sized>(
    state: &mut MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    offense_players: &[&Player],
    defense_players: &[&Player],
    rng: &mut R,
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let ctx = DownResolutionContext::build(
        state,
        call_context,
        pass_phase,
        pass_phase.artrine,
        offense_players,
        defense_players,
    );

    let decision = resolve_decision(&ctx, state, rng, sink);

    let zone = ctx.zone;
    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        ctx.carrier.id(),
        TouchActionType::from(decision),
        zone,
        current_time,
    );

    let contest = resolve_contest(&ctx, decision, state, rng);
    let progression = resolve_progression(&ctx, decision, &contest, state, rng);
    let collateral = resolve_collateral_events(&ctx, &contest, state, rng);

    let mut duels = contest.all_duels();
    let scoring = resolve_scoring(
        &ctx,
        decision,
        &contest,
        &progression,
        state,
        call_context,
        pass_phase,
        &mut duels,
        rng,
    );

    if let Some(receiver) = contest.receiver {
        if receiver.id() != ctx.carrier.id() && contest.attacker_won {
            state.possession_mut().live_sequence_mut().record_touch(
                receiver.id(),
                TouchActionType::Reception,
                progression.new_zone,
                current_time + progression.live_duration.value() * 0.5,
            );
        }
    }

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        match decision {
            ArtrineDecisionKind::SelfCarry => DurationComponentKind::CarrierMovement,
            ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => {
                DurationComponentKind::DistributionEngagement
            }
            ArtrineDecisionKind::Cross => DurationComponentKind::CrossFlight,
            ArtrineDecisionKind::SelfFinish => DurationComponentKind::FinishingEngagement,
        },
        progression.live_duration,
    );

    let end_x_m = if ctx.is_home_offense {
        progression.new_normalized_proximity * ctx.pitch_length_mirim * MIRIM_TO_METERS
    } else {
        (1.0 - progression.new_normalized_proximity) * ctx.pitch_length_mirim * MIRIM_TO_METERS
    };
    let end_position = VectorPosition::from_components(end_x_m, state.pitch().width().value() * 0.5, 0.0);

    let distribution_flight = contest.distribution_flight.map(|f| DistributionFlightInfo {
        reception_point: end_position,
        distance_mirim: progression.mirins_advanced,
        ..f
    });

    let receiver_id = contest
        .receiver
        .map(|p| p.id())
        .or_else(|| Some(ctx.carrier.id()));

    let outcome = ArtrineExecutionOutcome {
        mirins_advanced: progression.mirins_advanced,
        drives_recorded: progression.drives_recorded,
        drive_row_indices: progression.drive_row_indices,
        turnover: contest.turnover_team,
        recovering_player_id: contest.recovering_player_id,
        scoring_decision: scoring,
        duration_ledger,
        end_position,
        duels,
        fouls: collateral.fouls,
        injuries: collateral.injuries,
        receiver_id,
        distribution_flight,
    };

    Ok((decision, outcome))
}