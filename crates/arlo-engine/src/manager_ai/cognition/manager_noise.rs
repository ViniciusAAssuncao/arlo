use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, MANAGER_NOISE_DISCIPLINE_SCALE,
};
use arlo_math::stats::SkewNormalParams;

pub fn derive_manager_decision_noise(discipline: f64) -> SkewNormalParams {
    let disc = discipline.clamp(0.0, 20.0);
    let scale =
        MANAGER_NOISE_DISCIPLINE_SCALE * (1.0 + (20.0 - disc) / ATTRIBUTE_SATURATION_THRESHOLD);
    let location = 0.0;
    let shape = 0.0;
    SkewNormalParams::new(location, scale, shape)
}
