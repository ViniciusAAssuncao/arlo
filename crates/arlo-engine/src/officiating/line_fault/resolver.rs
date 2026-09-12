use crate::ai::cognitive::action_probability;
use crate::officiating::foul::origin::FoulOrigin;
use crate::officiating::foul::outcome::FoulResolution;
use crate::officiating::heads_or_tails::flip_officiating_coin;
use crate::officiating::line_fault::context::LineFaultEvaluationContext;
use arlo_domain::sport_constants::{
    LINE_FAULT_MARGIN_STIMULUS_SCALE_MIRIM, PEACE_REFEREE_AUTHORITY_SCALE,
    PEACE_REFEREE_BASE_SENSITIVITY, PEACE_REFEREE_INTERVENTION_STEEPNESS,
};
use arlo_domain::{AttributeKey, PunishmentKind};
use arlo_math::units::MIRIM_TO_METERS;
use rand::Rng;

pub fn evaluate_and_resolve_line_fault<R: Rng + ?Sized>(
    ctx: &LineFaultEvaluationContext,
    rng: &mut R,
) -> Option<FoulResolution> {
    if ctx.offside_margin_meters <= 0.0 {
        return None;
    }

    let original_call_correct = flip_officiating_coin(rng);

    let margin_mirim = ctx.offside_margin_meters / MIRIM_TO_METERS;
    let normalized = margin_mirim / LINE_FAULT_MARGIN_STIMULUS_SCALE_MIRIM;
    let stimulus = normalized / (normalized + 1.0);

    let intervention_prob = action_probability(
        stimulus,
        ctx.peace_referee_table.get(AttributeKey::Authority),
        PEACE_REFEREE_BASE_SENSITIVITY,
        PEACE_REFEREE_AUTHORITY_SCALE,
        PEACE_REFEREE_INTERVENTION_STEEPNESS,
    );
    let peace_referee_intervened = intervention_prob.sample(rng);

    Some(FoulResolution::new(
        ctx.receiver_id,
        ctx.receiver_team_id,
        ctx.defender_id,
        ctx.defender_team_id,
        FoulOrigin::LineFault,
        1.0,
        original_call_correct,
        peace_referee_intervened,
        None,
        Some(PunishmentKind::KickFoulAwarded),
        None,
    ))
}