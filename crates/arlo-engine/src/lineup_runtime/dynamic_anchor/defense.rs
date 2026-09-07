use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, FormationSlot, Player, Position, PositionLine};
use arlo_math::units::MIRIM_TO_METERS;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_defense_attractor_coordinates(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    _scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> (f64, f64) {
    let pitch_width_m = pitch.width().value();
    let target_position = slot.defensive_position();

    let retreat_distance_m = match target_position.line() {
        PositionLine::OffensiveLine => {
            let work_rate = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::WorkRate,
            );
            let tactical_knowledge = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::TacticalKnowledge,
            );
            let determination = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Determination,
            );

            let tracking_factor = (
                (work_rate * 0.5 + tactical_knowledge * 0.35 + determination * 0.15) /
                20.0
            ).clamp(0.1, 1.0);

            let retreat_mirim = match target_position {
                Position::CenterOffense | Position::WingOffense => {
                    14.0 + 10.0 * tracking_factor
                }
                _ => 10.0 + 8.0 * tracking_factor,
            };

            retreat_mirim * MIRIM_TO_METERS
        }
        PositionLine::BackLine => {
            let work_rate = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::WorkRate,
            );
            let tactical_knowledge = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::TacticalKnowledge,
            );
            let tracking_factor = ((work_rate * 0.55 + tactical_knowledge * 0.45) / 20.0).clamp(
                0.1,
                1.0,
            );
            (6.0 + 6.0 * tracking_factor) * MIRIM_TO_METERS
        }
        _ => 0.0,
    };

    let x_shift = if attacking_positive_x { -retreat_distance_m } else { retreat_distance_m };

    let center_y = pitch_width_m * 0.5;
    let pinch_factor = match target_position.line() {
        PositionLine::OffensiveLine => 0.18,
        PositionLine::BackLine => 0.12,
        _ => 0.06,
    };
    let y_pos = base_y + (center_y - base_y) * pinch_factor;

    (base_x + x_shift, y_pos)
}