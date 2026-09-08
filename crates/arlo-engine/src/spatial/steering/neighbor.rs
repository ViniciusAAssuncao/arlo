use arlo_math::units::{Position, Velocity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpatialNeighbor {
    pub id: Uuid,
    pub position: Position,
    pub velocity: Velocity,
    pub is_teammate: bool,
    pub physical_radius: f64,
}

impl SpatialNeighbor {
    pub fn new(
        id: Uuid,
        position: Position,
        velocity: Velocity,
        is_teammate: bool,
        physical_radius: f64,
    ) -> Self {
        Self {
            id,
            position,
            velocity,
            is_teammate,
            physical_radius,
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self {
            id: Uuid::nil(),
            position,
            velocity: Velocity::zero(),
            is_teammate: false,
            physical_radius: 0.55,
        }
    }
}