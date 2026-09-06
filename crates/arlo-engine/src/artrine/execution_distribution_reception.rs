use crate::artrine::execution_outcome::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::artrine::reception::resolve_reception;
use crate::artrine::run_after_catch::resolve_run_after_catch;
use crate::physical::FatigueState;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::{AttributedDuelOutcome, DuelContext};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_post_throw_reception<F, R>(
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    throw_advance: f64,
    flight_duration: Duration,
    dist_duration: Duration,
    dist_duel: AttributedDuelOutcome,
    attacking_positive_x: bool,
    defense_team_id: Uuid,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let reception_outcome = resolve_reception(
        decision_kind,
        artrine,
        offense_helpers,
        defenders,
        attribute_keys,
        pitch,
        spatial_map,
        offense_position_index,
        defense_position_index,
        attacking_positive_x,
        context,
        fatigue_for,
        rng,
    );

    let receiver_player = reception_outcome.receiver_player;
    let receiver_id = reception_outcome.receiver;
    let receiver_pos_domain = offense_position_index
        .get(&receiver_id)
        .copied()
        .unwrap_or_else(|| {
            receiver_player
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(DomainPosition::CenterOffense)
        });

    let receiver_pos_vec = spatial_map
        .get_position(&receiver_id)
        .unwrap_or(start_pos);

    if !reception_outcome.caught {
        let distribution_flight = Some(DistributionFlightInfo {
            receiver_id,
            passer_id: artrine.id(),
            decision_kind,
            is_aerial: reception_outcome.is_aerial,
            reception_point: receiver_pos_vec,
            distance_mirim: throw_advance,
            caught: false,
        });

        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::DistributionEngagement,
            dist_duration,
        );
        ledger.record_live(
            DurationComponentKind::DistributionFlight,
            flight_duration,
        );
        ledger.record_live(
            DurationComponentKind::ReceptionEngagement,
            reception_outcome.duration,
        );

        let (turnover, recovering_player_id, end_position) = if let Some(interceptor_id) = reception_outcome.intercepted_by_defender {
            (Some(defense_team_id), Some(interceptor_id), receiver_pos_vec)
        } else {
            (None, None, start_pos)
        };

        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger: ledger,
            end_position,
            duels: vec![dist_duel, reception_outcome.duel],
            receiver_id: Some(receiver_id),
            distribution_flight,
        };
    }

    let rac_helpers: Vec<&Player> = std::iter::once(artrine)
        .chain(
            offense_helpers
                .iter()
                .copied()
                .filter(|p| p.id() != receiver_id),
        )
        .collect();

    let rac_outcome = resolve_run_after_catch(
        &receiver_player,
        receiver_pos_domain,
        &rac_helpers,
        offense_position_index,
        defenders,
        defense_position_index,
        attribute_keys,
        pitch,
        spatial_map,
        defense_team_id,
        context,
        fatigue_for,
        rng,
    );

    let total_advance = throw_advance + rac_outcome.additional_mirins_advanced;
    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if attacking_positive_x {
        (start_x_mirim + total_advance).min(pitch.length_mirim())
    } else {
        (start_x_mirim - total_advance).max(0.0)
    };

    let mirins_advanced = (end_x_mirim - start_x_mirim).abs();
    let end_position = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        start_pos.raw().1,
        0.0,
    );

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
    ledger.record_live(
        DurationComponentKind::DistributionEngagement,
        dist_duration,
    );
    ledger.record_live(
        DurationComponentKind::DistributionFlight,
        flight_duration,
    );
    ledger.record_live(
        DurationComponentKind::ReceptionEngagement,
        reception_outcome.duration,
    );
    ledger.merge(rac_outcome.duration_ledger);

    ArtrineExecutionOutcome {
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
    }
}