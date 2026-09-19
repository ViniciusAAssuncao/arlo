use crate::resolution::outcome_distribution::types::ProgressionDistributionParams;

pub fn short_pass_distribution_params(multiplier: f64) -> ProgressionDistributionParams {
    let mult = multiplier.max(0.1);
    ProgressionDistributionParams::new(2.8, 4.8 * mult, 0.30, 0.8, 16.0)
}