use arlo_domain::ArtrineDecisionKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionProgressionKind {
    Carry,
    ShortPass,
    LongLaunch,
    Cross,
}

impl ActionProgressionKind {
    pub fn from_decision_kind(kind: ArtrineDecisionKind) -> Option<Self> {
        match kind {
            ArtrineDecisionKind::SelfCarry => Some(Self::Carry),
            ArtrineDecisionKind::ShortPass => Some(Self::ShortPass),
            ArtrineDecisionKind::LongLaunch => Some(Self::LongLaunch),
            ArtrineDecisionKind::Cross => Some(Self::Cross),
            ArtrineDecisionKind::SelfFinish => None,
        }
    }

    pub fn distribution_params(self, multiplier: f64) -> (f64, f64, f64, f64, f64) {
        let params = match self {
            Self::Carry => crate::resolution::outcome_distribution::carry::carry_distribution_params(multiplier),
            Self::ShortPass => crate::resolution::outcome_distribution::short_pass::short_pass_distribution_params(multiplier),
            Self::LongLaunch => crate::resolution::outcome_distribution::long_launch::long_launch_distribution_params(multiplier),
            Self::Cross => crate::resolution::outcome_distribution::cross::cross_distribution_params(multiplier),
        };
        (
            params.shape,
            params.base_mean,
            params.advantage_factor,
            params.min_mean,
            params.max_mean,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressionDistributionParams {
    pub shape: f64,
    pub base_mean: f64,
    pub advantage_factor: f64,
    pub min_mean: f64,
    pub max_mean: f64,
}

impl ProgressionDistributionParams {
    pub const fn new(
        shape: f64,
        base_mean: f64,
        advantage_factor: f64,
        min_mean: f64,
        max_mean: f64,
    ) -> Self {
        Self {
            shape,
            base_mean,
            advantage_factor,
            min_mean,
            max_mean,
        }
    }
}