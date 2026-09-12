use crate::officiating::foul::attribution::{recklessness_score, FoulOffendingSide};
use crate::officiating::foul::context::FoulEvaluationContext;
use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{
    FAULT_SEVERITY_FLAGRANT_THRESHOLD, FAULT_SEVERITY_MODERATE_THRESHOLD,
    FAULT_SEVERITY_SEVERE_THRESHOLD, FIRST_ZONE_FOUL_SEVERITY_MULTIPLIER,
    FOUL_INTENSITY_CONTACT_SEVERITY_WEIGHT, FOUL_INTENSITY_DUEL_BASELINE_WEIGHT,
    FOUL_INTENSITY_RECKLESSNESS_WEIGHT,
};
use arlo_domain::{FaultSeverity, PitchZone};

pub fn estimate_foul_severity(
    ctx: &FoulEvaluationContext<'_>,
    offending_side: FoulOffendingSide,
) -> FaultSeverity {
    let recklessness = match offending_side {
        FoulOffendingSide::Carrier => recklessness_score(ctx.carrier_table),
        FoulOffendingSide::Defender => recklessness_score(ctx.defender_table),
    };
    let contact = ctx.contact_severity.clamp(0.0, 1.0);
    let baseline = ctx.duel_outcome.kind().physicality_baseline().clamp(0.0, 1.0);

    let intensity = calculate_weighted_average(&[
        (contact, FOUL_INTENSITY_CONTACT_SEVERITY_WEIGHT),
        (recklessness, FOUL_INTENSITY_RECKLESSNESS_WEIGHT),
        (baseline, FOUL_INTENSITY_DUEL_BASELINE_WEIGHT),
    ])
    .unwrap_or(0.0);

    let zone_multiplier = if ctx.defender_zone == PitchZone::FirstZone {
        FIRST_ZONE_FOUL_SEVERITY_MULTIPLIER
    } else {
        1.0
    };

    let normalized_intensity = (intensity * zone_multiplier).clamp(0.0, 1.0);

    if normalized_intensity >= FAULT_SEVERITY_FLAGRANT_THRESHOLD {
        FaultSeverity::Flagrant
    } else if normalized_intensity >= FAULT_SEVERITY_SEVERE_THRESHOLD {
        FaultSeverity::Severe
    } else if normalized_intensity >= FAULT_SEVERITY_MODERATE_THRESHOLD {
        FaultSeverity::Moderate
    } else {
        FaultSeverity::Minor
    }
}