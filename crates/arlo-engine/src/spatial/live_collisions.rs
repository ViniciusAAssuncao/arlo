use arlo_math::units::{Position, Velocity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LiveCollision {
    pub carrier_id: Uuid,
    pub defender_id: Uuid,
    pub carrier_position: Position,
    pub defender_position: Position,
    pub distance_meters: f64,
    pub overlap_meters: f64,
    pub contact_severity: f64,
}

impl LiveCollision {
    pub fn new(
        carrier_id: Uuid,
        defender_id: Uuid,
        carrier_position: Position,
        defender_position: Position,
        distance_meters: f64,
        overlap_meters: f64,
        contact_severity: f64,
    ) -> Self {
        Self {
            carrier_id,
            defender_id,
            carrier_position,
            defender_position,
            distance_meters,
            overlap_meters,
            contact_severity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CollisionResolution {
    Continue { velocity_mitigation: f64 },
    Halt {
        turnover_team: Option<Uuid>,
        recovering_player: Option<Uuid>,
    },
}

pub fn check_collision(
    carrier_id: Uuid,
    carrier_pos: Position,
    carrier_vel: Velocity,
    carrier_radius: f64,
    defender_id: Uuid,
    defender_pos: Position,
    defender_vel: Velocity,
    defender_radius: f64,
) -> Option<LiveCollision> {
    let p_diff = carrier_pos.raw() - defender_pos.raw();
    let distance = (p_diff.0 * p_diff.0 + p_diff.1 * p_diff.1 + p_diff.2 * p_diff.2).sqrt();
    let combined_radii = carrier_radius + defender_radius;

    if distance <= combined_radii {
        let overlap = (combined_radii - distance).max(0.0);
        let normal = if distance > 1e-6 {
            p_diff / distance
        } else {
            p_diff
        };

        let v_rel = carrier_vel.raw() - defender_vel.raw();
        let closing_speed = -(v_rel.0 * normal.0 + v_rel.1 * normal.1 + v_rel.2 * normal.2).min(0.0);
        let severity = overlap * (1.0 + closing_speed * 0.35);

        Some(LiveCollision::new(
            carrier_id,
            defender_id,
            carrier_pos,
            defender_pos,
            distance,
            overlap,
            severity,
        ))
    } else {
        None
    }
}

pub fn is_severe_contact(collision: &LiveCollision) -> bool {
    collision.overlap_meters >= 0.05 || collision.contact_severity >= 0.08
}