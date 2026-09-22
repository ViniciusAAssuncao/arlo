use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::{select_target, ReceptionRole};
use crate::physical::PhysicalState;
use arlo_domain::{Pitch, Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn select_next_holder<'a, F, R>(
    candidates: &[&'a Player],
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    select_target(
        candidates,
        pitch,
        position_index,
        instructions_index,
        role_index,
        attribute_tables,
        attacking_positive_x,
        ReceptionRole::ContinuationReceiver,
        openness_by_player,
        fatigue_for,
        rng,
    )
}
