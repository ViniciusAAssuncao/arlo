use crate::injury::age_risk::calculate_age_risk_multiplier;
use crate::injury::contact::context::ContactInjuryContext;
use crate::injury::susceptibility::derive_effective_susceptibility;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{
    BASE_CONTACT_INJURY_PROBABILITY, CONTACT_COLLISION_INTENSITY_WEIGHT, CONTACT_FATIGUE_WEIGHT,
    CONTACT_VULNERABILITY_WEIGHT,
};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;

pub fn evaluate_contact_injury_probability(
    is_carrier: bool,
    ctx: &ContactInjuryContext<'_>,
) -> f64 {
    let (table, physical_state, profile, age_years) = if is_carrier {
        (
            ctx.carrier_table,
            &ctx.carrier_physical_state,
            &ctx.carrier_injury_profile,
            ctx.carrier_age_years,
        )
    } else {
        (
            ctx.defender_table,
            &ctx.defender_physical_state,
            &ctx.defender_injury_profile,
            ctx.defender_age_years,
        )
    };

    let collision_intensity = ctx.collision.contact_severity.clamp(0.0, 1.0);
    let fatigue = calculate_physical_exhaustion(physical_state).clamp(0.0, 1.0);
    let susceptibility = derive_effective_susceptibility(table, profile);
    let normalized_vulnerability = (susceptibility / 2.0).clamp(0.0, 1.0);

    let risk_stimulus = calculate_weighted_average(&[
        (collision_intensity, CONTACT_COLLISION_INTENSITY_WEIGHT),
        (fatigue, CONTACT_FATIGUE_WEIGHT),
        (normalized_vulnerability, CONTACT_VULNERABILITY_WEIGHT),
    ])
    .unwrap_or(0.1);

    let age_mult = calculate_age_risk_multiplier(age_years);
    let p0 = BASE_CONTACT_INJURY_PROBABILITY;
    let base_logit = (p0 / (1.0 - p0)).ln();
    let modulated_logit = base_logit + (risk_stimulus * 2.5) + (age_mult - 1.0) * 0.8;

    logistic(modulated_logit)
}

pub fn sample_contact_injury_trigger<R: Rng + ?Sized>(
    is_carrier: bool,
    ctx: &ContactInjuryContext<'_>,
    rng: &mut R,
) -> (bool, f64) {
    let prob = evaluate_contact_injury_probability(is_carrier, ctx);
    let triggered = Probability::new_clamped(prob).sample(rng);
    (triggered, prob)
}
