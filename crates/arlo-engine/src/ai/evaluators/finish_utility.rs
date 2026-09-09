use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::evaluator_trait::ActionUtilityEvaluator;
use crate::artrine::decision_profiles::self_finish_profile;
use arlo_domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIELD_POINT_VALUE, FIRST_ZONE_DEPTH_MIRIM,
    GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_domain::ArtrineDecisionKind;
use arlo_math::units::MIRIM_TO_METERS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelfFinishUtilityEvaluator;

impl ActionUtilityEvaluator for SelfFinishUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfFinish
    }

    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64 {
        let profile = self_finish_profile();
        let intrinsic_rating = ctx.carrier_rating(&profile);
        let skill_mult = ctx.skill_multiplier(intrinsic_rating);

        let value = if ctx.drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
            GOAL_POINT_VALUE as f64
        } else if ctx.drives_in_series >= 1 {
            FIELD_POINT_VALUE as f64
        } else {
            2.0
        };

        let distance_to_goal_mirim =
            ((1.0 - ctx.normalized_proximity) * ctx.pitch_length_mirim).max(0.0);
        let second_zone_limit = FIRST_ZONE_DEPTH_MIRIM + AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;

        let zone_multiplier = if distance_to_goal_mirim <= FIRST_ZONE_DEPTH_MIRIM {
            1.85
        } else if distance_to_goal_mirim <= second_zone_limit {
            1.45
        } else if distance_to_goal_mirim <= second_zone_limit + 15.0 {
            1.10
        } else {
            (1.0 / (1.0 + (distance_to_goal_mirim - second_zone_limit - 15.0) * 0.08))
                .clamp(0.30, 1.0)
        };

        let center_y_m = (ctx.pitch_width_mirim * 0.5) * MIRIM_TO_METERS;
        let angle_offset =
            ((ctx.carrier_pos_vec.raw().1 - center_y_m).abs() / center_y_m.max(1.0)).clamp(0.0, 1.0);
        let shooting_angle_factor = (1.0 - 0.40 * angle_offset).clamp(0.60, 1.0);

        let pitch_control = ctx.pitch_control();
        let shooting_lane_clearance = 0.60 + 0.40 * pitch_control;

        let distance_p_factor = (1.0 / (1.0 + distance_to_goal_mirim * 0.05)).clamp(0.20, 1.0);
        let raw_p = (0.20 + 0.40 * distance_p_factor + 0.20 * skill_mult + 0.20 * pitch_control)
            * shooting_angle_factor;
        let p_goal = ctx.bound_probability(raw_p);
        let v_opp = ctx.epv_model.opponent_epa(ctx.normalized_proximity);

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
            .risk_multiplier_for_action(ArtrineDecisionKind::SelfFinish);
        let game_state_bias = ctx
            .game_state_pressure
            .bias_for_decision(ArtrineDecisionKind::SelfFinish, ctx.drives_in_series);
        let emphasis_multiplier = 1.0 + ctx.play_call_emphasis.self_finish().value();
        let tactical_bias = ctx.carrier_tactical_bias(ArtrineDecisionKind::SelfFinish);

        ((expected_future_value * gravity_factor * zone_multiplier * shooting_lane_clearance)
            * risk_multiplier
            * game_state_bias
            * 3.5
            + (intrinsic_rating * 0.25))
            * emphasis_multiplier
            * tactical_bias
    }
}