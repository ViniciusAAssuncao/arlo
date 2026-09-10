use crate::artrine::constants::CARRY_FORWARD_TARGET_OFFSET_MIRIM;
use arlo_domain::pitch::{channel_y_meters, Pitch};
use arlo_domain::ArtroPlacement;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};

pub fn compute_carry_target_lane(start_pos: VectorPosition, pitch: &Pitch) -> f64 {
    let center_y_m = channel_y_meters(ArtroPlacement::Central, pitch).value();
    let lanes = [
        ArtroPlacement::LeftLateral,
        ArtroPlacement::Central,
        ArtroPlacement::RightLateral,
    ]
    .map(|p| channel_y_meters(p, pitch).value());

    lanes
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