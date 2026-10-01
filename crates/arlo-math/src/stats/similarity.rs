pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut mag_a_sq = 0.0;
    let mut mag_b_sq = 0.0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        mag_a_sq += x * x;
        mag_b_sq += y * y;
    }
    if mag_a_sq <= 0.0 || mag_b_sq <= 0.0 {
        return 0.0;
    }
    (dot / (mag_a_sq.sqrt() * mag_b_sq.sqrt())).clamp(-1.0, 1.0)
}
