use crate::spatial::decision_vector::extract_attribute_value;
use crate::team_identity::depth_from_bipolar;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, FormationSlot, Player, PositionLine};
use arlo_tactics::TeamInstructions;
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
    instructions: &TeamInstructions,
) -> (f64, f64) {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let target_position = slot.defensive_position();

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
        (work_rate * 0.5 + tactical_knowledge * 0.35 + determination * 0.15) / 20.0
    ).clamp(0.1, 1.0);

    let target_bipolar = match target_position.line() {
        PositionLine::DefenseLine => {
            Some(instructions.out_of_possession().defensive_line_height().value())
        }
        PositionLine::OffensiveLine => {
            Some(instructions.out_of_possession().engagement_line().value())
        }
        PositionLine::BackLine => {
            let dl = instructions.out_of_possession().defensive_line_height().value();
            let el = instructions.out_of_possession().engagement_line().value();
            Some((dl + el) * 0.5)
        }
        _ => None,
    };

    let x_pos = match target_bipolar {
        Some(bipolar_val) => {
            let target_depth = depth_from_bipolar(bipolar_val, pitch_length_m, attacking_positive_x);
            base_x + (target_depth - base_x) * tracking_factor
        }
        None => base_x,
    };

    let center_y = pitch_width_m * 0.5;
    let pinch_factor = (1.0 - instructions.out_of_possession().compactness().value()).clamp(0.0, 1.0);
    let y_pos = base_y + (center_y - base_y) * pinch_factor;

    (x_pos, y_pos)
}