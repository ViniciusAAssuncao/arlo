use arlo_math::units::{Duration, Position, Velocity};

pub fn advance_position(current: Position, velocity: Velocity, dt: Duration) -> Position {
    current + (velocity * dt)
}

pub fn calculate_displacement(velocity: Velocity, dt: Duration) -> Position {
    velocity * dt
}
