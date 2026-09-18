use crate::ai::cognitive::decision_threshold::action_probability;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_math::stats::Probability;
use rand::Rng;

pub fn manager_action_probability(stimulus: f64, manager_attribute: f64) -> Probability {
    action_probability(
        stimulus,
        manager_attribute,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    )
}

pub fn sample_manager_action<R: Rng + ?Sized>(
    stimulus: f64,
    manager_attribute: f64,
    rng: &mut R,
) -> bool {
    manager_action_probability(stimulus, manager_attribute).sample(rng)
}

pub struct ManagerDecisionFactory;

impl ManagerDecisionFactory {
    pub fn action_probability(stimulus: f64, manager_attribute: f64) -> Probability {
        manager_action_probability(stimulus, manager_attribute)
    }

    pub fn sample_action<R: Rng + ?Sized>(
        stimulus: f64,
        manager_attribute: f64,
        rng: &mut R,
    ) -> bool {
        sample_manager_action(stimulus, manager_attribute, rng)
    }
}