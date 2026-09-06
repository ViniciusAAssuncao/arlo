use crate::tactics::dynamic_anchor::compute_dynamic_anchors;
use crate::tactics::lineup::Lineup;
use arlo_domain::pitch::Pitch;
use arlo_math::units::Position;
use std::collections::HashMap;
use uuid::Uuid;

pub fn translate_formation_to_scrimmage(
    pitch: &Pitch,
    lineup: &Lineup,
    scrimmage_x_mirim: f64,
    attacking_positive_x: bool,
) -> HashMap<Uuid, Position> {
    compute_dynamic_anchors(
        pitch,
        lineup,
        scrimmage_x_mirim,
        true,
        attacking_positive_x,
        &HashMap::new(),
    )
}
