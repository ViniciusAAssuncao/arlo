use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::open_play_finish::resolve_open_play_finish_attempt;
use crate::artrine::execution::outcome::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::artrine::reception_phase::reception::{resolve_reception, ReceptionOutcome};
use crate::artrine::reception_phase::run_after_catch::resolve_run_after_catch;
use crate::match_decision::scoring::ScoringDecision;
use crate::physical::FatigueState;
use crate::resolution::AttributedDuelOutcome;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

fn handle_failed_reception(
    defense_team_id: Uuid,
    artrine_id: Uuid,
    decision_kind: ArtrineDecisionKind,
    start_pos: VectorPosition,
    throw_advance: f64,
    flight_duration: Duration,
    dist_duration: Duration,
    dist_duel: AttributedDuelOutcome,
    reception_outcome: ReceptionOutcome,
    receiver_pos_vec: VectorPosition,
) -> ArtrineExecutionOutcome {
    let distribution_flight = Some(DistributionFlightInfo {
        receiver_id: reception_outcome.receiver,
        passer_id: artrine_id,
        decision_kind,
        is_aerial: reception_outcome.is_aerial,
        reception_point: receiver_pos_vec,
        distance_mirim: throw_advance,
        caught: false,
    });

    let mut ledger = DurationLedger::new();
    ledger.record_live(DurationComponentKind::DistributionEngagement, dist_duration);
    ledger.record_live(DurationComponentKind::DistributionFlight, flight_duration);
    ledger.record_live(
        DurationComponentKind::ReceptionEngagement,
        reception_outcome.duration,
    );

    let (turnover, recovering_player_id, end_position) =
        if let Some(interceptor_id) = reception_outcome.intercepted_by_defender {
            (
                Some(defense_team_id),
                Some(interceptor_id),
                receiver_pos_vec,
            )
        } else {
            (None, None, start_pos)
        };

    ArtrineExecutionOutcome {
        mirins_advanced: 0.0,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover,
        recovering_player_id,
        scoring_decision: ScoringDecision::NoOpportunity,
        duration_ledger: ledger,
        end_position,
        duels: vec![dist_duel, reception_outcome.duel],
        receiver_id: Some(reception_outcome.receiver),
        distribution_flight,
        kinematic_trajectories: HashMap::new(),
    }
}

pub fn execute_post_throw_reception<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    throw_advance: f64,
    flight_duration: Duration,
    dist_duration: Duration,
    dist_duel: AttributedDuelOutcome,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let reception_outcome = resolve_reception(
        decision_kind,
        artrine,
        ctx.offense_helpers,
        ctx.defenders,
        ctx.attribute_keys,
        ctx.pitch,
        spatial_map,
        ctx.offense_position_index,
        ctx.defense_position_index,
        ctx.offense_instructions_index,
        ctx.defense_instructions_index,
        ctx.offense_instructions,
        ctx.attacking_positive_x,
        ctx.duel_context,
        ctx.fatigue_for,
        ctx.defense_pressing_multiplier,
        rng,
    );

    let receiver_player = reception_outcome.receiver_player.clone();
    let receiver_id = reception_outcome.receiver;
    let receiver_pos_domain = ctx
        .offense_position_index
        .get(&receiver_id)
        .copied()
        .unwrap_or_else(|| {
            receiver_player
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(DomainPosition::CenterOffense)
        });

    let receiver_pos_vec = spatial_map.get_position(&receiver_id).unwrap_or(start_pos);

    if !reception_outcome.caught {
        return handle_failed_reception(
            ctx.defense_team_id,
            artrine.id(),
            decision_kind,
            start_pos,
            throw_advance,
            flight_duration,
            dist_duration,
            dist_duel,
            reception_outcome,
            receiver_pos_vec,
        );
    }

    let rac_helpers: Vec<&Player> = std::iter::once(artrine)
        .chain(
            ctx.offense_helpers
                .iter()
                .copied()
                .filter(|p| p.id() != receiver_id),
        )
        .collect();

    let rac_outcome = resolve_run_after_catch(
        &receiver_player,
        receiver_pos_domain,
        &rac_helpers,
        ctx.offense_position_index,
        ctx.offense_role_index,
        ctx.defenders,
        ctx.defense_position_index,
        ctx.defense_instructions_index,
        ctx.attribute_keys,
        ctx.pitch,
        spatial_map,
        ctx.defense_team_id,
        ctx.duel_context,
        ctx.fatigue_for,
        ctx.defense_pressing_multiplier,
        rng,
    );

    let total_advance = throw_advance + rac_outcome.additional_mirins_advanced;
    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if ctx.attacking_positive_x {
        (start_x_mirim + total_advance).min(ctx.pitch.length_mirim())
    } else {
        (start_x_mirim - total_advance).max(0.0)
    };

    let mirins_advanced = (end_x_mirim - start_x_mirim).abs();
    let end_position =
        VectorPosition::from_components(end_x_mirim * MIRIM_TO_METERS, start_pos.raw().1, 0.0);

    let distribution_flight = Some(DistributionFlightInfo {
        receiver_id,
        passer_id: artrine.id(),
        decision_kind,
        is_aerial: reception_outcome.is_aerial,
        reception_point: receiver_pos_vec,
        distance_mirim: throw_advance,
        caught: true,
    });

    let mut duels = vec![dist_duel, reception_outcome.duel];
    duels.extend(rac_outcome.duels);

    let mut ledger = DurationLedger::new();
    ledger.record_live(DurationComponentKind::DistributionEngagement, dist_duration);
    ledger.record_live(DurationComponentKind::DistributionFlight, flight_duration);
    ledger.record_live(
        DurationComponentKind::ReceptionEngagement,
        reception_outcome.duration,
    );
    ledger.merge(rac_outcome.duration_ledger);

    if rac_outcome.turnover.is_some() {
        return ArtrineExecutionOutcome {
            mirins_advanced,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover: rac_outcome.turnover,
            recovering_player_id: rac_outcome.recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger: ledger,
            end_position,
            duels,
            receiver_id: Some(receiver_id),
            distribution_flight,
            kinematic_trajectories: HashMap::new(),
        };
    }

    let should_attempt_open_play_finish = matches!(
        decision_kind,
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch
    );

    let (turnover, recovering_player_id, scoring_decision) = if should_attempt_open_play_finish {
        let total_territory = ctx.accumulated_advance_mirim + mirins_advanced;
        resolve_open_play_finish_attempt(
            ctx,
            artrine.id(),
            &receiver_player,
            end_position,
            total_territory,
            spatial_map,
            &mut ledger,
            &mut duels,
            rng,
        )
    } else {
        (None, None, ScoringDecision::NoOpportunity)
    };

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover,
        recovering_player_id,
        scoring_decision,
        duration_ledger: ledger,
        end_position,
        duels,
        receiver_id: Some(receiver_id),
        distribution_flight,
        kinematic_trajectories: HashMap::new(),
    }
}
