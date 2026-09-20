use arlo_math::units::Duration;

pub fn ball_flight_duration(distance_mirim: f64) -> Duration {
    let seconds = if distance_mirim > 15.0 {
        3.0
    } else if distance_mirim > 6.0 {
        2.0
    } else {
        1.0
    };
    Duration::new(seconds)
}
