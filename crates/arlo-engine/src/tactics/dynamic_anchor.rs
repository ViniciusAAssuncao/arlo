use crate::spatial::decision_vector::extract_attribute_value;
use crate::tactics::lineup::Lineup;
use arlo_domain::pitch::{ project_slot, project_slot_mirrored, Pitch };
use arlo_domain::{ AttributeKey, FormationSlot, Player, Position, PositionLine };
use arlo_math::units::{ Position as VectorPosition, MIRIM_TO_METERS };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DynamicAnchorManager;

pub fn calculate_player_dynamic_attractor(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    scrimmage_x_mirim: f64,
    is_offense: bool,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>
) -> VectorPosition {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let scrimmage_x_m = (scrimmage_x_mirim * MIRIM_TO_METERS).clamp(0.0, pitch_length_m);

    let base_coord = if attacking_positive_x {
        project_slot(pitch, slot)
    } else {
        project_slot_mirrored(pitch, slot)
    };

    let base_x = base_coord.x_meters();
    let base_y = base_coord.y_meters();

    let target_position = if is_offense {
        slot.offensive_position()
    } else {
        slot.defensive_position()
    };

    let (adjusted_x, adjusted_y) = if is_offense {
        let push_distance_m = match target_position.line() {
            PositionLine::DefenseLine => {
                let tactical_knowledge = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::TacticalKnowledge
                );
                let positioning = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::Positioning
                );
                let anticipation = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::Anticipation
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
            | Position::WingOffense
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
    } else {
        let retreat_distance_m = match target_position.line() {
            PositionLine::OffensiveLine => {
                let work_rate = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::WorkRate
                );
                let tactical_knowledge = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::TacticalKnowledge
                );
                let determination = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::Determination
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
                    AttributeKey::WorkRate
                );
                let tactical_knowledge = extract_attribute_value(
                    player,
                    attribute_keys,
                    AttributeKey::TacticalKnowledge
                );
                let tracking_factor = ((work_rate * 0.55 + tactical_knowledge * 0.45) / 20.0).clamp(
                    0.1,
                    1.0
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
    };

    let min_x = 0.5 * MIRIM_TO_METERS;
    let max_x = pitch_length_m - 0.5 * MIRIM_TO_METERS;
    let min_y = 0.5 * MIRIM_TO_METERS;
    let max_y = pitch_width_m - 0.5 * MIRIM_TO_METERS;

    let clamped_x = adjusted_x.clamp(min_x, max_x);
    let clamped_y = adjusted_y.clamp(min_y, max_y);

    VectorPosition::from_components(clamped_x, clamped_y, 0.0)
}

pub fn compute_dynamic_anchors(
    pitch: &Pitch,
    lineup: &Lineup,
    scrimmage_x_mirim: f64,
    is_offense: bool,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>
) -> HashMap<Uuid, VectorPosition> {
    if lineup.assignments().is_empty() {
        return HashMap::new();
    }

    let pitch_length_m = pitch.length().value();
    let scrimmage_x_m = (scrimmage_x_mirim * MIRIM_TO_METERS).clamp(0.0, pitch_length_m);

    let mut raw_attractors: Vec<(Uuid, VectorPosition)> = Vec::with_capacity(lineup.len());
    let mut total_x = 0.0;

    for assignment in lineup.assignments() {
        let player = assignment.player();
        let slot = assignment.slot();
        let attractor = calculate_player_dynamic_attractor(
            pitch,
            player,
            slot,
            scrimmage_x_mirim,
            is_offense,
            attacking_positive_x,
            attribute_keys
        );
        total_x += attractor.raw().0;
        raw_attractors.push((player.id(), attractor));
    }

    let centroid_x = total_x / (raw_attractors.len() as f64);
    let target_centroid_x = if is_offense {
        if attacking_positive_x {
            (scrimmage_x_m - 3.0 * MIRIM_TO_METERS).clamp(0.0, pitch_length_m)
        } else {
            (scrimmage_x_m + 3.0 * MIRIM_TO_METERS).clamp(0.0, pitch_length_m)
        }
    } else {
        if attacking_positive_x {
            (scrimmage_x_m - 8.0 * MIRIM_TO_METERS).clamp(0.0, pitch_length_m)
        } else {
            (scrimmage_x_m + 8.0 * MIRIM_TO_METERS).clamp(0.0, pitch_length_m)
        }
    };

    let delta_x = (target_centroid_x - centroid_x) * 0.45;

    let min_x = 0.5 * MIRIM_TO_METERS;
    let max_x = pitch_length_m - 0.5 * MIRIM_TO_METERS;
    let min_y = 0.5 * MIRIM_TO_METERS;
    let max_y = pitch.width().value() - 0.5 * MIRIM_TO_METERS;

    let mut result = HashMap::with_capacity(raw_attractors.len());
    for (id, pos) in raw_attractors {
        let shifted_x = (pos.raw().0 + delta_x).clamp(min_x, max_x);
        let clamped_y = pos.raw().1.clamp(min_y, max_y);
        result.insert(id, VectorPosition::from_components(shifted_x, clamped_y, 0.0));
    }

    result
}

pub fn translate_dynamic_formation_to_scrimmage(
    pitch: &Pitch,
    lineup: &Lineup,
    scrimmage_x_mirim: f64,
    is_offense: bool,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>
) -> HashMap<Uuid, VectorPosition> {
    compute_dynamic_anchors(
        pitch,
        lineup,
        scrimmage_x_mirim,
        is_offense,
        attacking_positive_x,
        attribute_keys
    )
}

impl DynamicAnchorManager {
    pub fn calculate_attractor(
        pitch: &Pitch,
        player: &Player,
        slot: &FormationSlot,
        scrimmage_x_mirim: f64,
        is_offense: bool,
        attacking_positive_x: bool,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) -> VectorPosition {
        calculate_player_dynamic_attractor(
            pitch,
            player,
            slot,
            scrimmage_x_mirim,
            is_offense,
            attacking_positive_x,
            attribute_keys
        )
    }

    pub fn compute_team_anchors(
        pitch: &Pitch,
        lineup: &Lineup,
        scrimmage_x_mirim: f64,
        is_offense: bool,
        attacking_positive_x: bool,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) -> HashMap<Uuid, VectorPosition> {
        compute_dynamic_anchors(
            pitch,
            lineup,
            scrimmage_x_mirim,
            is_offense,
            attacking_positive_x,
            attribute_keys
        )
    }
}
