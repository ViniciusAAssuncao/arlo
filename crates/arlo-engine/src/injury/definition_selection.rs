use arlo_domain::{InjuryCatalog, InjuryDefinition, InjuryMechanism};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use smallvec::SmallVec;

pub fn select_injury_definition<'a, R: Rng + ?Sized>(
    catalog: &'a InjuryCatalog,
    mechanism: InjuryMechanism,
    rng: &mut R,
) -> Option<&'a InjuryDefinition> {
    let def_ids = catalog.definitions_for_mechanism(mechanism);
    if def_ids.is_empty() {
        return None;
    }

    if def_ids.len() == 1 {
        return catalog.definition(&def_ids[0]);
    }

    let mut weights: SmallVec<[f64; 16]> = SmallVec::with_capacity(def_ids.len());
    let mut definitions: SmallVec<[&'a InjuryDefinition; 16]> =
        SmallVec::with_capacity(def_ids.len());

    for id in def_ids {
        if let Some(def) = catalog.definition(id) {
            weights.push(def.relative_frequency().max(0.0));
            definitions.push(def);
        }
    }

    if definitions.is_empty() {
        return None;
    }

    let chosen_idx = sample_categorical(&weights, rng).unwrap_or(0);
    definitions.get(chosen_idx).copied()
}
