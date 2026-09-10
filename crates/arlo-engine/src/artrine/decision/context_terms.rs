use arlo_domain::Pitch;
use arlo_math::units::Position;

pub fn calculate_normalized_proximity(
    artrine_pos: Position,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let total_len = pitch.length().value();
    if total_len <= 0.0 {
        return 0.0;
    }
    let x = artrine_pos.raw().0;
    if attacking_positive_x {
        (x / total_len).max(0.0).min(1.0)
    } else {
        ((total_len - x) / total_len).max(0.0).min(1.0)
    }
}
