pub fn calculate_normalized_proximity(
    scrimmage_x_mirim: f64,
    pitch_length_mirim: f64,
    attacking_positive_x: bool,
) -> f64 {
    if pitch_length_mirim <= 0.0 {
        return 0.0;
    }
    if attacking_positive_x {
        (scrimmage_x_mirim / pitch_length_mirim).clamp(0.0, 1.0)
    } else {
        ((pitch_length_mirim - scrimmage_x_mirim) / pitch_length_mirim).clamp(0.0, 1.0)
    }
}
