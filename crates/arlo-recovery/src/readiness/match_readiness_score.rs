use crate::domain::{InjuryStatusKind, PlayerCondition};
use crate::tuning::ReadinessTuningProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReadinessLevel {
    FullyFit,
    RecentReturn,
    CautionRecommended,
    HighRisk,
}

impl ReadinessLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FullyFit => "FullyFit",
            Self::RecentReturn => "RecentReturn",
            Self::CautionRecommended => "CautionRecommended",
            Self::HighRisk => "HighRisk",
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            Self::FullyFit => "Totalmente Apto",
            Self::RecentReturn => "Retorno Recente",
            Self::CautionRecommended => "Cautela Recomendada",
            Self::HighRisk => "Alto Risco",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MatchReadinessAssessment {
    pub score: f64,
    pub level: ReadinessLevel,
}

impl MatchReadinessAssessment {
    pub fn new(score: f64, level: ReadinessLevel) -> Self {
        Self { score, level }
    }
}

pub fn calculate_match_readiness_raw(
    energy: f64,
    anaerobic_reserve: f64,
    conditioning_score: f64,
    status: InjuryStatusKind,
    days_since_resolution: Option<u32>,
    tuning: &ReadinessTuningProfile,
) -> MatchReadinessAssessment {
    let base_physical = (energy.clamp(0.0, 1.0) * tuning.energy_weight)
        + (anaerobic_reserve.clamp(0.0, 1.0) * tuning.anaerobic_weight)
        + (conditioning_score.clamp(0.0, 1.0) * tuning.conditioning_weight);

    let medical_factor = match status {
        InjuryStatusKind::Injured => 1.0 - tuning.active_injury_penalty,
        InjuryStatusKind::Observation => 1.0 - tuning.active_observation_penalty,
        InjuryStatusKind::Healthy => match days_since_resolution {
            Some(days) if days < tuning.recent_return_window_days => {
                let fraction_remaining = (tuning.recent_return_window_days - days) as f64
                    / tuning.recent_return_window_days as f64;
                let penalty = tuning.recent_return_max_penalty * fraction_remaining;
                1.0 - penalty
            }
            _ => 1.0,
        },
    };

    let score = (base_physical * medical_factor).clamp(0.0, 1.0);

    let is_recent_return = status == InjuryStatusKind::Healthy
        && days_since_resolution.map_or(false, |d| d < tuning.recent_return_window_days);

    let level = if status == InjuryStatusKind::Injured || score < tuning.high_risk_threshold {
        ReadinessLevel::HighRisk
    } else if is_recent_return {
        ReadinessLevel::RecentReturn
    } else if status == InjuryStatusKind::Observation || score < tuning.caution_threshold {
        ReadinessLevel::CautionRecommended
    } else if score >= tuning.fully_fit_threshold {
        ReadinessLevel::FullyFit
    } else {
        ReadinessLevel::CautionRecommended
    };

    MatchReadinessAssessment::new(score, level)
}

pub fn calculate_match_readiness(
    condition: &PlayerCondition,
    days_since_resolution: Option<u32>,
    tuning: &ReadinessTuningProfile,
) -> MatchReadinessAssessment {
    calculate_match_readiness_raw(
        condition.fatigue().energy(),
        condition.fatigue().w_prime(),
        condition.conditioning().readiness(),
        condition.status(),
        days_since_resolution,
        tuning,
    )
}