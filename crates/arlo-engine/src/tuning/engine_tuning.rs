use crate::injury::tuning::InjuryTuningProfile;
use crate::physical::tuning::EnergyTuningProfile;
use crate::scoring_model::ScoringDifficultyProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineTuning {
    pub scoring_difficulty: ScoringDifficultyProfile,
    pub energy_tuning: EnergyTuningProfile,
    pub injury_tuning: InjuryTuningProfile,
}

impl EngineTuning {
    pub fn new(
        scoring_difficulty: ScoringDifficultyProfile,
        energy_tuning: EnergyTuningProfile,
        injury_tuning: InjuryTuningProfile,
    ) -> Self {
        Self {
            scoring_difficulty,
            energy_tuning,
            injury_tuning,
        }
    }

    pub fn scoring_difficulty(&self) -> &ScoringDifficultyProfile {
        &self.scoring_difficulty
    }

    pub fn energy_tuning(&self) -> &EnergyTuningProfile {
        &self.energy_tuning
    }

    pub fn injury_tuning(&self) -> &InjuryTuningProfile {
        &self.injury_tuning
    }
}

impl Default for EngineTuning {
    fn default() -> Self {
        Self {
            scoring_difficulty: ScoringDifficultyProfile::default(),
            energy_tuning: EnergyTuningProfile::default(),
            injury_tuning: InjuryTuningProfile::default(),
        }
    }
}
