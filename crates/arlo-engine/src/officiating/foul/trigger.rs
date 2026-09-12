use crate::officiating::foul::attribution::recklessness_score;
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, BASE_FOUL_TRIGGER_LOGIT, FOUL_FATIGUE_LOGIT_SCALE,
    FOUL_LEVERAGE_LOGIT_SCALE, FOUL_NET_ADVANTAGE_LOGIT_SCALE,
    FOUL_RECKLESSNESS_LOGIT_SCALE, FOUL_REFEREE_CONSISTENCY_NOISE_SCALE,
    FOUL_REFEREE_RIGOR_LOGIT_SCALE,
};
use arlo_domain::AttributeKey;
use arlo_math::stats::contrast::logistic;
use arlo_math::stats::noise::sample_gaussian_noise;
use arlo_math::Probability;
use rand::Rng;

pub fn evaluate_foul_trigger_probability(ctx: &FoulEvaluationContext) -> f64 {
    let net_advantage_term =
        ctx.duel_outcome.net_advantage().abs() * FOUL_NET_ADVANTAGE_LOGIT_SCALE;

    let recklessness =
        recklessness_score(&ctx.carrier_table) + recklessness_score(&ctx.defender_table);
    let recklessness_term = recklessness * FOUL_RECKLESSNESS_LOGIT_SCALE;

    let physicality_term = ctx.duel_context.physicality_logit_offset();
    let aggression_term = ctx.duel_context.aggression_logit_offset();

    let combined_exhaustion = calculate_physical_exhaustion(&ctx.carrier_physical_state)
        + calculate_physical_exhaustion(&ctx.defender_physical_state);
    let fatigue_term = combined_exhaustion * FOUL_FATIGUE_LOGIT_SCALE;

    let leverage_term = ctx.game_state_pressure.urgency_index() * FOUL_LEVERAGE_LOGIT_SCALE;

    let norm_rigor = (ctx
        .head_referee_table
        .get(AttributeKey::Rigor)
        .clamp(0.0, ATTRIBUTE_MAX))
        / ATTRIBUTE_MAX;
    let rigor_term = norm_rigor * FOUL_REFEREE_RIGOR_LOGIT_SCALE;

    BASE_FOUL_TRIGGER_LOGIT
        + net_advantage_term
        + recklessness_term
        + physicality_term
        + aggression_term
        + fatigue_term
        + leverage_term
        + rigor_term
}

pub fn sample_foul_trigger<R: Rng + ?Sized>(ctx: &FoulEvaluationContext, rng: &mut R) -> bool {
    let base_logit = evaluate_foul_trigger_probability(ctx);

    let consistency = ctx
        .head_referee_table
        .get(AttributeKey::Consistency)
        .clamp(0.0, ATTRIBUTE_MAX);
    let norm_consistency = consistency / ATTRIBUTE_MAX;
    let noise_scale = FOUL_REFEREE_CONSISTENCY_NOISE_SCALE * (1.0 + (1.0 - norm_consistency));
    let jitter = sample_gaussian_noise(noise_scale, rng);

    let total_logit = base_logit + jitter;
    let prob = Probability::new_clamped(logistic(total_logit));
    prob.sample(rng)
}
