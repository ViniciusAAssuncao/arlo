use crate::lineup_runtime::dynamic_anchor::calculate_defense_attractor_coordinates;
use crate::lineup_runtime::dynamic_anchor::offense::calculate_offense_attractor_coordinates_for_position;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::{project_slot, project_slot_mirrored, Pitch};
use arlo_domain::{AttributeKey, FormationSlot, Player, Position};
use arlo_math::geometry::VoronoiSite;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use arlo_tactics::TeamInstructions;
use std::collections::HashMap;
use uuid::Uuid;

pub fn decoy_attractor(
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
        calculate_offense_attractor_coordinates_for_position(
            pitch,
            player,
            Position::Artrine,
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

pub fn phantom_voronoi_site<F>(
    false_artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let pos = spatial_map
        .get_position(&false_artrine.id())
        .unwrap_or_else(VectorPosition::zero);
    let fatigue = fatigue_for(&false_artrine.id());
    let speed = calculate_effective_player_speed(false_artrine, attribute_keys, &fatigue).value();
    let bluff = extract_attribute_value(
        false_artrine,
        attribute_keys,
        AttributeKey::FalseArtrineBluff,
    );
    let reaction_time = ((20.0 - bluff) * 0.015).max(0.05);

    VoronoiSite::new(pos.raw().0, pos.raw().1, speed, reaction_time, team_id)
}