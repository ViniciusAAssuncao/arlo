use arlo_domain::pitch::{channel_y_meters, Pitch};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::RouteAssignment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RouteWaypoints {
    pub stem_point: VectorPosition,
    pub break_point: VectorPosition,
    pub total_length_meters: f64,
}

impl RouteWaypoints {
    pub fn new(
        stem_point: VectorPosition,
        break_point: VectorPosition,
        total_length_meters: f64,
    ) -> Self {
        Self {
            stem_point,
            break_point,
            total_length_meters,
        }
    }

    pub fn stem_point(&self) -> VectorPosition {
        self.stem_point
    }

    pub fn break_point(&self) -> VectorPosition {
        self.break_point
    }

    pub fn total_length_meters(&self) -> f64 {
        self.total_length_meters
    }
}

pub fn resolve_route_waypoints(
    anchor_pos: VectorPosition,
    route: &RouteAssignment,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> RouteWaypoints {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();

    let x0 = anchor_pos.raw().0;
    let y0 = anchor_pos.raw().1;

    let remaining_distance = if attacking_positive_x {
        (pitch_length_m - x0).max(0.0)
    } else {
        x0.max(0.0)
    };

    let total_depth = route.depth_ratio() * remaining_distance;
    let break_depth = total_depth * route.break_ratio();

    let stem_x = if attacking_positive_x {
        x0 + break_depth
    } else {
        x0 - break_depth
    };
    let stem_y = y0;

    let target_y = channel_y_meters(route.target_channel(), pitch).value();
    let break_x = if attacking_positive_x {
        x0 + total_depth
    } else {
        x0 - total_depth
    };
    let break_y = target_y;

    let clamped_stem_x = stem_x.clamp(0.0, pitch_length_m);
    let clamped_stem_y = stem_y.clamp(0.0, pitch_width_m);
    let clamped_break_x = break_x.clamp(0.0, pitch_length_m);
    let clamped_break_y = break_y.clamp(0.0, pitch_width_m);

    let stem_point = VectorPosition::from_components(clamped_stem_x, clamped_stem_y, 0.0);
    let break_point = VectorPosition::from_components(clamped_break_x, clamped_break_y, 0.0);

    let d1 = (stem_point.raw() - anchor_pos.raw()).magnitude();
    let d2 = (break_point.raw() - stem_point.raw()).magnitude();
    let total_length_meters = d1 + d2;

    RouteWaypoints::new(stem_point, break_point, total_length_meters)
}
