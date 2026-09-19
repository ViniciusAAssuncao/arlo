use crate::scoring_model::{
    calculate_scoring_probability, ScoringDifficultyProfile, ScoringKind, ScoringOrigin,
    ScoringSituation,
};
use crate::scoring_regime::ScoringRegimePolicy;
use arlo_domain::sport_constants::{FIELD_POINT_VALUE, GOAL_POINT_VALUE};
use arlo_domain::PitchZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DynamicEpvModel {
    offensive_gravity: f64,
    difficulty_profile: ScoringDifficultyProfile,
}

impl DynamicEpvModel {
    pub fn new(offensive_gravity: f64) -> Self {
        Self {
            offensive_gravity: offensive_gravity.max(0.1),
            difficulty_profile: ScoringDifficultyProfile::default(),
        }
    }

    pub fn with_difficulty(
        offensive_gravity: f64,
        difficulty_profile: ScoringDifficultyProfile,
    ) -> Self {
        Self {
            offensive_gravity: offensive_gravity.max(0.1),
            difficulty_profile,
        }
    }

    pub fn offensive_gravity(&self) -> f64 {
        self.offensive_gravity
    }

    pub fn difficulty_profile(&self) -> ScoringDifficultyProfile {
        self.difficulty_profile
    }

    pub fn goal_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        _down: u8,
        _remaining_advance_mirim: f64,
        regime: &ScoringRegimePolicy,
    ) -> f64 {
        if drives_in_series < regime.goal_point_required_drives {
            return (normalized_x * 0.05).clamp(0.0, 0.05);
        }

        let situation = ScoringSituation::new(
            PitchZone::FirstZone,
            normalized_x,
            drives_in_series,
            20.0,
            12.0 + self.offensive_gravity,
            10.0,
            true,
            ScoringOrigin::OpenPlay,
        );
        calculate_scoring_probability(
            ScoringKind::GoalPoint,
            &situation,
            &self.difficulty_profile,
        )
        .value()
    }

    pub fn field_point_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        _down: u8,
        _remaining_advance_mirim: f64,
        regime: &ScoringRegimePolicy,
    ) -> f64 {
        if drives_in_series < regime.field_point_required_drives && normalized_x < 0.40 {
            return 0.0;
        }

        let situation = ScoringSituation::new(
            PitchZone::SecondZone,
            normalized_x,
            drives_in_series,
            20.0,
            10.0 + self.offensive_gravity,
            10.0,
            false,
            ScoringOrigin::OpenPlay,
        );
        calculate_scoring_probability(
            ScoringKind::FieldPoint,
            &situation,
            &self.difficulty_profile,
        )
        .value()
    }

    pub fn turnover_probability(
        &self,
        normalized_x: f64,
        down: u8,
        _remaining_advance_mirim: f64,
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        if down >= 4 {
            (0.60 - x * 0.30).clamp(0.15, 0.85)
        } else {
            (0.12 - x * 0.06 + (down as f64) * 0.03).clamp(0.02, 0.40)
        }
    }

    pub fn opponent_epa(&self, normalized_x: f64) -> f64 {
        let opp_x = (1.0 - normalized_x).clamp(0.0, 1.0);
        opp_x * 3.5
    }

    pub fn calculate_epa(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
        regime: &ScoringRegimePolicy,
    ) -> f64 {
        let p_goal = self.goal_probability(
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
            regime,
        );
        let p_field = self.field_point_probability(
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
            regime,
        );
        let p_to = self.turnover_probability(normalized_x, down, remaining_advance_mirim);
        let opp_val = self.opponent_epa(normalized_x);

        p_goal * (GOAL_POINT_VALUE as f64) + p_field * (FIELD_POINT_VALUE as f64)
            - p_to * opp_val
    }
}

impl Default for DynamicEpvModel {
    fn default() -> Self {
        Self::new(1.0)
    }
}

pub type EpvModel = DynamicEpvModel;