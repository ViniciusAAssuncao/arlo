use crate::injury::definition_selection::select_injury_definition;
use crate::injury::exertion::context::ExertionInjuryContext;
use crate::injury::exertion::trigger::sample_exertion_injury_trigger;
use crate::injury::outcome::InjuryIncidentResolution;
use crate::injury::severity_estimation::estimate_injury_severity;
use crate::injury::susceptibility::derive_effective_susceptibility;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{NON_CONTACT_FATIGUE_WEIGHT, NON_CONTACT_VELOCITY_WEIGHT};
use arlo_domain::{BodyRegion, InjuryCatalog, InjuryMechanism};
use rand::Rng;

pub fn evaluate_and_resolve_exertion_injury<R: Rng + ?Sized>(
    ctx: &ExertionInjuryContext<'_>,
    catalog: &InjuryCatalog,
    rng: &mut R,
) -> Option<InjuryIncidentResolution> {
    let (triggered, trigger_prob) = sample_exertion_injury_trigger(ctx, rng);
    if !triggered {
        return None;
    }

    let fatigue = calculate_physical_exhaustion(&ctx.physical_state).clamp(0.0, 1.0);
    let speed_ratio = (ctx.peak_speed_meters_per_sec
        / ctx.critical_speed_meters_per_sec.max(1.0))
    .clamp(0.0, 2.0)
        / 2.0;
    let susceptibility = derive_effective_susceptibility(ctx.player_table, &ctx.injury_profile);

    let stimulus = calculate_weighted_average(&[
        (fatigue, NON_CONTACT_FATIGUE_WEIGHT),
        (speed_ratio, NON_CONTACT_VELOCITY_WEIGHT),
        ((susceptibility / 2.0).clamp(0.0, 1.0), 0.20),
    ])
    .unwrap_or(0.3);

    let severity_grade = estimate_injury_severity(stimulus);
    let selected_def = select_injury_definition(catalog, InjuryMechanism::NonContact, rng);

    let (injury_definition_id, body_region) = match selected_def {
        Some(def) => (Some(def.id()), def.body_region()),
        None => (None, BodyRegion::Thigh),
    };

    Some(InjuryIncidentResolution::new(
        ctx.player_id,
        ctx.team_id,
        InjuryMechanism::NonContact,
        body_region,
        severity_grade,
        injury_definition_id,
        trigger_prob,
    ))
}
