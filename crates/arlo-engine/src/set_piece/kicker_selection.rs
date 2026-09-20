use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::select_finisher;
use crate::physical::PhysicalState;
use arlo_domain::{Pitch, Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn select_kicker<'a, F, R>(
    candidates: &[&'a Player],
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<&'a Player>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    let empty_openness = HashMap::new();
    let selected_id = select_finisher(
        candidates,
        role_index,
        pitch,
        position_index,
        instructions_index,
        attribute_tables,
        attacking_positive_x,
        &empty_openness,
        fatigue_for,
        rng,
    )?;
    candidates.iter().copied().find(|p| p.id() == selected_id)
}
