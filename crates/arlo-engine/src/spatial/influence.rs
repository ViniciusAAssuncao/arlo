use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::proximity::calculate_distance;
use arlo_domain::pitch::artro_rows_for_pitch;
use arlo_domain::{AttributeKey, Pitch, Player};
use arlo_math::units::Position;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn defender_variance(defender: &Player, attribute_keys: &HashMap<Uuid, AttributeKey>) -> f64 {
    let accel = extract_attribute_value(defender, attribute_keys, AttributeKey::Acceleration);
    let pace = extract_attribute_value(defender, attribute_keys, AttributeKey::Pace);
    2.0 + (0.35 * pace) + (0.35 * accel)
}

pub fn defender_projected_mean(defender: &Player, spatial_map: &DynamicSpatialMap) -> Position {
    let pos = spatial_map
        .get_position(&defender.id())
        .unwrap_or_else(Position::zero);
    let vel = spatial_map
        .get_velocity(&defender.id())
        .unwrap_or_else(arlo_math::units::Velocity::zero);

    Position::from_components(
        pos.raw().0 + vel.raw().0,
        pos.raw().1 + vel.raw().1,
        0.0,
    )
}

pub fn calculate_point_resistance(
    point: Position,
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let mut total_resistance = 0.0;

    for defender in defenders {
        let mean = defender_projected_mean(defender, spatial_map);
        let variance = defender_variance(defender, attribute_keys);

        let dx = point.raw().0 - mean.raw().0;
        let dy = point.raw().1 - mean.raw().1;
        let d_sq = (dx * dx) + (dy * dy);

        let decay = (-d_sq / (2.0 * variance)).exp();
        total_resistance += decay;
    }

    total_resistance
}

pub fn calculate_spatial_resistance_between(
    start_pos: Position,
    target_pos: Position,
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    if defenders.is_empty() {
        return 0.0;
    }

    let steps = 10;
    let mut accumulated = 0.0;

    for i in 0..=steps {
        let t = (i as f64) / (steps as f64);
        let x = start_pos.raw().0 + t * (target_pos.raw().0 - start_pos.raw().0);
        let y = start_pos.raw().1 + t * (target_pos.raw().1 - start_pos.raw().1);
        let pt = Position::from_components(x, y, 0.0);

        accumulated += calculate_point_resistance(pt, defenders, spatial_map, attribute_keys);
    }

    accumulated / ((steps + 1) as f64)
}

pub fn calculate_spatial_resistance(
    target_vector: Position,
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let defender_ids: HashSet<Uuid> = defenders.iter().map(|p| p.id()).collect();
    let start_pos = spatial_map
        .positions()
        .iter()
        .filter(|(id, _)| !defender_ids.contains(id))
        .min_by(|(_, a), (_, b)| {
            let dist_a = calculate_distance(**a, target_vector).value();
            let dist_b = calculate_distance(**b, target_vector).value();
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, pos)| *pos)
        .unwrap_or_else(Position::zero);

    calculate_spatial_resistance_between(
        start_pos,
        target_vector,
        defenders,
        spatial_map,
        attribute_keys,
    )
}

pub fn find_next_artro_position(
    artrine_pos: Position,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> Position {
    let rows = artro_rows_for_pitch(pitch);
    let artrine_x = artrine_pos.raw().0;

    let candidate_rows: Vec<_> = rows
        .into_iter()
        .filter(|r| {
            let rx = r.x().value();
            if attacking_positive_x {
                rx > artrine_x
            } else {
                rx < artrine_x
            }
        })
        .collect();

    let eligible_rows = if candidate_rows.is_empty() {
        artro_rows_for_pitch(pitch)
    } else {
        candidate_rows
    };

    let mut best_artro_pos = artrine_pos;
    let mut min_dist = f64::MAX;

    for row in &eligible_rows {
        for artro in row.artros() {
            let artro_pos = Position::from_components(artro.x().value(), artro.y().value(), 0.0);
            let dist = calculate_distance(artrine_pos, artro_pos).value();
            if dist < min_dist {
                min_dist = dist;
                best_artro_pos = artro_pos;
            }
        }
    }

    best_artro_pos
}