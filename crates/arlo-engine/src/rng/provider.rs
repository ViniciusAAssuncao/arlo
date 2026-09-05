use crate::rng::seed::MatchSeed;
use crate::rng::stream::{derive_sub_seed, derive_sub_seed_indexed, RngStream};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RngProvider {
    seed: MatchSeed,
}

impl RngProvider {
    pub fn new(seed: MatchSeed) -> Self {
        Self { seed }
    }

    pub fn from_u64(seed: u64) -> Self {
        Self {
            seed: MatchSeed::new(seed),
        }
    }

    pub fn seed(&self) -> MatchSeed {
        self.seed
    }

    pub fn rng_for(&self, stream: RngStream) -> ChaCha8Rng {
        let sub_seed = derive_sub_seed(self.seed, stream);
        ChaCha8Rng::seed_from_u64(sub_seed)
    }

    pub fn stream_rng(&self, stream: RngStream) -> ChaCha8Rng {
        self.rng_for(stream)
    }

    pub fn indexed_rng_for(&self, stream: RngStream, index: u64) -> ChaCha8Rng {
        let sub_seed = derive_sub_seed_indexed(self.seed, stream, index);
        ChaCha8Rng::seed_from_u64(sub_seed)
    }

    pub fn duel_resolution_rng(&self) -> ChaCha8Rng {
        self.rng_for(RngStream::DuelResolution)
    }

    pub fn spatial_noise_rng(&self) -> ChaCha8Rng {
        self.rng_for(RngStream::SpatialNoise)
    }

    pub fn finisher_selection_rng(&self) -> ChaCha8Rng {
        self.rng_for(RngStream::FinisherSelection)
    }

    pub fn progression_distribution_rng(&self) -> ChaCha8Rng {
        self.rng_for(RngStream::ProgressionDistribution)
    }
}
