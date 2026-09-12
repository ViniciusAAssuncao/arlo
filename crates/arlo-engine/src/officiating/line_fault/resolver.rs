use crate::officiating::foul::origin::FoulOrigin;
use crate::officiating::foul::outcome::FoulResolution;
use crate::officiating::heads_or_tails::resolve_peace_referee_review;
use crate::officiating::line_fault::context::LineFaultEvaluationContext;
use crate::officiating::stimulus::saturating_stimulus;
use arlo_domain::sport_constants::LINE_FAULT_MARGIN_STIMULUS_SCALE_MIRIM;
use arlo_domain::PunishmentKind;
use arlo_math::units::MIRIM_TO_METERS;
use rand::Rng;

pub fn evaluate_and_resolve_line_fault<R: Rng + ?Sized>(
    ctx: &LineFaultEvaluationContext,
    rng: &mut R,
) -> Option<FoulResolution> {
    if ctx.offside_margin_meters <= 0.0 {
        return None;
    }

    let margin_mirim = ctx.offside_margin_meters / MIRIM_TO_METERS;
    let normalized = margin_mirim / LINE_FAULT_MARGIN_STIMULUS_SCALE_MIRIM;
    let stimulus = saturating_stimulus(normalized);

    let (original_call_correct, peace_referee_intervened) =
        resolve_peace_referee_review(stimulus, &ctx.peace_referee_table, rng);

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
