use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::evaluator_trait::ActionUtilityEvaluator;
use crate::artrine::decision_profiles::cross_profile;
use arlo_domain::sport_constants::{
    FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_domain::ArtrineDecisionKind;
use arlo_math::units::MIRIM_TO_METERS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossUtilityEvaluator;

impl ActionUtilityEvaluator for CrossUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::Cross
    }

    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64 {
        let profile = cross_profile();
        let intrinsic_rating = ctx.carrier_rating(&profile);
        let skill_mult = ctx.skill_multiplier(intrinsic_rating);
        let target_qual = ctx.target_quality();

        let value = if ctx.drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
            GOAL_POINT_VALUE as f64
        } else if ctx.drives_in_series >= 1 {
            FIELD_POINT_VALUE as f64
        } else {
            2.0
        };

        let center_y_m = (ctx.pitch_width_mirim * 0.5) * MIRIM_TO_METERS;
        let dist_from_center_m = (ctx.carrier_pos_vec.raw().1 - center_y_m).abs();
        let lateral_ratio = (dist_from_center_m / center_y_m.max(1.0)).clamp(0.0, 1.0);
        let lateral_geometry_factor = 0.70 + 0.60 * lateral_ratio;

        let raw_p =
            (0.35 + 0.35 * ctx.normalized_proximity + 0.20 * target_qual + 0.10 * skill_mult)
                * (0.60 + 0.40 * ctx.offensive_gravity.min(2.0))
                * (0.80 + 0.25 * lateral_ratio);
        let p_goal = ctx.bound_probability(raw_p);

        let v_opp = ctx.opponent_epa();

        let delta_succ = value - ctx.current_epv;
        let delta_to = -v_opp - ctx.current_epv;

        let v_succ = ctx.risk_profile.transform_value(delta_succ);
        let v_to = ctx.risk_profile.transform_value(delta_to);

        let w_succ = ctx.risk_profile.weight_probability(p_goal);
        let w_to = ctx.risk_profile.weight_probability(1.0 - p_goal);

        let expected_future_value = w_succ * v_succ + w_to * v_to;

        let gravity_factor = 0.70 + 0.30 * ctx.offensive_gravity.min(2.0);
        let risk_multiplier = ctx
            .risk_profile
            .risk_multiplier_for_action(ArtrineDecisionKind::Cross);
        let game_state_bias = ctx
            .game_state_pressure
            .bias_for_decision(ArtrineDecisionKind::Cross, ctx.drives_in_series);
        let emphasis_multiplier = 1.0 + ctx.play_call_emphasis.cross().value();
        let tactical_bias = ctx.carrier_tactical_bias(ArtrineDecisionKind::Cross);

        ((expected_future_value * gravity_factor * lateral_geometry_factor)
            * risk_multiplier
            * game_state_bias
            * 3.5
            + (intrinsic_rating * 0.2))
            * emphasis_multiplier
            * tactical_bias
    }
}
