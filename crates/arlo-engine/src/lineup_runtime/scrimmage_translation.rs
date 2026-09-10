use crate::lineup_runtime::dynamic_anchor::{compute_dynamic_anchors, AnchorComputationContext};
use crate::lineup_runtime::lineup::Lineup;
use arlo_domain::pitch::Pitch;
use arlo_math::units::Position;
use arlo_tactics::TeamInstructions;
use std::collections::HashMap;
use uuid::Uuid;

pub fn translate_formation_to_scrimmage(
    pitch: &Pitch,
    lineup: &Lineup,
    scrimmage_x_mirim: f64,
    attacking_positive_x: bool,
    instructions: &TeamInstructions,
) -> HashMap<Uuid, Position> {
    let empty_instructions = HashMap::new();
    let ctx = AnchorComputationContext {
        player_instructions_index: &empty_instructions,
        opposing_lineup: None,
        spatial_map: None,
        block_marking_roles: None,
        press_reference_pos: None,
        attribute_tables: None,
    };
    compute_dynamic_anchors(
        pitch,
        lineup,
        scrimmage_x_mirim,
        true,
        attacking_positive_x,
        &HashMap::new(),
        instructions,
        &ctx,
    )
}