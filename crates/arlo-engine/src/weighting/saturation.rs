pub fn apply_saturation(value: f64, threshold: f64, multiplier: f64) -> f64 {
    if value > threshold {
        threshold + (value - threshold) * multiplier
    } else {
        value
    }
}
