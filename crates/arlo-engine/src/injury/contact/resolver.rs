use crate::injury::contact::context::ContactInjuryContext;
use crate::injury::contact::trigger::sample_contact_injury_trigger;
use crate::injury::definition_selection::select_injury_definition;
use crate::injury::outcome::InjuryIncidentResolution;
use crate::injury::severity_estimation::estimate_injury_severity;
use crate::injury::susceptibility::derive_effective_susceptibility;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{CONTACT_COLLISION_INTENSITY_WEIGHT, CONTACT_FATIGUE_WEIGHT};
use arlo_domain::{BodyRegion, InjuryCatalog, InjuryMechanism};
use rand::Rng;

pub fn evaluate_and_resolve_contact_injury<R: Rng + ?Sized>(
    is_carrier: bool,
    ctx: &ContactInjuryContext<'_>,
    catalog: &InjuryCatalog,
    rng: &mut R,
) -> Option<InjuryIncidentResolution> {
    let (triggered, trigger_prob) = sample_contact_injury_trigger(is_carrier, ctx, rng);
    if !triggered {
        return None;
    }

    let (player_id, team_id, table, physical_state, profile) = if is_carrier {
        (
            ctx.carrier_id,
            ctx.carrier_team_id,
            ctx.carrier_table,
            &ctx.carrier_physical_state,
            &ctx.carrier_injury_profile,
        )
    } else {
        (
            ctx.defender_id,
            ctx.defender_team_id,
            ctx.defender_table,
            &ctx.defender_physical_state,
            &ctx.defender_injury_profile,
        )
    };

    let collision_intensity = ctx.collision.contact_severity.clamp(0.0, 1.0);
    let fatigue = calculate_physical_exhaustion(physical_state).clamp(0.0, 1.0);
    let susceptibility = derive_effective_susceptibility(table, profile);

    let stimulus = calculate_weighted_average(&[
        (collision_intensity, CONTACT_COLLISION_INTENSITY_WEIGHT),
        (fatigue, CONTACT_FATIGUE_WEIGHT),
        ((susceptibility / 2.0).clamp(0.0, 1.0), 0.20),
    ])
    .unwrap_or(0.3);

    let severity_grade = estimate_injury_severity(stimulus);
    let selected_def = select_injury_definition(catalog, InjuryMechanism::Contact, rng);

    let (injury_definition_id, body_region) = match selected_def {
        Some(def) => (Some(def.id()), def.body_region()),
        None => (None, BodyRegion::Knee),
    };

    Some(InjuryIncidentResolution::new(
        player_id,
        team_id,
        InjuryMechanism::Contact,
        body_region,
        severity_grade,
        injury_definition_id,
        trigger_prob,
    ))
}