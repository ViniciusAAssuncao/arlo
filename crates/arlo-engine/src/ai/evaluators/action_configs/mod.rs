pub mod carry;
pub mod cross;
pub mod finish;
pub mod long_launch;
pub mod short_pass;
pub mod types;

pub use carry::carry_config;
pub use cross::cross_config;
pub use finish::finish_config;
pub use long_launch::long_launch_config;
pub use short_pass::short_pass_config;
pub use types::{
    ActionEvaluationConfig, ActionKindConfig, CarryUtilityEvaluator, CrossUtilityEvaluator,
    LongLaunchUtilityEvaluator, ProgressionConfig, SelfFinishUtilityEvaluator,
    ShortPassUtilityEvaluator, TerminalScoreConfig,
};

use arlo_domain::ArtrineDecisionKind;

pub fn get_action_config(kind: ArtrineDecisionKind) -> ActionEvaluationConfig {
    match kind {
        ArtrineDecisionKind::SelfCarry => carry_config(),
        ArtrineDecisionKind::ShortPass => short_pass_config(),
        ArtrineDecisionKind::LongLaunch => long_launch_config(),
        ArtrineDecisionKind::Cross => cross_config(),
        ArtrineDecisionKind::SelfFinish => finish_config(),
    }
}