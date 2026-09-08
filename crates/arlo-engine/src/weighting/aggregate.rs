use crate::weighting::saturation::apply_saturation;

pub fn calculate_weighted_average(items: &[(f64, f64)]) -> Option<f64> {
    let total_weight: f64 = items.iter().map(|(_, w)| *w).sum();
    if total_weight <= 0.0 {
        return None;
    }

    let accumulated: f64 = items.iter().map(|(v, w)| v * w).sum();
    Some(accumulated / total_weight)
}

pub fn calculate_weighted_saturated_average(
    items: &[(f64, f64)],
    threshold: f64,
    multiplier: f64,
) -> Option<f64> {
    let total_weight: f64 = items.iter().map(|(_, w)| *w).sum();
    if total_weight <= 0.0 {
        return None;
    }

    let accumulated: f64 = items
        .iter()
        .map(|&(v, w)| apply_saturation(v, threshold, multiplier) * w)
        .sum();
    Some(accumulated / total_weight)
}
