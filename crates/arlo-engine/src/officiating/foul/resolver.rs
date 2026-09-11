use crate::ai::cognitive::action_probability;
use crate::officiating::foul::attribution::{resolve_offending_side, FoulOffendingSide};
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::officiating::foul::outcome::FoulResolution;
use crate::officiating::foul::punishment_selection::select_punishment;
use crate::officiating::foul::severity_estimation::estimate_foul_severity;
use crate::officiating::foul::trigger::{evaluate_foul_trigger_probability, sample_foul_trigger};
use crate::officiating::heads_or_tails::flip_officiating_coin;
use arlo_domain::sport_constants::{
    PEACE_REFEREE_AUTHORITY_SCALE, PEACE_REFEREE_BASE_SENSITIVITY,
    PEACE_REFEREE_INTERVENTION_STEEPNESS,
};
use arlo_domain::{AttributeKey, FaultCatalog};
use arlo_math::stats::contrast::logistic;
use rand::Rng;

pub fn evaluate_and_resolve_foul<R: Rng + ?Sized>(
    ctx: &FoulEvaluationContext,
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
    let punishment = select_punishment(catalog, severity, rng);

    let (fault_definition_id, punishment_kind, punishment_magnitude) = match punishment {
        Some(p) => (Some(p.fault_definition_id), Some(p.kind), p.magnitude),
        None => (None, None, None),
    };

    let original_call_correct = flip_officiating_coin(rng);

    let stimulus = if ctx.contact_severity > 1e-6 {
        ctx.contact_severity / (ctx.contact_severity + 1.0)
    } else {
        let net_adv = ctx.duel_outcome.net_advantage().abs();
        net_adv / (net_adv + 1.0)
    };

    let intervention_prob = action_probability(
        stimulus,
        ctx.peace_referee_table.get(AttributeKey::Authority),
        PEACE_REFEREE_BASE_SENSITIVITY,
        PEACE_REFEREE_AUTHORITY_SCALE,
        PEACE_REFEREE_INTERVENTION_STEEPNESS,
    );
    let peace_referee_intervened = intervention_prob.sample(rng);

    let trigger_probability = logistic(evaluate_foul_trigger_probability(ctx));

    Some(FoulResolution::new(
        offending_player_id,
        offending_team_id,
        opposing_player_id,
        opposing_team_id,
        ctx.duel_outcome.kind(),
        trigger_probability,
        original_call_correct,
        peace_referee_intervened,
        fault_definition_id,
        punishment_kind,
        punishment_magnitude,
    ))
}