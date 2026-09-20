pub fn bonus_phase_scrimmage_x(
    pitch_length_mirim: f64,
    _second_zone_depth_mirim: f64,
    attacking_positive_x: bool,
) -> f64 {
    let distance_from_goal = arlo_domain::sport_constants::FIRST_ZONE_DEPTH_MIRIM + 8.0;
    if attacking_positive_x {
        (pitch_length_mirim - distance_from_goal).max(0.0)
    } else {
        distance_from_goal.min(pitch_length_mirim)
    }
}