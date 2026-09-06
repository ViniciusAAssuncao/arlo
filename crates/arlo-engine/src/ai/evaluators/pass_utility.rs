use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::evaluator_trait::ActionUtilityEvaluator;
use crate::artrine::decision_profiles::{long_launch_profile, short_pass_profile};
use arlo_domain::ArtrineDecisionKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortPassUtilityEvaluator;

impl ActionUtilityEvaluator for ShortPassUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::ShortPass
    }

    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64 {
        let profile = short_pass_profile();
        let intrinsic_rating = ctx.artrine_rating(&profile);
        let skill_mult = ctx.skill_multiplier(intrinsic_rating);
        let target_qual = ctx.target_quality();

        let adv_mirim = (6.0 + 3.0 * target_qual) * skill_mult;
        let (new_down, new_rem) = if adv_mirim >= ctx.remaining_advance_mirim {
            (1, 10.0)
        } else {
            (
                ctx.down.saturating_add(1).min(4),
                (ctx.remaining_advance_mirim - adv_mirim).max(0.0),
            )
        };

        let new_norm_x = (ctx.normalized_proximity + adv_mirim / ctx.pitch_length_mirim.max(1.0)).min(1.0);
        let epv_success = ctx.epv_model.calculate_epa(new_norm_x, new_down, new_rem, ctx.drives_in_series);
        let epv_fail = if ctx.down >= 4 && ctx.remaining_advance_mirim > 0.0 {
            -ctx.epv_model.opponent_epa(ctx.normalized_proximity)
        } else {
            ctx.epv_model.calculate_epa(
                ctx.normalized_proximity,
                ctx.down.saturating_add(1).min(4),
                ctx.remaining_advance_mirim,
                ctx.drives_in_series,
            )
        };
        let epv_to = -ctx.epv_model.opponent_epa(ctx.normalized_proximity);

        let delta_succ = epv_success - ctx.current_epv;
        let delta_fail = epv_fail - ctx.current_epv;
        let delta_to = epv_to - ctx.current_epv;

        let v_succ = ctx.risk_profile.transform_value(delta_succ);
        let v_fail = ctx.risk_profile.transform_value(delta_fail);
        let v_to = ctx.risk_profile.transform_value(delta_to);

        let p_succ = (0.50
            + 0.04 * ctx.pass_protection_net_advantage
            + 0.20 * target_qual
            + 0.10 * skill_mult)
            .clamp(0.15, 0.95);
        let p_to = (((1.0 - p_succ) * 0.20) / ctx.game_state_pressure.turnover_aversion_scale()).clamp(0.01, 0.35);
        let p_fail = (1.0 - p_succ - p_to).max(0.0);

        let w_succ = ctx.risk_profile.weight_probability(p_succ);
        let w_to = ctx.risk_profile.weight_probability(p_to);
        let w_fail = ctx.risk_profile.weight_probability(p_fail);
        let total_w = w_succ + w_to + w_fail;
        let (nw_succ, nw_to, nw_fail) = if total_w > 0.0 {
            (w_succ / total_w, w_to / total_w, w_fail / total_w)
        } else {
            (p_succ, p_to, p_fail)
        };

        let expected_future_value = nw_succ * v_succ + nw_fail * v_fail + nw_to * v_to;

        let gravity_factor = 0.70 + 0.30 * ctx.offensive_gravity.min(2.0);
        let risk_multiplier = ctx.risk_profile.risk_multiplier_for_action(ArtrineDecisionKind::ShortPass);
        let game_state_bias = ctx.game_state_pressure.bias_for_decision(ArtrineDecisionKind::ShortPass, ctx.drives_in_series);

        (expected_future_value * gravity_factor) * risk_multiplier * game_state_bias * 3.5 + (intrinsic_rating * 0.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LongLaunchUtilityEvaluator;

impl ActionUtilityEvaluator for LongLaunchUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::LongLaunch
    }

    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64 {
        let profile = long_launch_profile();
        let intrinsic_rating = ctx.artrine_rating(&profile);
        let skill_mult = ctx.skill_multiplier(intrinsic_rating);
        let target_qual = ctx.target_quality();

        let adv_mirim = (14.0 + 6.0 * target_qual) * skill_mult;
        let (new_down, new_rem) = if adv_mirim >= ctx.remaining_advance_mirim {
            (1, 10.0)
        } else {
            (
                ctx.down.saturating_add(1).min(4),
                (ctx.remaining_advance_mirim - adv_mirim).max(0.0),
            )
        };

        let new_norm_x = (ctx.normalized_proximity + adv_mirim / ctx.pitch_length_mirim.max(1.0)).min(1.0);
        let epv_success = ctx.epv_model.calculate_epa(new_norm_x, new_down, new_rem, ctx.drives_in_series);
        let epv_fail = if ctx.down >= 4 && ctx.remaining_advance_mirim > 0.0 {
            -ctx.epv_model.opponent_epa(ctx.normalized_proximity)
        } else {
            ctx.epv_model.calculate_epa(
                ctx.normalized_proximity,
                ctx.down.saturating_add(1).min(4),
                ctx.remaining_advance_mirim,
                ctx.drives_in_series,
            )
        };
        let epv_to = -ctx.epv_model.opponent_epa(ctx.normalized_proximity);

        let delta_succ = epv_success - ctx.current_epv;
        let delta_fail = epv_fail - ctx.current_epv;
        let delta_to = epv_to - ctx.current_epv;

        let v_succ = ctx.risk_profile.transform_value(delta_succ);
        let v_fail = ctx.risk_profile.transform_value(delta_fail);
        let v_to = ctx.risk_profile.transform_value(delta_to);

        let p_succ = (0.35
            + 0.03 * ctx.pass_protection_net_advantage
            + 0.25 * target_qual
            + 0.10 * skill_mult)
            .clamp(0.10, 0.85);
        let p_to = (((1.0 - p_succ) * 0.35) / ctx.game_state_pressure.turnover_aversion_scale()).clamp(0.02, 0.50);
        let p_fail = (1.0 - p_succ - p_to).max(0.0);

        let w_succ = ctx.risk_profile.weight_probability(p_succ);
        let w_to = ctx.risk_profile.weight_probability(p_to);
        let w_fail = ctx.risk_profile.weight_probability(p_fail);
        let total_w = w_succ + w_to + w_fail;
        let (nw_succ, nw_to, nw_fail) = if total_w > 0.0 {
            (w_succ / total_w, w_to / total_w, w_fail / total_w)
        } else {
            (p_succ, p_to, p_fail)
        };

        let expected_future_value = nw_succ * v_succ + nw_fail * v_fail + nw_to * v_to;

        let gravity_factor = 0.70 + 0.30 * ctx.offensive_gravity.min(2.0);
        let risk_multiplier = ctx.risk_profile.risk_multiplier_for_action(ArtrineDecisionKind::LongLaunch);
        let game_state_bias = ctx.game_state_pressure.bias_for_decision(ArtrineDecisionKind::LongLaunch, ctx.drives_in_series);

        (expected_future_value * gravity_factor) * risk_multiplier * game_state_bias * 3.5 + (intrinsic_rating * 0.2)
    }
}