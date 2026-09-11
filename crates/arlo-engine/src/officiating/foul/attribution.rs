use crate::attributes::PlayerAttributeTable;
use crate::officiating::foul::context::FoulEvaluationContext;
use arlo_domain::sport_constants::{ATTRIBUTE_MAX, FOUL_ATTRIBUTION_STEEPNESS};
use arlo_domain::AttributeKey;
use arlo_math::stats::contrast::bradley_terry_with_offset;
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
    ctx: &FoulEvaluationContext,
    rng: &mut R,
) -> FoulOffendingSide {
    let defender_reckless = recklessness_score(&ctx.defender_table);
    let carrier_reckless = recklessness_score(&ctx.carrier_table);
    let prob = bradley_terry_with_offset(
        defender_reckless,
        carrier_reckless,
        FOUL_ATTRIBUTION_STEEPNESS,
        0.0,
    );
    if prob.sample(rng) {
        FoulOffendingSide::Defender
    } else {
        FoulOffendingSide::Carrier
    }
}
