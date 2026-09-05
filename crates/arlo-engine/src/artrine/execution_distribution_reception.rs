use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::artrine::reception::resolve_reception;
use crate::artrine::run_after_catch::resolve_run_after_catch;
use crate::fatigue::{compute_player_fatigue_multiplier, FatigueState};
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
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
    dist_duel: DuelOutcome,
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

    if !reception_outcome.caught {
        let receiver_pos_vec = spatial_map
            .get_position(&receiver_id)
            .unwrap_or(start_pos);

        let close_defenders: Vec<&Player> = defenders
            .iter()
            .copied()
            .filter(|cand| {
                get_drifted_defender_position(cand, spatial_map, attribute_keys, rng)
                    .map(|p| {
                        calculate_distance_mirim(receiver_pos_vec, p)
                            <= PROXIMITY_CONTEST_RADIUS_MIRIM
                    })
                    .unwrap_or(false)
            })
            .collect();

        let (sec_defenders, sec_lead) = if !close_defenders.is_empty() {
            (close_defenders.as_slice(), close_defenders[0])
        } else {
            (defenders, defenders[0])
        };

        let sec_result = resolve_ball_security(
            DuelKind::BallSecurityDistribution,
            &receiver_player,
            receiver_pos_domain,
            sec_defenders,
            defense_position_index,
            attribute_keys,
            defense_team_id,
            context,
            rng,
        );

        let sec_def_pos = get_drifted_defender_position(sec_lead, spatial_map, attribute_keys, rng)
            .unwrap_or(receiver_pos_vec);
        let sec_def_mult = compute_player_fatigue_multiplier(
            sec_lead,
            &fatigue_for(&sec_lead.id()),
            attribute_keys,
        );
        let sec_def_spd = calculate_player_speed(sec_lead, attribute_keys, sec_def_mult);

        let rec_mult = compute_player_fatigue_multiplier(
            &receiver_player,
            &fatigue_for(&receiver_player.id()),
            attribute_keys,
        );
        let rec_spd = calculate_player_speed(&receiver_player, attribute_keys, rec_mult);
        let sec_duration =
            derive_duel_duration(receiver_pos_vec, rec_spd, sec_def_pos, sec_def_spd);

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
        ledger.record_live(
            DurationComponentKind::BallSecurityEngagement,
            sec_duration,
        );

        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover: sec_result.turnover_team_id,
            recovering_player_id: sec_result.recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger: ledger,
            end_position: start_pos,
            duels: vec![dist_duel, reception_outcome.duel, sec_result.duel_outcome],
            receiver_id: Some(receiver_id),
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
    }
}