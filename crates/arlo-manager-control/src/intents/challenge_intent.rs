use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ChallengeIntent;

impl ChallengeIntent {
    pub fn new() -> Self {
        Self
    }
}
