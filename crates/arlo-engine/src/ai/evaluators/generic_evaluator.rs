use crate::ai::evaluators::action_evalutors::{ActionEvaluationConfig, ActionKindConfig};
use crate::ai::evaluators::context::DecisionEvaluationContext;
use arlo_domain::sport_constants::{
    FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};

pub fn evaluate_action_utility(
    ctx: &DecisionEvaluationContext<'_>,
    config: &ActionEvaluationConfig,
) -> f64 {
    let profile = (config.profile_fn)(config.decision_kind);
    let intrinsic_rating = ctx.carrier_rating(profile);
    let skill_mult = intrinsic_rating / 10.0;

    let (p_succ, p_to, p_fail, delta_succ, delta_fail, delta_to, geometry_factor, urgency_bonus) =
        match &config.kind_config {
            ActionKindConfig::Progression(prog) => {
                let (adv_mirim, est_drives) = (prog.advance_fn)(ctx, skill_mult);
                let new_drives = ctx.situation.drives_in_series + est_drives;
                let (new_down, new_rem) = if adv_mirim >= ctx.situation.remaining_advance_mirim {
                    (1, 10.0)
                } else {
                    (
                        ctx.situation.down.saturating_add(1).min(4),
                        (ctx.situation.remaining_advance_mirim - adv_mirim).max(0.0),
                    )
                };
                let new_norm_x = (ctx.situation.normalized_proximity
                    + adv_mirim / ctx.situation.pitch_length_mirim.max(1.0))
                .min(1.0);
                let epv_success = ctx.epv_model.calculate_epa(
                    new_norm_x,
                    new_down,
                    new_rem,
                    new_drives,
                    ctx.situation.is_bonus_phase,
                    &ctx.scoring_regime,
                );
                let epv_fail = if ctx.situation.down >= 4 && ctx.situation.remaining_advance_mirim > 0.0 {
                    -ctx.opponent_epa()
                } else {
                    ctx.epv_model.calculate_epa(
                        ctx.situation.normalized_proximity,
                        ctx.situation.down.saturating_add(1).min(4),
                        ctx.situation.remaining_advance_mirim,
                        ctx.situation.drives_in_series,
                        ctx.situation.is_bonus_phase,
                        &ctx.scoring_regime,
                    )
                };
                let epv_to = -ctx.opponent_epa();

                let raw_p = (prog.success_prob_fn)(ctx, skill_mult);
                let p_succ = ctx.bound_probability(raw_p);
                let (min_p, _) = ctx.carrier.probability_bounds;
                let p_to = (((1.0 - p_succ) * prog.turnover_scale)
                    / ctx.situation.game_state_pressure.turnover_aversion_scale())
                .clamp(min_p, (1.0 - p_succ).max(min_p));
                let p_fail = (1.0 - p_succ - p_to).max(0.0);

                let urgency = (prog.urgency_bonus_fn)(ctx, est_drives, skill_mult);

                (
                    p_succ,
                    p_to,
                    p_fail,
                    epv_success - ctx.current_epv,
                    epv_fail - ctx.current_epv,
                    epv_to - ctx.current_epv,
                    1.0,
                    urgency,
                )
            }
            ActionKindConfig::TerminalScore(term) => {
                let value = if ctx.situation.is_bonus_phase {
                    FIELD_GOAL_GOALPOST_VALUE as f64
                } else if ctx.situation.drives_in_series
                    >= ctx.scoring_regime.goal_point_required_drives
                {
                    GOAL_POINT_VALUE as f64
                } else {
                    FIELD_POINT_VALUE as f64
                };
                let v_opp = ctx.opponent_epa();
                let raw_p = (term.success_prob_fn)(ctx, skill_mult);
                let p_succ = ctx.bound_probability(raw_p);
                let p_fail = 1.0 - p_succ;
                let p_to = 0.0;
                let geom = (term.geometry_factor_fn)(ctx);

                (
                    p_succ,
                    p_to,
                    p_fail,
                    value - ctx.current_epv,
                    -v_opp - (ctx.current_epv * 0.15),
                    0.0,
                    geom,
                    0.0,
                )
            }
        };

    let v_succ = ctx.risk_profile.transform_value(delta_succ);
    let v_fail = ctx.risk_profile.transform_value(delta_fail);
    let v_to = ctx.risk_profile.transform_value(delta_to);

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

    let gravity_factor = 0.70 + 0.30 * ctx.situation.offensive_gravity.min(2.0);
    let risk_multiplier = ctx.risk_profile.risk_multiplier_for_action(config.decision_kind);
    let game_state_bias = ctx.situation.game_state_pressure.bias_for_decision(
        config.decision_kind,
        ctx.situation.drives_in_series,
        ctx.situation.is_bonus_phase,
        &ctx.scoring_regime,
        ctx.situation.down,
        ctx.situation.normalized_proximity,
    );
    let emphasis_multiplier = 1.0 + ctx.emphasis_for(config.decision_kind);
    let tactical_bias = ctx.carrier_tactical_bias(config.decision_kind);

    let base_utility = (expected_future_value * gravity_factor * geometry_factor + urgency_bonus)
        * risk_multiplier
        * game_state_bias
        * 3.5
        + (intrinsic_rating * config.rating_additive_weight);

    let biased_utility = if base_utility >= 0.0 {
        base_utility * tactical_bias * emphasis_multiplier
    } else {
        base_utility / (tactical_bias.max(0.1) * emphasis_multiplier.max(0.1))
    };

    biased_utility
}