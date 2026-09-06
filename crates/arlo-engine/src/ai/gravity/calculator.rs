use crate::ai::gravity::model::OffensiveGravity;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::DynamicSpatialMap;
use crate::tactics::calculate_fit_for_position;
use crate::weighting::calculate_weighted_saturated_average;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD,
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIRST_ZONE_DEPTH_MIRIM,
};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_offensive_gravity(
    player: &Player,
    assigned_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    zone_factor: f64,
) -> OffensiveGravity {
    let finishing_attrs = [
        (extract_attribute_value(player, attribute_keys, AttributeKey::Finishing), 5.0),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Composure), 4.0),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Anticipation), 3.5),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Technique), 3.5),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Positioning), 3.0),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Decisions), 2.5),
    ];

    let creation_attrs = [
        (extract_attribute_value(player, attribute_keys, AttributeKey::Passing), 4.5),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Vision), 4.5),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Flair), 3.5),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Agility), 3.0),
        (extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration), 3.0),
        (extract_attribute_value(player, attribute_keys, AttributeKey::ArloControl), 3.0),
    ];

    let finishing_avg = calculate_weighted_saturated_average(
        &finishing_attrs,
        ATTRIBUTE_SATURATION_THRESHOLD,
        ATTRIBUTE_SATURATION_MULTIPLIER,
    )
    .unwrap_or(10.0);

    let creation_avg = calculate_weighted_saturated_average(
        &creation_attrs,
        ATTRIBUTE_SATURATION_THRESHOLD,
        ATTRIBUTE_SATURATION_MULTIPLIER,
    )
    .unwrap_or(10.0);

    let fit = calculate_fit_for_position(player, assigned_position);
    let fit_mult = fit.efficiency_multiplier();

    let finishing_threat = (finishing_avg / 10.0).powf(1.4) * fit_mult;
    let creation_threat = (creation_avg / 10.0) * fit_mult;

    let position_role_weight = match assigned_position {
        Position::CenterOffense => 0.85,
        Position::WingOffense | Position::WideEnd => 0.65,
        Position::TightWing | Position::RunningEnd | Position::Corridor => 0.50,
        _ => 0.30,
    };

    let composite_threat = (finishing_threat * position_role_weight)
        + (creation_threat * (1.0 - position_role_weight));

    let raw_curve = 0.50 + 1.50 / (1.0 + (-2.2 * (composite_threat - 1.15)).exp());
    let multiplier = (raw_curve * zone_factor).clamp(0.5, 2.0);

    OffensiveGravity::new(
        multiplier,
        finishing_threat,
        creation_threat,
        zone_factor,
    )
}

pub fn calculate_zone_factor(
    player_pos: VectorPosition,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let pitch_length_m = pitch.length().value();
    let player_x_m = player_pos.raw().0;

    let dist_to_goal_m = if attacking_positive_x {
        (pitch_length_m - player_x_m).max(0.0)
    } else {
        player_x_m.max(0.0)
    };

    let first_zone_limit_m = FIRST_ZONE_DEPTH_MIRIM * MIRIM_TO_METERS;
    let second_zone_limit_m =
        (FIRST_ZONE_DEPTH_MIRIM + AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM) * MIRIM_TO_METERS;

    if dist_to_goal_m <= first_zone_limit_m {
        1.25
    } else if dist_to_goal_m <= second_zone_limit_m {
        1.15
    } else if dist_to_goal_m <= second_zone_limit_m + (10.0 * MIRIM_TO_METERS) {
        1.05
    } else {
        1.00
    }
}

pub fn calculate_team_max_finishing_gravity(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
) -> OffensiveGravity {
    if players.is_empty() {
        return OffensiveGravity::default();
    }

    let mut best_gravity = OffensiveGravity::default();

    for &player in players {
        let assigned_pos = position_index
            .get(&player.id())
            .copied()
            .unwrap_or_else(|| {
                player
                    .positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::CenterOffense)
            });

        let player_vec_pos = spatial_map
            .get_position(&player.id())
            .unwrap_or_else(VectorPosition::zero);

        let zone_factor = calculate_zone_factor(player_vec_pos, pitch, attacking_positive_x);

        let grav = calculate_player_offensive_gravity(
            player,
            assigned_pos,
            attribute_keys,
            zone_factor,
        );

        if grav.multiplier() > best_gravity.multiplier() {
            best_gravity = grav;
        }
    }

    best_gravity
}