pub fn saturating_stimulus(magnitude: f64) -> f64 {
    if magnitude <= 0.0 {
        0.0
    } else {
        magnitude / (magnitude + 1.0)
    }
}
