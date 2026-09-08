use crate::units::duration::Duration;
use crate::units::length::Length;
use crate::units::position::Position;
use crate::units::velocity::Velocity;

pub fn compute_swept_sphere_intersection(
    pos_a: Position,
    vel_a: Velocity,
    pos_b: Position,
    vel_b: Velocity,
    radius: Length,
) -> Option<Duration> {
    let r = radius.value();
    if r < 0.0 {
        return None;
    }

    let dp = pos_a.raw() - pos_b.raw();
    let dv = vel_a.raw() - vel_b.raw();

    let c = dp.dot(&dp) - r * r;
    if c <= 0.0 {
        return Some(Duration::new(0.0));
    }

    let a = dv.dot(&dv);
    if a <= 1e-12 {
        return None;
    }

    let b = 2.0 * dp.dot(&dv);

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = (-b - sqrt_d) / (2.0 * a);
    let t2 = (-b + sqrt_d) / (2.0 * a);

    if t1 >= 0.0 {
        Some(Duration::new(t1))
    } else if t2 >= 0.0 {
        Some(Duration::new(0.0))
    } else {
        None
    }
}
