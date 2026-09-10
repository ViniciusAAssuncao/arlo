use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{ATTRIBUTE_MAX, FIRST_ZONE_DEPTH_MIRIM};
use arlo_domain::{AttributeKey, Player, Position, PositionLine};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_carrier_defensive_shift(
    defender: &Player,
    defender_pos: VectorPosition,
    carrier_pos: VectorPosition,
    carrier_gravity_mult: f64,
    assigned_position: Position,
    pitch: &Pitch,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> VectorPosition {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let center_y_m = pitch_width_m * 0.5;

    let goal_x_m = if attacking_positive_x {
        pitch_length_m
    } else {
        0.0
    };

    let first_zone_boundary_x_m = if attacking_positive_x {
        (pitch_length_m - FIRST_ZONE_DEPTH_MIRIM * MIRIM_TO_METERS).max(0.0)
    } else {
        (FIRST_ZONE_DEPTH_MIRIM * MIRIM_TO_METERS).min(pitch_length_m)
    };

    let positioning = extract_attribute_value(defender, attribute_keys, AttributeKey::Positioning);
    let anticipation = extract_attribute_value(defender, attribute_keys, AttributeKey::Anticipation);
    let decisions = extract_attribute_value(defender, attribute_keys, AttributeKey::Decisions);
    let tactical_knowledge =
        extract_attribute_value(defender, attribute_keys, AttributeKey::TacticalKnowledge);
    let containment =
        extract_attribute_value(defender, attribute_keys, AttributeKey::DefensiveContainment);

    let tactical_rating = ((positioning * 0.30
        + anticipation * 0.25
        + decisions * 0.20
        + tactical_knowledge * 0.15
        + containment * 0.10)
        / ATTRIBUTE_MAX)
        .clamp(0.2, 1.0);

    let dist_to_carrier_mirim = calculate_distance_mirim(defender_pos, carrier_pos);
    let proximity_factor = (1.0 / (1.0 + dist_to_carrier_mirim * 0.08)).clamp(0.2, 1.0);
    let gravity_factor = (carrier_gravity_mult / 1.5).clamp(0.6, 1.8);

    let base_x = defender_pos.raw().0;
    let base_y = defender_pos.raw().1;
    let carrier_x = carrier_pos.raw().0;
    let carrier_y = carrier_pos.raw().1;

    let (target_x, target_y) = match assigned_position {
        Position::Lineback => {
            let intermediate_x =
                base_x + (carrier_x - base_x) * (0.45 * tactical_rating * gravity_factor);
            let intermediate_y =
                base_y + (carrier_y - base_y) * (0.55 * tactical_rating * gravity_factor);
            let goal_funnel_x = intermediate_x + (goal_x_m - intermediate_x) * 0.15;
            let goal_funnel_y = intermediate_y + (center_y_m - intermediate_y) * 0.20;
            (goal_funnel_x, goal_funnel_y)
        }
        Position::MiddleZonerback => {
            let shift_x = base_x + (first_zone_boundary_x_m - base_x) * (0.35 * tactical_rating);
            let shift_y = base_y + (carrier_y - base_y) * (0.50 * tactical_rating * proximity_factor);
            (shift_x, shift_y)
        }
        Position::OutsideZonerback => {
            let flank_shift_x = base_x + (carrier_x - base_x) * (0.35 * tactical_rating);
            let same_side = (base_y - center_y_m) * (carrier_y - center_y_m) > 0.0;
            let flank_shift_y = if same_side {
                base_y + (carrier_y - base_y) * (0.60 * tactical_rating)
            } else {
                base_y + (center_y_m - base_y) * (0.25 * tactical_rating)
            };
            (flank_shift_x, flank_shift_y)
        }
        Position::Centerback => {
            let anchor_x = base_x + (first_zone_boundary_x_m - base_x) * (0.40 * tactical_rating);
            let anchor_y = base_y + (carrier_y - base_y) * (0.30 * tactical_rating);
            (anchor_x, anchor_y)
        }
        Position::DefensiveEnd | Position::Rougieback | Position::PassRusher => {
            let direct_x =
                base_x + (carrier_x - base_x) * (0.55 * tactical_rating * proximity_factor);
            let direct_y =
                base_y + (carrier_y - base_y) * (0.55 * tactical_rating * proximity_factor);
            (direct_x, direct_y)
        }
        Position::Goalguard => {
            let goalguard_x = base_x + (first_zone_boundary_x_m - base_x) * 0.10;
            let goalguard_y = center_y_m + (carrier_y - center_y_m) * 0.25;
            (goalguard_x, goalguard_y)
        }
        _ => match assigned_position.line() {
            PositionLine::DefenseLine => {
                let shift_x = base_x + (carrier_x - base_x) * (0.30 * tactical_rating);
                let shift_y = base_y + (carrier_y - base_y) * (0.35 * tactical_rating);
                (shift_x, shift_y)
            }
            PositionLine::BackLine => {
                let shift_x = base_x + (carrier_x - base_x) * (0.40 * tactical_rating);
                let shift_y = base_y + (carrier_y - base_y) * (0.40 * tactical_rating);
                (shift_x, shift_y)
            }
            _ => (base_x, base_y),
        },
    };

    let min_x = 0.5 * MIRIM_TO_METERS;
    let max_x = pitch_length_m - 0.5 * MIRIM_TO_METERS;
    let min_y = 0.5 * MIRIM_TO_METERS;
    let max_y = pitch_width_m - 0.5 * MIRIM_TO_METERS;

    VectorPosition::from_components(
        target_x.clamp(min_x, max_x),
        target_y.clamp(min_y, max_y),
        0.0,
    )
}

pub fn recalibrate_defenders_for_carrier(
    defenders: &[&Player],
    defender_positions: &HashMap<Uuid, Position>,
    spatial_map: &DynamicSpatialMap,
    carrier_pos: VectorPosition,
    carrier_gravity_mult: f64,
    pitch: &Pitch,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> HashMap<Uuid, VectorPosition> {
    let mut shifted_anchors = HashMap::with_capacity(defenders.len());

    for &defender in defenders {
        let def_pos = spatial_map
            .get_position(&defender.id())
            .unwrap_or_else(VectorPosition::zero);

        let pos_role = defender_positions
            .get(&defender.id())
            .copied()
            .unwrap_or_else(|| {
                defender
                    .positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::Centerback)
            });

        let shifted = calculate_carrier_defensive_shift(
            defender,
            def_pos,
            carrier_pos,
            carrier_gravity_mult,
            pos_role,
            pitch,
            attacking_positive_x,
            attribute_keys,
        );

        shifted_anchors.insert(defender.id(), shifted);
    }

    shifted_anchors
}