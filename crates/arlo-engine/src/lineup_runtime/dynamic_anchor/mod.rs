pub mod defense;
pub mod offense;

pub use defense::*;
pub use offense::*;

use crate::lineup_runtime::lineup::Lineup;
use arlo_domain::pitch::{project_slot, project_slot_mirrored, Pitch};
use arlo_domain::{AttributeKey, FormationSlot, Player};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use arlo_tactics::TeamInstructions;
use serde::{Deserialize, Serialize};
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
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
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

    let (adjusted_x, adjusted_y) = if is_offense {
        calculate_offense_attractor_coordinates(
            pitch,
            player,
            slot,
            scrimmage_x_m,
            base_x,
            base_y,
            attacking_positive_x,
            attribute_keys,
            instructions,
        )
    } else {
        calculate_defense_attractor_coordinates(
            pitch,
            player,
            slot,
            scrimmage_x_m,
            base_x,
            base_y,
            attacking_positive_x,
            attribute_keys,
            instructions,
        )
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
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
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
            attribute_keys,
            instructions,
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
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
) -> HashMap<Uuid, VectorPosition> {
    compute_dynamic_anchors(
        pitch,
        lineup,
        scrimmage_x_mirim,
        is_offense,
        attacking_positive_x,
        attribute_keys,
        instructions,
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
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        instructions: &TeamInstructions,
    ) -> VectorPosition {
        calculate_player_dynamic_attractor(
            pitch,
            player,
            slot,
            scrimmage_x_mirim,
            is_offense,
            attacking_positive_x,
            attribute_keys,
            instructions,
        )
    }

    pub fn compute_team_anchors(
        pitch: &Pitch,
        lineup: &Lineup,
        scrimmage_x_mirim: f64,
        is_offense: bool,
        attacking_positive_x: bool,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        instructions: &TeamInstructions,
    ) -> HashMap<Uuid, VectorPosition> {
        compute_dynamic_anchors(
            pitch,
            lineup,
            scrimmage_x_mirim,
            is_offense,
            attacking_positive_x,
            attribute_keys,
            instructions,
        )
    }
}