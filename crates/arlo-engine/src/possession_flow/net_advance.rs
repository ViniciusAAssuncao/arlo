pub fn calculate_net_advance(
    start_x_mirim: f64,
    end_x_mirim: f64,
    attacking_positive_x: bool,
    pitch_length_mirim: f64,
) -> f64 {
    let max_len = pitch_length_mirim.max(0.0);
    let clamped_start = start_x_mirim.clamp(0.0, max_len);
    let clamped_end = end_x_mirim.clamp(0.0, max_len);

    if attacking_positive_x {
        clamped_end - clamped_start
    } else {
        clamped_start - clamped_end
    }
}