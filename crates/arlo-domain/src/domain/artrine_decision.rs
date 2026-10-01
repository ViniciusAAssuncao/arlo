use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtrineDecisionKind {
    SelfCarry,
    ShortPass,
    LongLaunch,
    Cross,
    SelfFinish,
}
