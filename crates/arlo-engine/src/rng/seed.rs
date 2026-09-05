use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct MatchSeed(u64);

impl MatchSeed {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for MatchSeed {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MatchSeed> for u64 {
    fn from(seed: MatchSeed) -> Self {
        seed.0
    }
}
