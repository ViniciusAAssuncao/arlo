use crate::rng::seed::MatchSeed;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SPLITMIX_GAMMA: u64 = 0x9e3779b97f4a7c15;
pub const SPLITMIX_MIX_1: u64 = 0xbf58476d1ce4e5b9;
pub const SPLITMIX_MIX_2: u64 = 0x94d049bb133111eb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RngStream {
    DuelResolution,
    ChallengeResolution,
    PlayCallSelection,
    RefereeAssignment,
    KickFoulResolution,
    AddedTimeDecision,
}

impl RngStream {
    pub fn stream_id(self) -> u64 {
        match self {
            Self::DuelResolution => 1,
            Self::ChallengeResolution => 2,
            Self::PlayCallSelection => 3,
            Self::RefereeAssignment => 4,
            Self::KickFoulResolution => 5,
            Self::AddedTimeDecision => 6,
        }
    }

    pub fn all() -> [Self; 6] {
        [
            Self::DuelResolution,
            Self::ChallengeResolution,
            Self::PlayCallSelection,
            Self::RefereeAssignment,
            Self::KickFoulResolution,
            Self::AddedTimeDecision,
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

pub fn derive_sub_seed_team_indexed(
    match_seed: MatchSeed,
    stream: RngStream,
    team_id: Uuid,
    index: u64,
) -> u64 {
    let stream_offset = stream.stream_id().wrapping_mul(SPLITMIX_GAMMA);
    let index_offset = (index.wrapping_add(1)).wrapping_mul(SPLITMIX_GAMMA ^ SPLITMIX_MIX_1);
    let val = team_id.as_u128();
    let high = (val >> 64) as u64;
    let low = val as u64;
    let team_offset = high.wrapping_mul(SPLITMIX_MIX_1) ^ low.wrapping_mul(SPLITMIX_MIX_2);
    let combined = match_seed
        .value()
        .wrapping_add(stream_offset)
        .wrapping_add(index_offset)
        .wrapping_add(team_offset);
    split_mix_64(combined)
}