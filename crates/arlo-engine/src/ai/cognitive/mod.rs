pub mod decision_factory;
pub mod decision_gate;
pub mod decision_threshold;
pub mod risk_profile;
pub mod signal_detection;

pub use decision_factory::{
    derive_manager_decision_noise, manager_action_probability, sample_manager_action,
    sample_manager_decision, sample_manager_decision_noise, ManagerDecisionFactory,
};
pub use decision_gate::evaluate_decision_gate;
pub use decision_threshold::action_probability;
pub use risk_profile::RiskProfile;
pub use signal_detection::{detection_probability, sample_detection_outcome};