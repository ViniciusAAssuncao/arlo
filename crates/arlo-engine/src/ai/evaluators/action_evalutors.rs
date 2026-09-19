use crate::ai::evaluators::progression_configs::{
    carry_config, long_launch_config, short_pass_config, ProgressionConfig,
};
use crate::ai::evaluators::terminal_configs::{cross_config, finish_config, TerminalScoreConfig};
use crate::attributes::profiles::AttributeProfile;
use arlo_domain::ArtrineDecisionKind;

#[derive(Clone, Copy)]
pub enum ActionKindConfig {
    Progression(ProgressionConfig),
    TerminalScore(TerminalScoreConfig),
}

#[derive(Clone, Copy)]
pub struct ActionEvaluationConfig {
    pub decision_kind: ArtrineDecisionKind,
    pub profile_fn: fn(ArtrineDecisionKind) -> &'static AttributeProfile,
    pub rating_additive_weight: f64,
    pub kind_config: ActionKindConfig,
}

pub fn get_action_config(kind: ArtrineDecisionKind) -> ActionEvaluationConfig {
    match kind {
        ArtrineDecisionKind::SelfCarry => carry_config(),
        ArtrineDecisionKind::ShortPass => short_pass_config(),
        ArtrineDecisionKind::LongLaunch => long_launch_config(),
        ArtrineDecisionKind::Cross => cross_config(),
        ArtrineDecisionKind::SelfFinish => finish_config(),
    }
}
