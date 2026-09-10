use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::{select_target_from_tables, ReceptionRole};
use crate::physical::PhysicalState;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn select_kicker(
    candidates: &[&Player],
    role_index_for_play: Option<&HashMap<Uuid, SlotRole>>,
) -> Option<Uuid> {
    if let Some(roles) = role_index_for_play {
        if let Some(kicker) = candidates
            .iter()
            .find(|p| roles.get(&p.id()) == Some(&SlotRole::Kicker))
        {
            return Some(kicker.id());
        }
    }
    None
}

pub fn select_kicker_from_tables<F, R>(
    candidates: &[&Player],
    role_index_for_play: Option<&HashMap<Uuid, SlotRole>>,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
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
    if let Some(kicker_id) = select_kicker(candidates, role_index_for_play) {
        return Some(kicker_id);
    }

    select_target_from_tables(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_tables,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
        fatigue_for,
        rng,
    )
}