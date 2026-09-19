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
use rand::Rng;

pub fn resolve_down<'a, R: Rng + ?Sized>(
    state: &mut MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'a>,
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    rng: &mut R,
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let seq = state.next_sequence();
    let clock_inst = state.clock.to_instant();
    let current_time_seconds = state.clock.seconds_in_period();

    let ctx = DownResolutionContext::build(
        state,
        call_context,
        pass_phase,
        pass_phase.artrine,
        offense_players,
        defense_players,
    );

    let is_home_offense = ctx.is_home_offense;
    let pitch_length_mirim = ctx.pitch_length_mirim;
    let carrier_id = ctx.carrier.id();
    let zone = ctx.zone;

    let decision = resolve_decision(&ctx, &state.attribute_keys, seq, clock_inst, rng, sink);

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

    state.possession.live_sequence_mut().record_touch(
        carrier_id,
        TouchActionType::from(decision),
        zone,
        current_time_seconds,
    );

    if let Some(receiver) = contest.receiver {
        if receiver.id() != carrier_id && contest.attacker_won {
            let reception_time = current_time_seconds + progression.live_duration.value() * 0.5;
            state.possession.live_sequence_mut().record_touch(
                receiver.id(),
                TouchActionType::Reception,
                progression.new_zone,
                reception_time,
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

    let end_x_mirim = if is_home_offense {
        progression.new_normalized_proximity * pitch_length_mirim
    } else {
        (1.0 - progression.new_normalized_proximity) * pitch_length_mirim
    };
    let end_y_mirim = state.pitch.width_mirim() * 0.5;

    let distribution_flight = contest.distribution_flight.map(|f| DistributionFlightInfo {
        distance_mirim: progression.mirins_advanced,
        ..f
    });

    let receiver_id = contest
        .receiver
        .map(|p| p.id())
        .or(Some(carrier_id));

    let outcome = ArtrineExecutionOutcome {
        mirins_advanced: progression.mirins_advanced,
        drives_recorded: progression.drives_recorded,
        turnover: contest.turnover_team,
        recovering_player_id: contest.recovering_player_id,
        scoring_decision: scoring,
        duration_ledger,
        end_x_mirim,
        end_y_mirim,
        duels,
        fouls: collateral.fouls,
        injuries: collateral.injuries,
        receiver_id,
        distribution_flight,
    };

    Ok((decision, outcome))
}