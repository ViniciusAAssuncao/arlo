use rand::Rng;

pub fn sample_categorical<R: Rng + ?Sized>(weights: &[f64], rng: &mut R) -> Option<usize> {
    if weights.is_empty() {
        return None;
    }
    let total_weight: f64 = weights.iter().sum();
    if total_weight <= 0.0 || !total_weight.is_finite() {
        return None;
    }
    let sample = rng.gen_range(0.0..total_weight);
    let mut cumulative = 0.0;
    for (i, &w) in weights.iter().enumerate() {
        cumulative += w;
        if sample <= cumulative {
            return Some(i);
        }
    }
    Some(weights.len() - 1)
}