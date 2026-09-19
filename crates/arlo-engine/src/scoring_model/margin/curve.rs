use crate::scoring_model::margin::margin_context::MarginContext;
use crate::scoring_model::margin::profile::MarginPenaltyProfile;
use crate::scoring_model::scoring_kind::ScoringKind;

pub fn calculate_margin_penalty_logit(
    kind: ScoringKind,
    context: &MarginContext,
    profile: &MarginPenaltyProfile,
) -> f64 {
    let type_weight = profile.weight_for_kind(kind);
    if type_weight <= 0.0 {
        return 0.0;
    }

    if context.advantage_gp_equivalents <= 0.0 {
        return 0.0;
    }

    let x = context.advantage_gp_equivalents - profile.threshold_gp;
    let beta = profile.softplus_beta.max(0.01);
    let sp = if beta * x > 30.0 {
        x
    } else if beta * x < -30.0 {
        0.0
    } else {
        (1.0 + (beta * x).exp()).ln() / beta
    };

    let sat_scale = profile.saturation_scale.max(0.01);
    let saturation = (sp / sat_scale).tanh().clamp(0.0, 1.0);

    let g0 = profile.equilibrium_z_width.max(0.01);
    let z_ratio = context.strength_z_gap.abs() / g0;
    let bell_factor = 1.0 / (1.0 + z_ratio * z_ratio);

    type_weight * profile.max_penalty_logit * saturation * bell_factor
}
