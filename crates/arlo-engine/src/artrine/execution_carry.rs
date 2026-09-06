use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::possession::drive::artrine_identity::TrueArtrine;
use crate::possession::drive::validator::validate_continuous_trajectory;
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
use crate::spatial::decision_vector::derive_velocity_towards_target;
use crate::spatial::interception::identify_kinematic_lead_defender_with_drift;
use crate::spatial::positioning_drift::{get_drifted_defender_position, nearest_drifted_opponent};
use crate::spatial::proximity::{calculate_distance_mirim, filter_active_duelists_swept};
use crate::spatial::run_spatial_tick_loop;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::{artro_rows_for_pitch, Pitch};
use arlo_domain::sport_constants::{
    DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM, MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM,
};
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Speed, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_carry<F, R>(
    artrine: &Player,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &mut DynamicSpatialMap,
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
    let (offense_profile, defense_profile) = get_duel_profiles(DuelKind::ArtroBreakthrough);
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

    let artrine_state = fatigue_for(&artrine.id());
    let artrine_speed = calculate_effective_player_speed(artrine, attribute_keys, &artrine_state);

    let pitch_width_m = pitch.width().value();
    let center_y_m = pitch_width_m / 2.0;
    let left_y_m = center_y_m - DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM * MIRIM_TO_METERS;
    let right_y_m = center_y_m + DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM * MIRIM_TO_METERS;

    let target_channel_y_m = [left_y_m, center_y_m, right_y_m]
        .into_iter()
        .min_by(|&a, &b| {
            let da = (a - start_pos.raw().1).abs();
            let db = (b - start_pos.raw().1).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(center_y_m);

    let forward_x_mirim = if attacking_positive_x {
        (start_pos.raw().0 / MIRIM_TO_METERS + 10.0).min(pitch.length_mirim())
    } else {
        (start_pos.raw().0 / MIRIM_TO_METERS - 10.0).max(0.0)
    };
    let target_carry_pos = VectorPosition::from_components(
        forward_x_mirim * MIRIM_TO_METERS,
        target_channel_y_m,
        0.0,
    );
    let carrier_vel = derive_velocity_towards_target(start_pos, target_carry_pos, artrine_speed);

    let contest_radius = Length::new(PROXIMITY_CONTEST_RADIUS_MIRIM * MIRIM_TO_METERS);
    let lead_defender = identify_kinematic_lead_defender_with_drift(
        start_pos,
        carrier_vel,
        defenders,
        spatial_map,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    )
    .unwrap_or(defenders[0]);

    let raw_artro_duel = resolve_duel_with_fatigue(
        DuelKind::ArtroBreakthrough,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        &artrine_state,
        &fatigue_for(&lead_defender.id()),
        attribute_keys,
        context,
        rng,
    );

    let (artro_duration, nearest_def_opt) =
        match nearest_drifted_opponent(start_pos, defenders, spatial_map, attribute_keys, rng) {
            Some((d, pos)) => {
                let d_state = fatigue_for(&d.id());
                let d_spd = calculate_effective_player_speed(d, attribute_keys, &d_state);
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
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
            (p, pos, spd)
        })
        .collect();

    let mut artro_attacker_ids = vec![artrine.id()];
    for id in filter_active_duelists_swept(
        start_pos,
        carrier_vel,
        &helper_candidates,
        contest_radius,
        artro_duration,
    ) {
        if !artro_attacker_ids.contains(&id) {
            artro_attacker_ids.push(id);
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

    let mut artro_defender_ids = vec![lead_defender.id()];
    for id in filter_active_duelists_swept(
        start_pos,
        carrier_vel,
        &defender_candidates,
        contest_radius,
        artro_duration,
    ) {
        if !artro_defender_ids.contains(&id) {
            artro_defender_ids.push(id);
        }
    }

    let artro_duel = AttributedDuelOutcome::new(
        raw_artro_duel,
        artro_attacker_ids,
        artro_defender_ids,
    );

    if !artro_duel.outcome().attacker_won() {
        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::ArtroBreakthroughEngagement,
            artro_duration,
        );

        let (close_defenders, closest_def_info) = match nearest_def_opt {
            Some((d, pos)) if calculate_distance_mirim(start_pos, pos) <= PROXIMITY_CONTEST_RADIUS_MIRIM => {
                let list: Vec<&Player> = defenders
                    .iter()
                    .copied()
                    .filter(|cand| {
                        get_drifted_defender_position(cand, spatial_map, attribute_keys, rng)
                            .map(|p| calculate_distance_mirim(start_pos, p) <= PROXIMITY_CONTEST_RADIUS_MIRIM)
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
                let closest_def_speed =
                    calculate_effective_player_speed(closest_def, attribute_keys, &closest_def_state);
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
                DuelKind::BallSecurityCarry,
                artrine,
                DomainPosition::Artrine,
                &close_defenders,
                defense_position_index,
                attribute_keys,
                defense_team_id,
                context,
                fatigue_for,
                rng,
            );
            (
                sec_result.turnover_team_id,
                sec_result.recovering_player_id,
                vec![artro_duel, sec_result.duel_outcome],
            )
        } else {
            (None, None, vec![artro_duel])
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
    let raw_advance = progression_strategy.resolve_progression(artro_duel.outcome(), rng);

    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if attacking_positive_x {
        (start_x_mirim + raw_advance).min(pitch.length_mirim())
    } else {
        (start_x_mirim - raw_advance).max(0.0)
    };

    let target_pos = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        target_channel_y_m,
        0.0,
    );

    spatial_map.set_position(artrine.id(), start_pos);
    spatial_map.apply_breakthrough_momentum(
        artrine,
        target_pos,
        artro_duel.outcome().net_advantage(),
        attribute_keys,
        artrine_state.energy(),
    );

    let mut movers = Vec::with_capacity(1 + offense_helpers.len() + defenders.len());
    movers.push((artrine, target_pos));

    for &helper in offense_helpers {
        if let Some(pos) = spatial_map.get_position(&helper.id()) {
            let offset_x = if attacking_positive_x { raw_advance * 0.7 * MIRIM_TO_METERS } else { -raw_advance * 0.7 * MIRIM_TO_METERS };
            let helper_target = VectorPosition::from_components(pos.raw().0 + offset_x, pos.raw().1, 0.0);
            movers.push((helper, helper_target));
        }
    }

    for &defender in defenders {
        movers.push((defender, target_pos));
    }

    let tick_result = run_spatial_tick_loop(spatial_map, &movers, attribute_keys);

    let end_position = spatial_map
        .get_position(&artrine.id())
        .unwrap_or(target_pos);
    let mirins_advanced = (end_position.raw().0 - start_pos.raw().0).abs() / MIRIM_TO_METERS;

    let all_rows = artro_rows_for_pitch(pitch);
    let true_artrine = TrueArtrine::new(artrine.id());
    let mut drive_row_indices = Vec::new();

    let trajectory = tick_result.get_trajectory(&artrine.id());
    let segments = if let Some(traj) = trajectory {
        traj.segments()
    } else {
        Vec::new()
    };

    let segments_to_test = if segments.is_empty() {
        vec![(start_pos, end_position)]
    } else {
        segments
    };

    let (min_x, max_x) = if start_pos.raw().0 < end_position.raw().0 {
        (start_pos.raw().0, end_position.raw().0)
    } else {
        (end_position.raw().0, start_pos.raw().0)
    };

    let artro_search_margin = 1.5 * MIRIM_TO_METERS;
    for row in &all_rows {
        let rx = row.x().value();
        if rx < min_x - artro_search_margin || rx > max_x + artro_search_margin {
            continue;
        }
        for artro in row.artros() {
            let drive_result = validate_continuous_trajectory(true_artrine, &segments_to_test, artro, false);
            if drive_result.is_valid() {
                if !drive_row_indices.contains(&row.row_index()) {
                    drive_row_indices.push(row.row_index());
                }
                break;
            }
        }
    }

    if attacking_positive_x {
        drive_row_indices.sort();
    } else {
        drive_row_indices.sort_by(|a, b| b.cmp(a));
    }

    let drives_recorded = drive_row_indices.len() as u32;

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::ArtroBreakthroughEngagement,
        artro_duration,
    );
    ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(tick_result.elapsed_seconds()),
    );

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded,
        drive_row_indices,
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        duration_ledger: ledger,
        end_position,
        duels: vec![artro_duel],
        receiver_id: None,
        distribution_flight: None,
        kinematic_trajectories: tick_result.trajectories().clone(),
    }
}