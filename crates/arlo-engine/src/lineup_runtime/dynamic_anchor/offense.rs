use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, FormationSlot, Player, Position, PositionLine};
use arlo_math::units::MIRIM_TO_METERS;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_offense_attractor_coordinates(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> (f64, f64) {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let target_position = slot.offensive_position();

    let push_distance_m = match target_position.line() {
        PositionLine::DefenseLine => {
            let tactical_knowledge = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::TacticalKnowledge,
            );
            let positioning = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Positioning,
            );
            let anticipation = extract_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Anticipation,
            );

            let push_factor = (
                (tactical_knowledge * 0.45 + positioning * 0.35 + anticipation * 0.2) /
                20.0
            ).clamp(0.1, 1.0);

            let ball_progress = if attacking_positive_x {
                (scrimmage_x_m / pitch_length_m).clamp(0.0, 1.0)
            } else {
                ((pitch_length_m - scrimmage_x_m) / pitch_length_m).clamp(0.0, 1.0)
            };

            let push_mirim = match target_position {
                Position::Centerback | Position::DefensiveEnd => {
                    (10.0 + 18.0 * ball_progress) * push_factor
                }
                _ => (7.0 + 14.0 * ball_progress) * push_factor,
            };

            push_mirim * MIRIM_TO_METERS
        }
        _ => 0.0,
    };

    let x_shift = if attacking_positive_x { push_distance_m } else { -push_distance_m };

    let y_pos = match target_position {
        Position::WingOffense
        | Position::TightWing
        | Position::WideEnd
        | Position::Corridor => {
            let center_y = pitch_width_m * 0.5;
            let spread = (base_y - center_y) * 1.08;
            center_y + spread
        }
        _ => base_y,
    };

    (base_x + x_shift, y_pos)
}