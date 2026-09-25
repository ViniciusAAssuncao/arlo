use crate::input::MatchInput;
use crate::state::MatchState;
use arlo_domain::{FaultPunishmentOption, FaultSeverity, PunishmentKind};
use rand::Rng;
use uuid::Uuid;

pub(in crate::resolution) fn select_punishments(
    input: &MatchInput,
    state: &mut MatchState,
    definition_id: Uuid,
) -> Vec<(PunishmentKind, Option<i32>)> {
    let options = input.fault_catalog().punishment_options(&definition_id);
    if options.is_empty() {
        return Vec::new();
    }
    let severity = input.fault_catalog().definition(&definition_id).map(|definition| definition.severity());
    let head_rigor = input.referees()[0]
        .attributes()
        .iter()
        .find(|attribute| input.referee_attribute_keys().get(&attribute.attribute_definition_id()) == Some(&arlo_domain::AttributeKey::Rigor))
        .map(|attribute| attribute.value() as f64)
        .unwrap_or(10.0);
    let firmness = ((head_rigor - 10.0) / 10.0).clamp(-1.0, 1.0);
    let first = choose_option(options, firmness, None, state);
    let mut selected = vec![first];
    let additional_probability: f64 = match severity {
        Some(FaultSeverity::Minor) => 0.04,
        Some(FaultSeverity::Moderate) => 0.17,
        Some(FaultSeverity::Severe) => 0.38,
        Some(FaultSeverity::Flagrant) => 0.65,
        None => 0.0,
    };
    if options.iter().any(|option| option.kind() != first.kind())
        && state.rng_mut().gen_range(0.0..1.0) < additional_probability
    {
        selected.push(choose_option(options, firmness, Some(first.kind()), state));
    }
    selected
        .into_iter()
        .map(|option| (option.kind(), sample_magnitude(option, state)))
        .collect()
}

fn choose_option<'a>(
    options: &'a [FaultPunishmentOption],
    firmness: f64,
    excluded: Option<PunishmentKind>,
    state: &mut MatchState,
) -> &'a FaultPunishmentOption {
    let eligible: Vec<_> = options.iter().filter(|option| Some(option.kind()) != excluded).collect();
    let weights: Vec<f64> = eligible
        .iter()
        .map(|option| (1.0 + firmness * (option.kind().relative_severity() - 1.5) * 0.55).max(0.15))
        .collect();
    let mut draw = state.rng_mut().gen_range(0.0..weights.iter().sum::<f64>());
    for (option, weight) in eligible.iter().zip(weights) {
        if draw < weight {
            return option;
        }
        draw -= weight;
    }
    eligible[eligible.len() - 1]
}

fn sample_magnitude(option: &FaultPunishmentOption, state: &mut MatchState) -> Option<i32> {
    match (option.magnitude_min(), option.magnitude_max()) {
        (Some(min), Some(max)) if min <= max => Some(state.rng_mut().gen_range(min..=max)),
        (Some(min), None) => Some(min),
        (None, Some(max)) => Some(max),
        _ => None,
    }
}
