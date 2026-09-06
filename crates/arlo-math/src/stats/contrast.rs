use crate::stats::probability::Probability;

pub fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn logistic_scaled(x: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (-steepness * x).exp())
}

pub fn softmax_weights(utilities: &[f64], steepness: f64) -> Vec<f64> {
    utilities.iter().map(|&u| (steepness * u).exp()).collect()
}

pub fn bradley_terry_probability(rating_a: f64, rating_b: f64, steepness: f64) -> Probability {
    bradley_terry_with_offset(rating_a, rating_b, steepness, 0.0)
}

pub fn bradley_terry_with_offset(
    rating_a: f64,
    rating_b: f64,
    steepness: f64,
    logit_offset: f64,
) -> Probability {
    let diff = rating_a - rating_b;
    let prob = logistic(steepness * diff + logit_offset);
    Probability::new_clamped(prob)
}

pub fn bradley_terry(rating_a: f64, rating_b: f64, steepness: f64) -> Probability {
    bradley_terry_probability(rating_a, rating_b, steepness)
}