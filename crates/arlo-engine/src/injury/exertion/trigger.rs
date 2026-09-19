use crate::injury::age_risk::calculate_age_risk_multiplier;
use crate::injury::exertion::context::ExertionInjuryContext;
use crate::injury::exposure::position_workload_multiplier;
use crate::injury::susceptibility::derive_effective_susceptibility;
use crate::injury::tuning::InjuryTuningProfile;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, NON_CONTACT_DECELERATION_WEIGHT, NON_CONTACT_FATIGUE_WEIGHT,
    NON_CONTACT_VELOCITY_WEIGHT,
};
use arlo_domain::{AttributeKey, Position};
use arlo_math::stats::hazard_rate_to_probability;
use arlo_math::Probability;
use rand::Rng;

pub fn evaluate_exertion_injury_probability(
    ctx: &ExertionInjuryContext<'_>,
    position: Position,
    tuning: &InjuryTuningProfile,
) -> f64 {
    let fatigue = calculate_physical_exhaustion(&ctx.physical_state).clamp(0.0, 1.0);
    let intensity = ctx.intensity_strain.clamp(0.0, 1.0);

    let agility = ctx.player_table.get(AttributeKey::Agility);
    let decel_vulnerability = ((ATTRIBUTE_MAX - agility) / ATTRIBUTE_MAX).clamp(0.0, 1.0);

    let exertion_score = calculate_weighted_average(&[
        (fatigue, NON_CONTACT_FATIGUE_WEIGHT),
        (intensity, NON_CONTACT_VELOCITY_WEIGHT),
        (decel_vulnerability, NON_CONTACT_DECELERATION_WEIGHT),
    ])
    .unwrap_or(0.1);

    let age_mult = calculate_age_risk_multiplier(ctx.age_years);
    let susceptibility = derive_effective_susceptibility(ctx.player_table, &ctx.injury_profile);
    let workload_mult = position_workload_multiplier(position);

    let hazard_rate = tuning.base_exertion_hazard_per_second()
        * age_mult
        * susceptibility
        * workload_mult
        * (0.50 + exertion_score * tuning.fatigue_hazard_multiplier());

    hazard_rate_to_probability(hazard_rate, ctx.exposure_duration_seconds)
}

pub fn sample_exertion_injury_trigger<R: Rng + ?Sized>(
    ctx: &ExertionInjuryContext<'_>,
    position: Position,
    tuning: &InjuryTuningProfile,
    rng: &mut R,
) -> (bool, f64) {
    let prob = evaluate_exertion_injury_probability(ctx, position, tuning);
    let triggered = Probability::new_clamped(prob).sample(rng);
    (triggered, prob)
}
