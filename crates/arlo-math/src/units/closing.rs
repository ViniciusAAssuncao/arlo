use crate::units::duration::Duration;
use crate::units::length::Length;
use crate::units::speed::Speed;

pub fn time_to_close(distance: Length, speed_a: Speed, speed_b: Speed) -> Option<Duration> {
    let combined_speed = speed_a + speed_b;
    if combined_speed.value() <= 0.0 {
        None
    } else {
        Some(distance / combined_speed)
    }
}
