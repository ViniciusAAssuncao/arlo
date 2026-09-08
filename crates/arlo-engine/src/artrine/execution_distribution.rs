use crate::artrine::execution_distribution_reception::execute_post_throw_reception;
use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_side_rating_from_index_with_fatigue,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed_with_state, calculate_pass_speed_with_state,
};
use crate::spatial::interception::identify_kinematic_lead_defender_with_drift;
use crate::spatial::positioning_drift::{get_drifted_defender_position, nearest_drifted_opponent};
use crate::spatial::proximity::{calculate_distance_mirim, filter_active_duelists_swept};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition, SlotRole};
use arlo_math::units::{
    Duration, Length, Position as VectorPosition, Speed, Velocity, MIRIM_TO_METERS,
};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_distribution<F, R>(
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    offense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    attacking_positive_x: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    goalguard: &Player,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    context: &DuelContext,
    fatigue_for: &F,
    defense_pressing_multiplier: f64,
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
    let attacker_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        artrine,
        DomainPosition::Artrine,
        offense_helpers,
        offense_position_index,
        attribute_keys,
        &offense_profile,
        fatigue_for,
    );
    let defender_rating = calculate_side_rating_from_index_with_fatigue(
        defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
        fatigue_for,
    );

    let contest_radius = Length::new(PROXIMITY_CONTEST_RADIUS_MIRIM * defense_pressing_multiplier * MIRIM_TO_METERS);
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

    let artrine_state = fatigue_for(&artrine.id());
    let lead_def_state = fatigue_for(&lead_defender.id());

    let dist_context = context.for_duel_kind(duel_kind);
    let raw_dist_duel = resolve_duel_with_fatigue(
        duel_kind,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        &artrine_state,
        &lead_def_state,
        attribute_keys,
        &dist_context,
        rng,
    );

    let artrine_speed = calculate_effective_player_speed(artrine, attribute_keys, &artrine_state);

    let (dist_duration, nearest_def_opt) = match
        nearest_drifted_opponent(start_pos, defenders, spatial_map, attribute_keys, rng)
    {
        Some((d, pos)) => {
            let d_state = fatigue_for(&d.id());
            let d_spd = calculate_effective_player_speed(d, attribute_keys, &d_state);
            (derive_duel_duration(start_pos, artrine_speed, pos, d_spd), Some((d, pos)))
        }
        None => (Duration::new(MINIMUM_ENGAGEMENT_SECONDS), None),
    };

    let helper_candidates: Vec<(&Player, VectorPosition, Speed)> = offense_helpers
        .iter()
        .map(|&p| {
            let pos = spatial_map.get_position(&p.id()).unwrap_or(start_pos);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
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
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
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

    let dist_duel = AttributedDuelOutcome::new(raw_dist_duel, dist_attacker_ids, dist_defender_ids);

    if !dist_duel.outcome().attacker_won() {
        let mut ledger = DurationLedger::new();
        ledger.record_live(DurationComponentKind::DistributionEngagement, dist_duration);

        let (close_defenders, closest_def_info) = match nearest_def_opt {
            Some((d, pos)) if
                calculate_distance_mirim(start_pos, pos) <= PROXIMITY_CONTEST_RADIUS_MIRIM
            => {
                let list: Vec<&Player> = defenders
                    .iter()
                    .copied()
                    .filter(|cand| {
                        get_drifted_defender_position(cand, spatial_map, attribute_keys, rng)
                            .map(|p| {
                                calculate_distance_mirim(start_pos, p) <=
                                    PROXIMITY_CONTEST_RADIUS_MIRIM
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
                let closest_def_state = fatigue_for(&closest_def.id());
                let closest_def_speed = calculate_effective_player_speed(
                    closest_def,
                    attribute_keys,
                    &closest_def_state,
                );
                let sec_duration = derive_duel_duration(
                    start_pos,
                    artrine_speed,
                    closest_pos,
                    closest_def_speed,
                );
                ledger.record_live(DurationComponentKind::BallSecurityEngagement, sec_duration);
            }
            let sec_context = context.for_duel_kind(DuelKind::BallSecurityDistribution);
            let sec_result = resolve_ball_security(
                DuelKind::BallSecurityDistribution,
                artrine,
                DomainPosition::Artrine,
                &close_defenders,
                defense_position_index,
                attribute_keys,
                defense_team_id,
                &sec_context,
                fatigue_for,
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
            kinematic_trajectories: HashMap::new(),
        };
    }

    let progression_strategy = AggregateProgressionStrategy::default();
    let throw_advance = progression_strategy.resolve_progression(dist_duel.outcome(), rng);

    let ball_speed = match decision_kind {
        ArtrineDecisionKind::Cross =>
            calculate_cross_speed_with_state(artrine, attribute_keys, &artrine_state),
        _ => calculate_pass_speed_with_state(artrine, attribute_keys, &artrine_state),
    };
    let flight_duration = ball_flight_duration(throw_advance, ball_speed);

    execute_post_throw_reception(
        decision_kind,
        artrine,
        offense_helpers,
        offense_position_index,
        offense_role_index,
        offense_instructions_index,
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
        offense_team_id,
        defense_team_id,
        goalguard,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        is_bonus_phase,
        context,
        fatigue_for,
        defense_pressing_multiplier,
        rng,
    )
}