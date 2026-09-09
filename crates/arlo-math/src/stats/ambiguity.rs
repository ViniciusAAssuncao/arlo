use crate::stats::probability::Probability;

pub fn uncertainty_from_probability(p: Probability) -> Probability {
    let u = 1.0 - (2.0 * p.value() - 1.0).abs();
    Probability::new_clamped(u)
}
