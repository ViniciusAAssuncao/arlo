use crate::ai::cognitive::decision_gate::evaluate_decision_gate;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, MANAGER_NOISE_DISCIPLINE_SCALE,
};
use arlo_math::stats::SkewNormalParams;
use arlo_math::Probability;
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

pub fn manager_action_probability(
    stimulus: f64,
    manager_attribute: f64,
    discipline: f64,
) -> Probability {
    evaluate_decision_gate(stimulus, manager_attribute, discipline)
}

pub fn sample_manager_action<R: Rng + ?Sized>(
    stimulus: f64,
    manager_attribute: f64,
    discipline: f64,
    rng: &mut R,
) -> bool {
    manager_action_probability(stimulus, manager_attribute, discipline).sample(rng)
}

pub fn sample_manager_decision<R: Rng + ?Sized>(
    stimulus: f64,
    manager_attribute: f64,
    discipline: f64,
    rng: &mut R,
) -> bool {
    let noise = sample_manager_decision_noise(discipline, rng);
    let noisy_stimulus = (stimulus + noise).clamp(0.0, 1.0);
    sample_manager_action(noisy_stimulus, manager_attribute, discipline, rng)
}

pub struct ManagerDecisionFactory;

impl ManagerDecisionFactory {
    pub fn action_probability(
        stimulus: f64,
        manager_attribute: f64,
        discipline: f64,
    ) -> Probability {
        manager_action_probability(stimulus, manager_attribute, discipline)
    }

    pub fn sample_action<R: Rng + ?Sized>(
        stimulus: f64,
        manager_attribute: f64,
        discipline: f64,
        rng: &mut R,
    ) -> bool {
        sample_manager_action(stimulus, manager_attribute, discipline, rng)
    }

    pub fn decide<R: Rng + ?Sized>(
        stimulus: f64,
        manager_attribute: f64,
        discipline: f64,
        rng: &mut R,
    ) -> bool {
        sample_manager_decision(stimulus, manager_attribute, discipline, rng)
    }

    pub fn noise<R: Rng + ?Sized>(discipline: f64, rng: &mut R) -> f64 {
        sample_manager_decision_noise(discipline, rng)
    }
}