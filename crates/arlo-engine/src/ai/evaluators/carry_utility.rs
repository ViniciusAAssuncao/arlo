use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::evaluator_trait::ActionUtilityEvaluator;
use crate::artrine::decision_profiles::self_carry_profile;
use arlo_domain::ArtrineDecisionKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryUtilityEvaluator;

impl ActionUtilityEvaluator for CarryUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfCarry
    }

    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64 {
        let profile = self_carry_profile();
        let intrinsic_rating = ctx.artrine_rating(&profile);
        let skill_mult = ctx.skill_multiplier(intrinsic_rating);
        let pc = ctx.pitch_control();

        let adv_mirim = (4.0 + 4.0 * pc) * skill_mult;
        let crosses_artro = adv_mirim >= ctx.distance_to_next_artro_mirim.max(0.5);
        let artros_crossed = if crosses_artro {
            1 + ((adv_mirim - ctx.distance_to_next_artro_mirim).max(0.0) / 3.0).floor() as u32
        } else {
            0
        };
        let new_drives = ctx.drives_in_series + artros_crossed;

        let (new_down, new_rem) = if adv_mirim >= ctx.remaining_advance_mirim {
            (1, 10.0)
        } else {
            (
                ctx.down.saturating_add(1).min(4),
                (ctx.remaining_advance_mirim - adv_mirim).max(0.0),
            )
        };

        let new_norm_x = (ctx.normalized_proximity + adv_mirim / ctx.pitch_length_mirim.max(1.0)).min(1.0);
        let epv_success = ctx.epv_model.calculate_epa(new_norm_x, new_down, new_rem, new_drives);
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

        let (min_p, _max_p) = ctx.probability_bounds();
        let p_succ = ctx.bound_probability(pc * 0.75 + 0.15 * skill_mult);
        let p_to = ((1.0 - pc) * 0.15 / ctx.game_state_pressure.turnover_aversion_scale())
            .clamp(min_p, (1.0 - p_succ).max(min_p));
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
        let risk_multiplier = ctx.risk_profile.risk_multiplier_for_action(ArtrineDecisionKind::SelfCarry);
        let game_state_bias = ctx.game_state_pressure.bias_for_decision(ArtrineDecisionKind::SelfCarry, ctx.drives_in_series);

        (expected_future_value * gravity_factor) * risk_multiplier * game_state_bias * 3.5 + (intrinsic_rating * 0.2)
    }
}