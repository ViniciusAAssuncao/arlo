use arlo_domain::{FaultCatalog, FaultSeverity, PunishmentKind};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PunishmentSelection {
    pub fault_definition_id: Uuid,
    pub kind: PunishmentKind,
    pub magnitude: Option<i32>,
}

impl PunishmentSelection {
    pub fn new(fault_definition_id: Uuid, kind: PunishmentKind, magnitude: Option<i32>) -> Self {
        Self {
            fault_definition_id,
            kind,
            magnitude,
        }
    }

    pub fn fault_definition_id(&self) -> Uuid {
        self.fault_definition_id
    }

    pub fn kind(&self) -> PunishmentKind {
        self.kind
    }

    pub fn magnitude(&self) -> Option<i32> {
        self.magnitude
    }
}

pub fn select_punishment<R: Rng + ?Sized>(
    catalog: &FaultCatalog,
    severity: FaultSeverity,
    is_open_play: bool,
    rng: &mut R,
) -> Option<PunishmentSelection> {
    let definitions = catalog.definitions_for_severity(severity);
    if definitions.is_empty() {
        return None;
    }

    let def_idx = rng.gen_range(0..definitions.len());
    let def_id = definitions[def_idx];

    let all_options = catalog.punishment_options(&def_id);
    let options: Vec<_> = all_options
        .iter()
        .filter(|opt| is_open_play || opt.kind() != PunishmentKind::KickFoulAwarded)
        .collect();

    if options.is_empty() {
        return None;
    }

    let weights: Vec<f64> = options
        .iter()
        .map(|opt| opt.kind().relative_severity().max(0.0))
        .collect();

    let chosen_opt_idx = sample_categorical(&weights, rng).unwrap_or(0);
    let chosen_opt = options[chosen_opt_idx];

    let magnitude = match (chosen_opt.magnitude_min(), chosen_opt.magnitude_max()) {
        (Some(min), Some(max)) => {
            if min <= max {
                Some(rng.gen_range(min..=max))
            } else {
                Some(min)
            }
        }
        (Some(min), None) => Some(min),
        (None, Some(max)) => Some(max),
        (None, None) => None,
    };

    Some(PunishmentSelection::new(
        def_id,
        chosen_opt.kind(),
        magnitude,
    ))
}