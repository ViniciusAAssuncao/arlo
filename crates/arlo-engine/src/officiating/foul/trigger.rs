use crate::attributes::PlayerAttributeTable;
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::resolution::{resolve_contest, ContestRequest, ContestOrientation};
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::AttributeKey;
use rand::Rng;

fn player_recklessness_rating(table: &PlayerAttributeTable) -> f64 {
    let strength = table.get(AttributeKey::Strength);
    let controlled_aggression = table.get(AttributeKey::ControlledAggression);
    (strength - controlled_aggression).max(0.0)
}

pub fn build_foul_trigger_request<'a>(ctx: &'a FoulEvaluationContext<'a>) -> ContestRequest<'a> {
    let carrier_exhaustion = calculate_physical_exhaustion(&ctx.carrier_physical_state);
    let defender_exhaustion = calculate_physical_exhaustion(&ctx.defender_physical_state);

    let carrier_reckless = player_recklessness_rating(ctx.carrier_table);
    let defender_reckless = player_recklessness_rating(ctx.defender_table);

    let carrier_imprudence = carrier_reckless * (1.0 + 0.25 * carrier_exhaustion);
    let defender_imprudence = defender_reckless * (1.0 + 0.25 * defender_exhaustion);
    let mean_imprudence = (carrier_imprudence + defender_imprudence) * 0.5;

    let contact_stimulus = ctx.contact_severity * 1.5 + ctx.duel_outcome.net_advantage().abs() * 0.15;
    let urgency_bonus = ctx.game_state_pressure.urgency_index() * 0.20;
    let attacker_imprudence_rating = mean_imprudence + contact_stimulus + urgency_bonus;

    let referee_rigor: f64 = ctx
        .head_referee_table
        .get(AttributeKey::Rigor)
        .clamp(0.0_f64, ATTRIBUTE_MAX);
    let referee_tolerance_rating: f64 = (35.0_f64 - referee_rigor).clamp(15.0_f64, 45.0_f64);

    ContestRequest::for_contest(
        ctx.duel_outcome.kind(),
        attacker_imprudence_rating,
        referee_tolerance_rating,
        &ctx.duel_context.with_orientation(ContestOrientation::Neutral),
    )
    .with_slope(0.15)
}

pub fn resolve_foul_trigger<R: Rng + ?Sized>(
    ctx: &FoulEvaluationContext<'_>,
    rng: &mut R,
) -> (bool, f64) {
    let req = build_foul_trigger_request(ctx);
    let outcome = resolve_contest(req, rng);
    (outcome.attacker_won(), outcome.win_probability().value())
}

pub fn sample_foul_trigger<R: Rng + ?Sized>(ctx: &FoulEvaluationContext<'_>, rng: &mut R) -> bool {
    resolve_foul_trigger(ctx, rng).0
}

pub fn evaluate_foul_trigger_probability(ctx: &FoulEvaluationContext<'_>) -> f64 {
    let req = build_foul_trigger_request(ctx);
    let slope = req.slope_override.unwrap_or(0.15);
    let mut offset = req.context.aggression_logit_offset() + req.context.physicality_logit_offset();
    if req.context.attacker_is_home() {
        offset += req.context.home_advantage_duel_logit();
    }
    if req.context.defender_is_home() {
        offset -= req.context.home_advantage_duel_logit();
    }
    arlo_math::stats::contrast::bradley_terry_with_offset(
        req.attacker_rating,
        req.defender_rating,
        slope,
        offset,
    )
    .value()
}