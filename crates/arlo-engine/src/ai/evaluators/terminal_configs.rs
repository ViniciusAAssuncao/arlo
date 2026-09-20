use crate::ai::evaluators::action_evalutors::{ActionEvaluationConfig, ActionKindConfig};
use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::scoring_model::{calculate_scoring_probability, ScoringKind, ScoringOrigin, ScoringSituation};
use arlo_domain::ArtrineDecisionKind;

#[derive(Clone, Copy)]
pub struct TerminalScoreConfig {
    pub success_prob_fn: fn(&DecisionEvaluationContext<'_>, f64) -> f64,
    pub geometry_factor_fn: fn(&DecisionEvaluationContext<'_>) -> f64,
}

pub fn cross_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::Cross,
        profile_fn: crate::caching::get_cached_decision_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::TerminalScore(TerminalScoreConfig {
            success_prob_fn: |ctx, skill_mult| {
                let scoring_kind = if ctx.situation.is_bonus_phase {
                    ScoringKind::FieldGoal(arlo_events::ScoringPost::Fieldpost)
                } else if ctx.situation.drives_in_series >= ctx.scoring_regime.goal_point_required_drives {
                    ScoringKind::GoalPoint
                } else {
                    ScoringKind::FieldPoint
                };
                let zone = crate::possession::locate_zone_default(
                    ctx.situation.normalized_proximity,
                    ctx.situation.pitch_length_mirim,
                );
                let situation = ScoringSituation::new(
                    zone,
                    ctx.situation.normalized_proximity,
                    ctx.situation.drives_in_series,
                    10.0,
                    10.0 + skill_mult * 10.0,
                    10.0,
                    false,
                    ScoringOrigin::OpenPlay,
                );

                let raw_prob = calculate_scoring_probability(
                    scoring_kind,
                    &situation,
                    &ctx.scoring_difficulty,
                )
                .value();

                let perceived = (raw_prob + 0.10 * ctx.situation.target_quality)
                    * (0.60 + 0.40 * ctx.situation.offensive_gravity.min(2.0))
                    * (0.80 + 0.25 * ctx.lateral_ratio());
                (perceived * 1.25).clamp(0.0, 1.0)
            },
            geometry_factor_fn: |ctx| 0.70 + 0.60 * ctx.lateral_ratio(),
        }),
    }
}

pub fn finish_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::SelfFinish,
        profile_fn: crate::caching::get_cached_decision_profile,
        rating_additive_weight: 0.25,
        kind_config: ActionKindConfig::TerminalScore(TerminalScoreConfig {
            success_prob_fn: |ctx, skill_mult| {
                let scoring_kind = if ctx.situation.is_bonus_phase {
                    ScoringKind::FieldGoal(arlo_events::ScoringPost::Goalpost)
                } else if ctx.situation.drives_in_series >= ctx.scoring_regime.goal_point_required_drives {
                    ScoringKind::GoalPoint
                } else {
                    ScoringKind::FieldPoint
                };
                let zone = crate::possession::locate_zone_default(
                    ctx.situation.normalized_proximity,
                    ctx.situation.pitch_length_mirim,
                );
                let situation = ScoringSituation::new(
                    zone,
                    ctx.situation.normalized_proximity,
                    ctx.situation.drives_in_series,
                    10.0,
                    10.0 + skill_mult * 10.0,
                    10.0,
                    ctx.situation.normalized_proximity >= 0.75,
                    ScoringOrigin::OpenPlay,
                );

                let raw_prob = calculate_scoring_probability(scoring_kind, &situation, &ctx.scoring_difficulty).value()
                    * ctx.shooting_angle_factor();
                (raw_prob * 1.25).clamp(0.0, 1.0)
            },
            geometry_factor_fn: |ctx| {
                let zone_multiplier =
                    crate::resolution::finish_distance_multiplier(ctx.situation.normalized_proximity);
                let shooting_lane_clearance = 0.60 + 0.40 * ctx.situation.pitch_control;
                zone_multiplier * shooting_lane_clearance
            },
        }),
    }
}
