use crate::resolution::outcome_distribution::types::ProgressionDistributionParams;

pub fn cross_distribution_params(multiplier: f64) -> ProgressionDistributionParams {
    let mult = multiplier.max(0.1);
    ProgressionDistributionParams::new(2.5, 4.5 * mult, 0.25, 0.8, 15.0)
}