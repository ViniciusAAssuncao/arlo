use crate::physical::{compute_player_fatigue_multiplier, FatigueState};
use crate::spatial::decision_vector::{
    calculate_player_speed, derive_velocity_towards_target, extract_attribute_value,
};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::{
    calculate_time_to_direct_intercept, calculate_time_to_moving_intercept,
};
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Player};
use arlo_math::compute_swept_sphere_intersection;
use arlo_math::units::{Duration, Length, Position, Speed, Velocity};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_defender_tti(
    defender: &Player,
    target_pos: Position,
    target_vel: Velocity,
    defender_pos: Position,
    defender_speed: Speed,
    contest_radius: Length,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> (Option<Duration>, f64) {
    let defender_vel = derive_velocity_towards_target(defender_pos, target_pos, defender_speed);
    let swept_t = compute_swept_sphere_intersection(
        target_pos,
        target_vel,
        defender_pos,
        defender_vel,
        contest_radius,
    );

    let raw_t = match swept_t {
        Some(d) => Some(d),
        None => calculate_time_to_moving_intercept(
            defender_pos,
            defender_speed,
            target_pos,
            target_vel,
        )
        .or_else(|| calculate_time_to_direct_intercept(defender_pos, defender_speed, target_pos)),
    };

    let ant = extract_attribute_value(defender, attribute_keys, AttributeKey::Anticipation);
    let pos = extract_attribute_value(defender, attribute_keys, AttributeKey::Positioning);
    let mental_scale = (1.0 - (ant * 0.015 + pos * 0.015)).clamp(0.35, 1.35);

    let effective_tti = match raw_t {
        Some(d) => d.value() * mental_scale,
        None => f64::INFINITY,
    };

    (raw_t, effective_tti)
}

pub fn identify_kinematic_lead_defender<'a, F>(
    target_pos: Position,
    target_vel: Velocity,
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    contest_radius: Length,
    max_duration: Option<Duration>,
) -> Option<&'a Player>
where
    F: Fn(&Uuid) -> FatigueState,
{
    if defenders.is_empty() {
        return None;
    }

    let mut best_defender: Option<&'a Player> = None;
    let mut min_effective_tti = f64::INFINITY;

    for &defender in defenders {
        let def_pos = spatial_map.get_position(&defender.id()).unwrap_or(target_pos);
        let fatigue = fatigue_for(&defender.id());
        let mult = compute_player_fatigue_multiplier(defender, &fatigue, attribute_keys);
        let speed = calculate_player_speed(defender, attribute_keys, mult);

        let (raw_t, effective_tti) = calculate_defender_tti(
            defender,
            target_pos,
            target_vel,
            def_pos,
            speed,
            contest_radius,
            attribute_keys,
        );

        if let Some(max_d) = max_duration {
            if let Some(t) = raw_t {
                if t.value() > max_d.value() {
                    continue;
                }
            } else {
                continue;
            }
        }

        if effective_tti < min_effective_tti {
            min_effective_tti = effective_tti;
            best_defender = Some(defender);
        }
    }

    best_defender.or_else(|| defenders.first().copied())
}

pub fn identify_kinematic_lead_defender_with_drift<'a, F, R>(
    target_pos: Position,
    target_vel: Velocity,
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    contest_radius: Length,
    max_duration: Option<Duration>,
    rng: &mut R,
) -> Option<&'a Player>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    if defenders.is_empty() {
        return None;
    }

    let mut best_defender: Option<&'a Player> = None;
    let mut min_effective_tti = f64::INFINITY;

    for &defender in defenders {
        let def_pos = get_drifted_defender_position(defender, spatial_map, attribute_keys, rng)
            .or_else(|| spatial_map.get_position(&defender.id()))
            .unwrap_or(target_pos);
        let fatigue = fatigue_for(&defender.id());
        let mult = compute_player_fatigue_multiplier(defender, &fatigue, attribute_keys);
        let speed = calculate_player_speed(defender, attribute_keys, mult);

        let (raw_t, effective_tti) = calculate_defender_tti(
            defender,
            target_pos,
            target_vel,
            def_pos,
            speed,
            contest_radius,
            attribute_keys,
        );

        if let Some(max_d) = max_duration {
            if let Some(t) = raw_t {
                if t.value() > max_d.value() {
                    continue;
                }
            } else {
                continue;
            }
        }

        if effective_tti < min_effective_tti {
            min_effective_tti = effective_tti;
            best_defender = Some(defender);
        }
    }

    best_defender.or_else(|| defenders.first().copied())
}

pub fn filter_kinematic_active_duelists(
    target_pos: Position,
    target_vel: Velocity,
    candidates: &[(&Player, Position, Speed)],
    contest_radius: Length,
    max_duration: Duration,
) -> Vec<Uuid> {
    candidates
        .iter()
        .filter(|(_, pos, speed)| {
            let vel = derive_velocity_towards_target(*pos, target_pos, *speed);
            if let Some(t) = compute_swept_sphere_intersection(
                target_pos,
                target_vel,
                *pos,
                vel,
                contest_radius,
            ) {
                t.value() <= max_duration.value()
            } else {
                false
            }
        })
        .map(|(player, _, _)| player.id())
        .collect()
}