use crate::resolution::outcome_distribution::types::ProgressionDistributionParams;

pub fn long_launch_distribution_params(multiplier: f64) -> ProgressionDistributionParams {
    let mult = multiplier.max(0.1);
    ProgressionDistributionParams::new(2.2, 10.5 * mult, 0.55, 2.0, 32.0)
}