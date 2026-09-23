use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RngStream {
    Core,
    Contest,
    Decision,
    Physical,
    Injury,
    RefereeAssignment,
    ChallengeResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct MatchSeed(pub u64);

impl MatchSeed {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for MatchSeed {
    fn from(seed: u64) -> Self {
        Self(seed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RngProvider {
    seed: MatchSeed,
}

impl RngProvider {
    pub fn new(seed: MatchSeed) -> Self {
        Self { seed }
    }

    pub fn seed(&self) -> MatchSeed {
        self.seed
    }

    pub fn rng_for(&self, _stream: RngStream) -> StdRng {
        StdRng::seed_from_u64(self.seed.0)
    }

    pub fn indexed_rng_for(&self, _stream: RngStream, index: u64) -> StdRng {
        StdRng::seed_from_u64(self.seed.0.wrapping_add(index))
    }
}