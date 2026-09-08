use crate::artrine::constants::CARRY_FORWARD_TARGET_OFFSET_MIRIM;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM;
use arlo_domain::{Player, SlotRole};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn filter_blocker_helpers<'a>(
    offense_helpers: &'a [&'a Player],
    offense_role_index: &HashMap<Uuid, SlotRole>,
) -> Vec<&'a Player> {
    let blockers: Vec<&Player> = offense_helpers
        .iter()
        .copied()
        .filter(|p| offense_role_index.get(&p.id()) == Some(&SlotRole::Blocker))
        .collect();

    if blockers.is_empty() {
        offense_helpers.to_vec()
    } else {
        blockers
    }
}

pub fn compute_carry_target_lane(start_pos: VectorPosition, pitch: &Pitch) -> f64 {
    let pitch_width_m = pitch.width().value();
    let center_y_m = pitch_width_m / 2.0;
    let left_y_m = center_y_m - DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM * MIRIM_TO_METERS;
    let right_y_m = center_y_m + DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM * MIRIM_TO_METERS;

    [left_y_m, center_y_m, right_y_m]
        .into_iter()
        .min_by(|&a, &b| {
            let da = (a - start_pos.raw().1).abs();
            let db = (b - start_pos.raw().1).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(center_y_m)
}

pub fn compute_forward_target_pos(
    start_pos: VectorPosition,
    target_channel_y_m: f64,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> VectorPosition {
    let forward_x_mirim = if attacking_positive_x {
        (start_pos.raw().0 / MIRIM_TO_METERS + CARRY_FORWARD_TARGET_OFFSET_MIRIM)
            .min(pitch.length_mirim())
    } else {
        (start_pos.raw().0 / MIRIM_TO_METERS - CARRY_FORWARD_TARGET_OFFSET_MIRIM).max(0.0)
    };

    VectorPosition::from_components(forward_x_mirim * MIRIM_TO_METERS, target_channel_y_m, 0.0)
}
