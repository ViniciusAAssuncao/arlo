use arlo_domain::{BodyRegion, InjurySeverityGrade};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RecoveryTuningProfile {
    pub energy_base_daily_recovery: f64,
    pub energy_stamina_weight: f64,
    pub energy_natural_fitness_weight: f64,
    pub energy_age_inflection_years: f64,
    pub energy_age_penalty_slope: f64,
    pub energy_max_age_penalty: f64,

    pub anaerobic_base_daily_recovery: f64,
    pub anaerobic_natural_fitness_weight: f64,

    pub impulse_base_daily_reversion_rate: f64,
    pub impulse_determination_weight: f64,
    pub impulse_composure_weight: f64,
    pub impulse_consistency_weight: f64,

    pub conditioning_daily_gain_active: f64,
    pub conditioning_daily_loss_inactive: f64,
    pub conditioning_daily_loss_injured: f64,
    pub conditioning_recovery_multiplier_min: f64,
    pub conditioning_recovery_multiplier_max: f64,

    pub injury_grade_1_base_days: f64,
    pub injury_grade_2_base_days: f64,
    pub injury_grade_3_base_days: f64,

    pub region_multiplier_head: f64,
    pub region_multiplier_neck: f64,
    pub region_multiplier_shoulder: f64,
    pub region_multiplier_arm: f64,
    pub region_multiplier_hand: f64,
    pub region_multiplier_trunk: f64,
    pub region_multiplier_hip: f64,
    pub region_multiplier_groin: f64,
    pub region_multiplier_thigh: f64,
    pub region_multiplier_knee: f64,
    pub region_multiplier_calf: f64,
    pub region_multiplier_ankle: f64,
    pub region_multiplier_foot: f64,

    pub observation_duration_fraction: f64,
    pub observation_min_days: u32,
    pub observation_max_days: u32,

    pub relapse_base_daily_probability: f64,
    pub relapse_conditioning_mitigation_factor: f64,
    pub relapse_natural_fitness_weight: f64,
    pub relapse_grade_1_multiplier: f64,
    pub relapse_grade_2_multiplier: f64,
    pub relapse_grade_3_multiplier: f64,
}

impl Default for RecoveryTuningProfile {
    fn default() -> Self {
        Self {
            energy_base_daily_recovery: 0.45,
            energy_stamina_weight: 0.25,
            energy_natural_fitness_weight: 0.3,
            energy_age_inflection_years: 28.0,
            energy_age_penalty_slope: 0.15,
            energy_max_age_penalty: 0.35,

            anaerobic_base_daily_recovery: 0.85,
            anaerobic_natural_fitness_weight: 0.2,

            impulse_base_daily_reversion_rate: 0.3,
            impulse_determination_weight: 0.2,
            impulse_composure_weight: 0.25,
            impulse_consistency_weight: 0.35,

            conditioning_daily_gain_active: 0.035,
            conditioning_daily_loss_inactive: 0.015,
            conditioning_daily_loss_injured: 0.04,
            conditioning_recovery_multiplier_min: 0.7,
            conditioning_recovery_multiplier_max: 1.2,

            injury_grade_1_base_days: 7.0,
            injury_grade_2_base_days: 28.0,
            injury_grade_3_base_days: 90.0,

            region_multiplier_head: 1.0,
            region_multiplier_neck: 1.1,
            region_multiplier_shoulder: 1.05,
            region_multiplier_arm: 0.85,
            region_multiplier_hand: 0.8,
            region_multiplier_trunk: 0.95,
            region_multiplier_hip: 1.25,
            region_multiplier_groin: 1.2,
            region_multiplier_thigh: 1.0,
            region_multiplier_knee: 1.4,
            region_multiplier_calf: 1.05,
            region_multiplier_ankle: 1.05,
            region_multiplier_foot: 1.15,

            observation_duration_fraction: 0.3,
            observation_min_days: 3,
            observation_max_days: 30,

            relapse_base_daily_probability: 0.0015,
            relapse_conditioning_mitigation_factor: 0.6,
            relapse_natural_fitness_weight: 0.25,
            relapse_grade_1_multiplier: 0.8,
            relapse_grade_2_multiplier: 1.8,
            relapse_grade_3_multiplier: 3.5,
        }
    }
}

impl RecoveryTuningProfile {
    pub fn base_injury_days(&self, grade: InjurySeverityGrade) -> f64 {
        match grade {
            InjurySeverityGrade::Grade1 => self.injury_grade_1_base_days,
            InjurySeverityGrade::Grade2 => self.injury_grade_2_base_days,
            InjurySeverityGrade::Grade3 => self.injury_grade_3_base_days,
        }
    }

    pub fn region_multiplier(&self, region: BodyRegion) -> f64 {
        match region {
            BodyRegion::Head => self.region_multiplier_head,
            BodyRegion::Neck => self.region_multiplier_neck,
            BodyRegion::Shoulder => self.region_multiplier_shoulder,
            BodyRegion::Arm => self.region_multiplier_arm,
            BodyRegion::Hand => self.region_multiplier_hand,
            BodyRegion::Trunk => self.region_multiplier_trunk,
            BodyRegion::Hip => self.region_multiplier_hip,
            BodyRegion::Groin => self.region_multiplier_groin,
            BodyRegion::Thigh => self.region_multiplier_thigh,
            BodyRegion::Knee => self.region_multiplier_knee,
            BodyRegion::Calf => self.region_multiplier_calf,
            BodyRegion::Ankle => self.region_multiplier_ankle,
            BodyRegion::Foot => self.region_multiplier_foot,
        }
    }

    pub fn calculate_observation_days(&self, injury_recovery_days: u32) -> u32 {
        let calculated =
            ((injury_recovery_days as f64) * self.observation_duration_fraction).round() as u32;
        calculated.clamp(self.observation_min_days, self.observation_max_days)
    }

    pub fn relapse_grade_multiplier(&self, grade: InjurySeverityGrade) -> f64 {
        match grade {
            InjurySeverityGrade::Grade1 => self.relapse_grade_1_multiplier,
            InjurySeverityGrade::Grade2 => self.relapse_grade_2_multiplier,
            InjurySeverityGrade::Grade3 => self.relapse_grade_3_multiplier,
        }
    }

    pub fn conditioning_fatigue_multiplier(&self, conditioning_score: f64) -> f64 {
        let clamped = conditioning_score.clamp(0.0, 1.0);
        self.conditioning_recovery_multiplier_min
            + clamped
                * (self.conditioning_recovery_multiplier_max
                    - self.conditioning_recovery_multiplier_min)
    }
}
