use crate::attributes::PlayerAttributeTable;
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::resolution::{resolve_contest, ContestRequest, DuelContext};
use arlo_domain::sport_constants::{ATTRIBUTE_MAX, FOUL_ATTRIBUTION_STEEPNESS};
use arlo_domain::AttributeKey;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoulOffendingSide {
    Carrier,
    Defender,
}

pub fn recklessness_score(table: &PlayerAttributeTable) -> f64 {
    let strength = table.get(AttributeKey::Strength);
    let controlled_aggression = table.get(AttributeKey::ControlledAggression);
    ((strength - controlled_aggression) / ATTRIBUTE_MAX).clamp(0.0, 1.0)
}

pub fn resolve_offending_side<R: Rng + ?Sized>(
    ctx: &FoulEvaluationContext<'_>,
    rng: &mut R,
) -> FoulOffendingSide {
    let defender_reckless = recklessness_score(ctx.defender_table);
    let carrier_reckless = recklessness_score(ctx.carrier_table);
    let duel_context = DuelContext::neutral();
    let req = ContestRequest::for_contest(
        ctx.duel_outcome.kind(),
        defender_reckless,
        carrier_reckless,
        &duel_context,
    )
    .with_slope(FOUL_ATTRIBUTION_STEEPNESS);
    let outcome = resolve_contest(req, rng);
    if outcome.attacker_won() {
        FoulOffendingSide::Defender
    } else {
        FoulOffendingSide::Carrier
    }
}
