use crate::ai::evaluators::action_configs::{ActionEvaluationConfig, ActionKindConfig};
use crate::ai::evaluators::context::DecisionEvaluationContext;
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE,
    GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};

pub fn evaluate_action_utility(
    ctx: &DecisionEvaluationContext<'_>,
    config: &ActionEvaluationConfig,
) -> f64 {
    if !(config.rule_validator_fn)(ctx) {
        return 0.0;
    }

    let profile = (config.profile_fn)();
    let intrinsic_rating = ctx.carrier_rating(&profile);
    let skill_mult = ctx.skill_multiplier(intrinsic_rating);

    let (p_succ, p_to, p_fail, delta_succ, delta_fail, delta_to, geometry_factor, urgency_bonus) =
        match &config.kind_config {
            ActionKindConfig::Progression(prog) => {
                let (adv_mirim, est_drives) = (prog.advance_fn)(ctx, skill_mult);
                let new_drives = ctx.drives_in_series + est_drives;
                let (new_down, new_rem) = if adv_mirim >= ctx.remaining_advance_mirim {
                    (1, 10.0)
                } else {
                    (
                        ctx.down.saturating_add(1).min(4),
                        (ctx.remaining_advance_mirim - adv_mirim).max(0.0),
                    )
                };
                let new_norm_x =
                    (ctx.normalized_proximity + adv_mirim / ctx.pitch_length_mirim().max(1.0)).min(1.0);
                let epv_success =
                    ctx.epv_model.calculate_epa(new_norm_x, new_down, new_rem, new_drives);
                let epv_fail = if ctx.down >= 4 && ctx.remaining_advance_mirim > 0.0 {
                    -ctx.opponent_epa()
                } else {
                    ctx.epv_model.calculate_epa(
                        ctx.normalized_proximity,
                        ctx.down.saturating_add(1).min(4),
                        ctx.remaining_advance_mirim,
                        ctx.drives_in_series,
                    )
                };
                let epv_to = -ctx.opponent_epa();

                let raw_p = (prog.success_prob_fn)(ctx, skill_mult);
                let p_succ = ctx.bound_probability(raw_p);
                let (min_p, _) = ctx.probability_bounds();
                let p_to = (((1.0 - p_succ) * prog.turnover_scale)
                    / ctx.game_state_pressure.turnover_aversion_scale())
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
                let advance_in_series = (10.0 - ctx.remaining_advance_mirim).max(0.0);
                let value = if ctx.drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
                    GOAL_POINT_VALUE as f64
                } else if ctx.drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
                    && (advance_in_series >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
                        || ctx.normalized_proximity >= 0.70)
                {
                    FIELD_POINT_VALUE as f64
                } else {
                    0.0
                };
                let v_opp = ctx.opponent_epa();
                let raw_p = (term.success_prob_fn)(ctx, skill_mult);
                let p_succ = ctx.bound_probability(raw_p);
                let p_to = 1.0 - p_succ;
                let p_fail = 0.0;
                let geom = (term.geometry_factor_fn)(ctx);

                (
                    p_succ,
                    p_to,
                    p_fail,
                    value - ctx.current_epv,
                    0.0,
                    -v_opp - ctx.current_epv,
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

    let gravity_factor = 0.70 + 0.30 * ctx.offensive_gravity.min(2.0);
    let risk_multiplier = ctx.risk_profile.risk_multiplier_for_action(config.decision_kind);
    let game_state_bias = ctx
        .game_state_pressure
        .bias_for_decision(config.decision_kind, ctx.drives_in_series);
    let emphasis_multiplier = 1.0 + ctx.emphasis_for(config.decision_kind);
    let tactical_bias = ctx.carrier_tactical_bias(config.decision_kind);

    ((expected_future_value * gravity_factor * geometry_factor + urgency_bonus)
        * risk_multiplier
        * game_state_bias
        * 3.5
        + (intrinsic_rating * config.rating_additive_weight))
        * emphasis_multiplier
        * tactical_bias
}