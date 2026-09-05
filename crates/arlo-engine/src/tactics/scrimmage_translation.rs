use crate::tactics::lineup::Lineup;
use arlo_domain::pitch::{project_slot, project_slot_mirrored, Pitch};
use arlo_math::units::{Position, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn translate_formation_to_scrimmage(
    pitch: &Pitch,
    lineup: &Lineup,
    scrimmage_x_mirim: f64,
    attacking_positive_x: bool,
) -> HashMap<Uuid, Position> {
    if lineup.assignments().is_empty() {
        return HashMap::new();
    }

    let projected_slots: Vec<(Uuid, f64, f64)> = lineup
        .assignments()
        .iter()
        .map(|a| {
            let coord = if attacking_positive_x {
                project_slot(pitch, a.slot())
            } else {
                project_slot_mirrored(pitch, a.slot())
            };
            (a.player().id(), coord.x_meters(), coord.y_meters())
        })
        .collect();

    let total_x: f64 = projected_slots.iter().map(|(_, x, _)| *x).sum();
    let centroid_x = total_x / (projected_slots.len() as f64);
    let target_x = scrimmage_x_mirim * MIRIM_TO_METERS;
    let delta_x = target_x - centroid_x;

    let max_x = pitch.length().value();
    let max_y = pitch.width().value();

    let mut translated = HashMap::with_capacity(projected_slots.len());
    for (id, orig_x, orig_y) in projected_slots {
        let new_x = (orig_x + delta_x).clamp(0.0, max_x);
        let new_y = orig_y.clamp(0.0, max_y);
        translated.insert(id, Position::from_components(new_x, new_y, 0.0));
    }

    translated
}
