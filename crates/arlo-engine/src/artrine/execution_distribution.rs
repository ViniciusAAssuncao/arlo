use crate::artrine::execution_distribution_reception::execute_post_throw_reception;
use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::fatigue::{compute_player_fatigue_multiplier, FatigueState};
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index, calculate_side_rating_from_index,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed, calculate_pass_speed,
};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::interception::identify_kinematic_lead_defender_with_drift;
use crate::spatial::positioning_drift::{get_drifted_defender_position, nearest_drifted_opponent};
use crate::spatial::proximity::{calculate_distance_mirim, filter_active_duelists_swept};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Speed, Velocity, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_distribution<F, R>(
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
    let duel_kind = match decision_kind {
        ArtrineDecisionKind::ShortPass => DuelKind::ShortDistribution,
        ArtrineDecisionKind::LongLaunch => DuelKind::LongDistribution,
        ArtrineDecisionKind::Cross => DuelKind::CrossDistribution,
        _ => DuelKind::ShortDistribution,
    };

    let (offense_profile, defense_profile) = get_duel_profiles(duel_kind);
    let attacker_rating = calculate_anchored_side_rating_from_index(
        artrine,
        DomainPosition::Artrine,
        offense_helpers,
        offense_position_index,
        attribute_keys,
        &offense_profile,
    );
    let defender_rating = calculate_side_rating_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
    );

    let contest_radius = Length::new(PROXIMITY_CONTEST_RADIUS_MIRIM * MIRIM_TO_METERS);
    let lead_defender = identify_kinematic_lead_defender_with_drift(
        start_pos,
        Velocity::zero(),
        defenders,
        spatial_map,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    )
    .unwrap_or(defenders[0]);

    let raw_dist_duel = resolve_duel(
        duel_kind,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        attribute_keys,
        context,
        rng,
    );

    let artrine_fatigue_mult =
        compute_player_fatigue_multiplier(artrine, &fatigue_for(&artrine.id()), attribute_keys);
    let artrine_speed =
        calculate_player_speed(artrine, attribute_keys, artrine_fatigue_mult);

    let (dist_duration, nearest_def_opt) =
        match nearest_drifted_opponent(start_pos, defenders, spatial_map, attribute_keys, rng) {
            Some((d, pos)) => {
                let d_mult = compute_player_fatigue_multiplier(d, &fatigue_for(&d.id()), attribute_keys);
                let d_spd = calculate_player_speed(d, attribute_keys, d_mult);
                (
                    derive_duel_duration(start_pos, artrine_speed, pos, d_spd),
                    Some((d, pos)),
                )
            }
            None => (Duration::new(MINIMUM_ENGAGEMENT_SECONDS), None),
        };

    let helper_candidates: Vec<(&Player, VectorPosition, Speed)> = offense_helpers
        .iter()
        .map(|&p| {
            let pos = spatial_map.get_position(&p.id()).unwrap_or(start_pos);
            let mult = compute_player_fatigue_multiplier(p, &fatigue_for(&p.id()), attribute_keys);
            let spd = calculate_player_speed(p, attribute_keys, mult);
            (p, pos, spd)
        })
        .collect();

    let mut dist_attacker_ids = vec![artrine.id()];
    for id in filter_active_duelists_swept(
        start_pos,
        Velocity::zero(),
        &helper_candidates,
        contest_radius,
        dist_duration,
    ) {
        if !dist_attacker_ids.contains(&id) {
            dist_attacker_ids.push(id);
        }
    }

    let defender_candidates: Vec<(&Player, VectorPosition, Speed)> = defenders
        .iter()
        .map(|&p| {
            let pos = get_drifted_defender_position(p, spatial_map, attribute_keys, rng)
                .or_else(|| spatial_map.get_position(&p.id()))
                .unwrap_or(start_pos);
            let mult = compute_player_fatigue_multiplier(p, &fatigue_for(&p.id()), attribute_keys);
            let spd = calculate_player_speed(p, attribute_keys, mult);
            (p, pos, spd)
        })
        .collect();

    let mut dist_defender_ids = vec![lead_defender.id()];
    for id in filter_active_duelists_swept(
        start_pos,
        Velocity::zero(),
        &defender_candidates,
        contest_radius,
        dist_duration,
    ) {
        if !dist_defender_ids.contains(&id) {
            dist_defender_ids.push(id);
        }
    }

    let dist_duel = AttributedDuelOutcome::new(
        raw_dist_duel,
        dist_attacker_ids,
        dist_defender_ids,
    );

    if !dist_duel.outcome().attacker_won() {
        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::DistributionEngagement,
            dist_duration,
        );

        let (close_defenders, closest_def_info) = match nearest_def_opt {
            Some((d, pos))
                if calculate_distance_mirim(start_pos, pos) <= PROXIMITY_CONTEST_RADIUS_MIRIM =>
            {
                let list: Vec<&Player> = defenders
                    .iter()
                    .copied()
                    .filter(|cand| {
                        get_drifted_defender_position(cand, spatial_map, attribute_keys, rng)
                            .map(|p| {
                                calculate_distance_mirim(start_pos, p)
                                    <= PROXIMITY_CONTEST_RADIUS_MIRIM
                            })
                            .unwrap_or(false)
                    })
                    .collect();
                (list, Some((d, pos)))
            }
            _ => (Vec::new(), None),
        };

        let (turnover, recovering_player_id, duels) = if !close_defenders.is_empty() {
            if let Some((closest_def, closest_pos)) = closest_def_info {
                let closest_def_mult = compute_player_fatigue_multiplier(
                    closest_def,
                    &fatigue_for(&closest_def.id()),
                    attribute_keys,
                );
                let closest_def_speed =
                    calculate_player_speed(closest_def, attribute_keys, closest_def_mult);
                let sec_duration = derive_duel_duration(
                    start_pos,
                    artrine_speed,
                    closest_pos,
                    closest_def_speed,
                );
                ledger.record_live(
                    DurationComponentKind::BallSecurityEngagement,
                    sec_duration,
                );
            }
            let sec_result = resolve_ball_security(
                DuelKind::BallSecurityDistribution,
                artrine,
                DomainPosition::Artrine,
                &close_defenders,
                defense_position_index,
                attribute_keys,
                defense_team_id,
                context,
                rng,
            );
            (
                sec_result.turnover_team_id,
                sec_result.recovering_player_id,
                vec![dist_duel, sec_result.duel_outcome],
            )
        } else {
            (None, None, vec![dist_duel])
        };

        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger: ledger,
            end_position: start_pos,
            duels,
            receiver_id: None,
            distribution_flight: None,
        };
    }

    let progression_strategy = AggregateProgressionStrategy::default();
    let throw_advance = progression_strategy.resolve_progression(dist_duel.outcome(), rng);

    let ball_speed = match decision_kind {
        ArtrineDecisionKind::Cross => calculate_cross_speed(artrine, attribute_keys),
        _ => calculate_pass_speed(artrine, attribute_keys),
    };
    let flight_duration = ball_flight_duration(throw_advance, ball_speed);

    execute_post_throw_reception(
        decision_kind,
        artrine,
        offense_helpers,
        offense_position_index,
        defenders,
        defense_position_index,
        attribute_keys,
        pitch,
        spatial_map,
        start_pos,
        throw_advance,
        flight_duration,
        dist_duration,
        dist_duel,
        attacking_positive_x,
        defense_team_id,
        context,
        fatigue_for,
        rng,
    )
}