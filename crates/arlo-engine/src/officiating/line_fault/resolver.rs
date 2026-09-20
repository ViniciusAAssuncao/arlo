use crate::officiating::foul::origin::FoulOrigin;
use crate::officiating::foul::outcome::FoulResolution;
use crate::officiating::heads_or_tails::resolve_peace_referee_review;
use crate::officiating::line_fault::context::LineFaultEvaluationContext;
use crate::officiating::line_fault::detection::is_line_fault;
use crate::officiating::stimulus::saturating_stimulus;
use arlo_domain::PunishmentKind;
use rand::Rng;

pub fn evaluate_and_resolve_line_fault<R: Rng + ?Sized>(
    ctx: &LineFaultEvaluationContext<'_>,
    rng: &mut R,
) -> Option<FoulResolution> {
    let (fault_detected, margin) = is_line_fault(ctx, rng);
    if !fault_detected {
        return None;
    }

    let stimulus = saturating_stimulus(margin);

    let (original_call_correct, peace_referee_intervened) =
        resolve_peace_referee_review(stimulus, ctx.peace_referee_table, rng);

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
