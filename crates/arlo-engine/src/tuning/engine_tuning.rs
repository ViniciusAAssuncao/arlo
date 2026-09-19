use crate::scoring_model::ScoringDifficultyProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineTuning {
    pub scoring_difficulty: ScoringDifficultyProfile,
}

impl EngineTuning {
    pub fn new(scoring_difficulty: ScoringDifficultyProfile) -> Self {
        Self { scoring_difficulty }
    }

    pub fn scoring_difficulty(&self) -> &ScoringDifficultyProfile {
        &self.scoring_difficulty
    }
}

impl Default for EngineTuning {
    fn default() -> Self {
        Self {
            scoring_difficulty: ScoringDifficultyProfile::default(),
        }
    }
}