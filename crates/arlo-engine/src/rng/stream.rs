use crate::rng::seed::MatchSeed;
use serde::{Deserialize, Serialize};

pub const SPLITMIX_GAMMA: u64 = 0x9e3779b97f4a7c15;
pub const SPLITMIX_MIX_1: u64 = 0xbf58476d1ce4e5b9;
pub const SPLITMIX_MIX_2: u64 = 0x94d049bb133111eb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RngStream {
    DuelResolution,
    SpatialNoise,
    FinisherSelection,
    ProgressionDistribution,
    ArtrineDecision,
    BallSecurity,
}

impl RngStream {
    pub fn stream_id(self) -> u64 {
        match self {
            Self::DuelResolution => 1,
            Self::SpatialNoise => 2,
            Self::FinisherSelection => 3,
            Self::ProgressionDistribution => 4,
            Self::ArtrineDecision => 5,
            Self::BallSecurity => 6,
        }
    }

    pub fn all() -> [Self; 6] {
        [
            Self::DuelResolution,
            Self::SpatialNoise,
            Self::FinisherSelection,
            Self::ProgressionDistribution,
            Self::ArtrineDecision,
            Self::BallSecurity,
        ]
    }
}

pub fn split_mix_64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(SPLITMIX_MIX_1);
    z = (z ^ (z >> 27)).wrapping_mul(SPLITMIX_MIX_2);
    z ^ (z >> 31)
}

pub fn derive_sub_seed(match_seed: MatchSeed, stream: RngStream) -> u64 {
    let combined = match_seed
        .value()
        .wrapping_add(stream.stream_id().wrapping_mul(SPLITMIX_GAMMA));
    split_mix_64(combined)
}

pub fn derive_sub_seed_indexed(match_seed: MatchSeed, stream: RngStream, index: u64) -> u64 {
    let stream_offset = stream.stream_id().wrapping_mul(SPLITMIX_GAMMA);
    let index_offset = (index.wrapping_add(1)).wrapping_mul(SPLITMIX_GAMMA ^ SPLITMIX_MIX_1);
    let combined = match_seed
        .value()
        .wrapping_add(stream_offset)
        .wrapping_add(index_offset);
    split_mix_64(combined)
}