use crate::officiating::foul::attribution::{resolve_offending_side, FoulOffendingSide};
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::officiating::foul::origin::FoulOrigin;
use crate::officiating::foul::outcome::FoulResolution;
use crate::officiating::foul::punishment_selection::select_punishment;
use crate::officiating::foul::severity_estimation::estimate_foul_severity;
use crate::officiating::foul::trigger::{evaluate_foul_trigger_probability, sample_foul_trigger};
use crate::officiating::heads_or_tails::resolve_peace_referee_review;
use crate::officiating::stimulus::saturating_stimulus;
use arlo_domain::FaultCatalog;
use arlo_math::stats::contrast::logistic;
use rand::Rng;

pub fn evaluate_and_resolve_foul<R: Rng + ?Sized>(
    ctx: &FoulEvaluationContext<'_>,
    catalog: &FaultCatalog,
    rng: &mut R,
) -> Option<FoulResolution> {
    if !sample_foul_trigger(ctx, rng) {
        return None;
    }

    let offending_side = resolve_offending_side(ctx, rng);
    let (offending_player_id, offending_team_id, opposing_player_id, opposing_team_id) =
        match offending_side {
            FoulOffendingSide::Carrier => (
                ctx.carrier_id,
                ctx.carrier_team_id,
                ctx.defender_id,
                ctx.defender_team_id,
            ),
            FoulOffendingSide::Defender => (
                ctx.defender_id,
                ctx.defender_team_id,
                ctx.carrier_id,
                ctx.carrier_team_id,
            ),
        };

    let severity = estimate_foul_severity(ctx, offending_side);
    let punishment = select_punishment(catalog, severity, ctx.is_open_play, rng);

    let (fault_definition_id, punishment_kind, punishment_magnitude) = match punishment {
        Some(p) => (Some(p.fault_definition_id), Some(p.kind), p.magnitude),
        None => (None, None, None),
    };

    let raw_stimulus = if ctx.contact_severity > 1e-6 {
        ctx.contact_severity
    } else {
        ctx.duel_outcome.net_advantage().abs()
    };
    let stimulus = saturating_stimulus(raw_stimulus);

    let (original_call_correct, peace_referee_intervened) =
        resolve_peace_referee_review(stimulus, ctx.peace_referee_table, rng);

    let trigger_probability = logistic(evaluate_foul_trigger_probability(ctx));

    Some(FoulResolution::new(
        offending_player_id,
        offending_team_id,
        opposing_player_id,
        opposing_team_id,
        FoulOrigin::ContactDuel(ctx.duel_outcome.kind()),
        trigger_probability,
        original_call_correct,
        peace_referee_intervened,
        fault_definition_id,
        punishment_kind,
        punishment_magnitude,
    ))
}
