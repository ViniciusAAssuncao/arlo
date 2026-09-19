use crate::artrine::execution::drive_profile::DriveAwardProfile;
use crate::home_advantage::HomeAdvantageProfile;
use crate::injury::tuning::InjuryTuningProfile;
use crate::physical::tuning::EnergyTuningProfile;
use crate::scoring_model::ScoringDifficultyProfile;
use crate::team_strength::{LeagueStrengthScale, TeamStrengthProfile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineTuning {
    pub scoring_difficulty: ScoringDifficultyProfile,
    pub energy_tuning: EnergyTuningProfile,
    pub injury_tuning: InjuryTuningProfile,
    pub league_strength_scale: LeagueStrengthScale,
    pub team_strength_profile: TeamStrengthProfile,
    pub home_advantage_profile: HomeAdvantageProfile,
    pub drive_award: DriveAwardProfile,
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
            league_strength_scale: LeagueStrengthScale::default(),
            team_strength_profile: TeamStrengthProfile::default(),
            home_advantage_profile: HomeAdvantageProfile::default(),
            drive_award: DriveAwardProfile::default(),
        }
    }

    pub fn with_league_strength_scale(mut self, league_strength_scale: LeagueStrengthScale) -> Self {
        self.league_strength_scale = league_strength_scale;
        self
    }

    pub fn with_team_strength_profile(mut self, team_strength_profile: TeamStrengthProfile) -> Self {
        self.team_strength_profile = team_strength_profile;
        self
    }

    pub fn with_home_advantage_profile(mut self, home_advantage_profile: HomeAdvantageProfile) -> Self {
        self.home_advantage_profile = home_advantage_profile;
        self
    }

    pub fn with_drive_award(mut self, drive_award: DriveAwardProfile) -> Self {
        self.drive_award = drive_award;
        self
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

    pub fn league_strength_scale(&self) -> &LeagueStrengthScale {
        &self.league_strength_scale
    }

    pub fn team_strength_profile(&self) -> &TeamStrengthProfile {
        &self.team_strength_profile
    }

    pub fn home_advantage_profile(&self) -> &HomeAdvantageProfile {
        &self.home_advantage_profile
    }

    pub fn drive_award(&self) -> &DriveAwardProfile {
        &self.drive_award
    }
}

impl Default for EngineTuning {
    fn default() -> Self {
        Self {
            scoring_difficulty: ScoringDifficultyProfile::default(),
            energy_tuning: EnergyTuningProfile::default(),
            injury_tuning: InjuryTuningProfile::default(),
            league_strength_scale: LeagueStrengthScale::default(),
            team_strength_profile: TeamStrengthProfile::default(),
            home_advantage_profile: HomeAdvantageProfile::default(),
            drive_award: DriveAwardProfile::default(),
        }
    }
}
