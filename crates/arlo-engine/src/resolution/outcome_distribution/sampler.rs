use crate::resolution::outcome_distribution::types::ActionProgressionKind;
use arlo_domain::sport_constants::PITCH_LENGTH_MIRIM_MAX;
use rand::Rng;
use rand_distr::{Distribution, Gamma};

pub fn sample_action_progression<R: Rng + ?Sized>(
    kind: ActionProgressionKind,
    margin: f64,
    multiplier: f64,
    rng: &mut R,
) -> f64 {
    let (shape, base_mean, advantage_factor, min_mean, max_mean) =
        kind.distribution_params(multiplier);
    let mean = (base_mean + margin * advantage_factor).clamp(min_mean, max_mean);
    let scale = mean / shape;

    if let Ok(gamma) = Gamma::new(shape, scale) {
        gamma.sample(rng).clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
    } else {
        mean.clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
    }
}