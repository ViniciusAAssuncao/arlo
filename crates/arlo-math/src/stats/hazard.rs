use crate::Probability;

pub fn hazard_rate_to_probability(hazard_rate: f64, dt_seconds: f64) -> f64 {
    if hazard_rate <= 0.0 || dt_seconds <= 0.0 {
        return 0.0;
    }
    let exponent = -hazard_rate * dt_seconds;
    (1.0 - exponent.exp()).clamp(0.0, 1.0)
}

pub fn continuous_hazard_to_discrete_probability(hazard_rate: f64, dt_seconds: f64) -> Probability {
    Probability::new_clamped(hazard_rate_to_probability(hazard_rate, dt_seconds))
}
