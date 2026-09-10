use crate::attributes::PlayerAttributeTable;
use crate::open_play::sample_carrier_decision;
pub use crate::open_play::sample_carrier_decision_from_table;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

use super::decision_engine::ArtrineDecisionResult;

pub fn sample_artrine_decision_from_table<R: Rng + ?Sized>(
    artrine: &Player,
    table: &PlayerAttributeTable,
    utilities: &[(ArtrineDecisionKind, f64)],
    artrine_physical_state: &PhysicalState,
    artrine_impulse_state: &ImpulseState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    sample_carrier_decision_from_table(
        artrine,
        table,
        utilities,
        artrine_physical_state,
        artrine_impulse_state,
        rng,
    )
}

pub fn sample_artrine_decision<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    utilities: &[(ArtrineDecisionKind, f64)],
    artrine_physical_state: &PhysicalState,
    artrine_impulse_state: &ImpulseState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    sample_carrier_decision(
        artrine,
        attribute_keys,
        utilities,
        artrine_physical_state,
        artrine_impulse_state,
        rng,
    )
}