use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, MANAGER_NOISE_DISCIPLINE_SCALE,
};
use arlo_math::stats::SkewNormalParams;
use rand::Rng;

pub fn derive_manager_decision_noise(discipline: f64) -> SkewNormalParams {
    let disc = discipline.clamp(0.0, 20.0);
    let undisciplined_factor = ((20.0 - disc) / ATTRIBUTE_SATURATION_THRESHOLD).clamp(0.0, 1.0);
    let scale = MANAGER_NOISE_DISCIPLINE_SCALE * undisciplined_factor.powf(1.5);
    let location = 0.0;
    let shape = 0.0;
    SkewNormalParams::new(location, scale, shape)
}

pub fn sample_manager_decision_noise<R: Rng + ?Sized>(discipline: f64, rng: &mut R) -> f64 {
    derive_manager_decision_noise(discipline).sample(rng)
}