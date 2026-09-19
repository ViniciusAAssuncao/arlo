use crate::resolution::outcome_distribution::types::ProgressionDistributionParams;

pub fn carry_distribution_params(multiplier: f64) -> ProgressionDistributionParams {
    let mult = multiplier.max(0.1);
    ProgressionDistributionParams::new(2.4, 3.8 * mult, 0.35, 0.5, 18.0)
}