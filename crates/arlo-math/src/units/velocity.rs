use crate::units::collision::compute_swept_sphere_intersection;
use crate::units::duration::Duration;
use crate::units::length::Length;
use crate::units::position::Position;
use crate::units::speed::Speed;
use crate::units::vector_macro::define_vector_quantity;

define_vector_quantity!(Velocity, Speed);

impl Velocity {
    pub fn swept_sphere_intersection(
        &self,
        self_pos: Position,
        other_pos: Position,
        other_vel: Velocity,
        radius: Length,
    ) -> Option<Duration> {
        compute_swept_sphere_intersection(self_pos, *self, other_pos, other_vel, radius)
    }
}