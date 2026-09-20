use crate::scoring_regime::ScoringRegimePolicy;
use crate::world_state::core::constants::*;
use arlo_domain::sport_constants::{
    KICK_FOUL_CROSS_LATERAL_BIAS_BASE, KICK_FOUL_CROSS_LATERAL_BIAS_SCALE,
    KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_FIRST_ZONE,
    KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_STANDARD,
};
use arlo_domain::{ArtrineDecisionKind, KickFoulDecisionKind, KickFoulScoringTier};

pub fn calculate_decision_bias(
    kind: ArtrineDecisionKind,
    drives_in_series: u32,
    regime: &ScoringRegimePolicy,
    offensive_risk_bias: f64,
) -> f64 {
    let action_mult = |coeff: f64| (1.0 + coeff * offensive_risk_bias).clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS);

    let raw = match kind {
        ArtrineDecisionKind::SelfCarry => {
            let base = action_mult(-0.35);
            if drives_in_series >= regime.goal_point_required_drives {
                base * 0.35
            } else if drives_in_series >= regime.field_point_required_drives {
                base * 0.55
            } else {
                base * (1.0
                    + CARRY_EARLY_DRIVE_BONUS_MULTIPLIER
                        * ((regime.goal_point_required_drives - drives_in_series) as f64))
            }
        }
        ArtrineDecisionKind::ShortPass => {
            let base = action_mult(-0.40);
            if drives_in_series >= regime.goal_point_required_drives {
                base * 0.35
            } else if drives_in_series >= regime.field_point_required_drives {
                base * 0.55
            } else {
                base
            }
        }
        ArtrineDecisionKind::LongLaunch => {
            let base = action_mult(0.70);
            if drives_in_series >= regime.goal_point_required_drives {
                base * 0.40
            } else if drives_in_series >= regime.field_point_required_drives {
                base * 0.60
            } else {
                base
            }
        }
        ArtrineDecisionKind::Cross => {
            let scoring = if drives_in_series >= regime.goal_point_required_drives {
                action_mult(0.80) * 3.60
            } else if drives_in_series >= regime.field_point_required_drives {
                action_mult(0.50) * 2.30
            } else {
                action_mult(0.20)
            };
            action_mult(0.65) * scoring
        }
        ArtrineDecisionKind::SelfFinish => {
            let scoring = if drives_in_series >= regime.goal_point_required_drives {
                action_mult(0.80) * 3.80
            } else if drives_in_series >= regime.field_point_required_drives {
                action_mult(0.50) * 2.50
            } else {
                action_mult(0.20)
            };
            action_mult(0.60) * scoring
        }
    };
    raw.clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
}

pub fn calculate_kick_foul_bias(
    kind: KickFoulDecisionKind,
    tier: KickFoulScoringTier,
    lateral_ratio: f64,
    offensive_risk_bias: f64,
) -> f64 {
    let action_mult = |coeff: f64| (1.0 + coeff * offensive_risk_bias).clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS);

    let raw = match kind {
        KickFoulDecisionKind::Shoot => match tier {
            KickFoulScoringTier::FirstZone => {
                let weight = KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_FIRST_ZONE;
                action_mult(0.60) * (1.0 - weight) + action_mult(0.80) * weight
            }
            KickFoulScoringTier::Standard => {
                let weight = KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_STANDARD;
                action_mult(0.60) * (1.0 - weight) + action_mult(0.20) * weight
            }
        },
        KickFoulDecisionKind::Cross => {
            let lateral_factor = KICK_FOUL_CROSS_LATERAL_BIAS_BASE
                + KICK_FOUL_CROSS_LATERAL_BIAS_SCALE * lateral_ratio.clamp(0.0, 1.0);
            action_mult(0.65) * lateral_factor
        }
        KickFoulDecisionKind::ShortPass => action_mult(-0.40),
        KickFoulDecisionKind::LongLaunch => action_mult(0.70),
    };
    raw.clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
}