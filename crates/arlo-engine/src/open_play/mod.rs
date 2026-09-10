pub mod carrier_evaluator;
pub mod carrier_sampler;
pub mod carry_targeting;
pub mod tactical_bias;

pub use carrier_evaluator::CarrierDecisionEvaluator;
pub use carrier_sampler::{
    sample_carrier_decision, sample_carrier_decision_from_table, CarrierDecisionResult,
};
pub use carry_targeting::{compute_carry_target_lane, compute_forward_target_pos};
pub use tactical_bias::CarrierTacticalBias;