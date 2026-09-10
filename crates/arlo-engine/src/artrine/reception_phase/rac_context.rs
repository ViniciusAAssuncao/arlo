use crate::attributes::PlayerAttributeTable;
use crate::physical::FatigueState;
use crate::resolution::DuelContext;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition, SlotRole};
use arlo_math::units::{Length, Position as VectorPosition, MIRIM_TO_METERS};
use arlo_tactics::PlayerInstructions;
use std::collections::HashMap;
use uuid::Uuid;

pub struct RacContext<'a, F> {
    pub receiver: &'a Player,
    pub receiver_pos_domain: DomainPosition,
    pub offense_helpers: &'a [&'a Player],
    pub offense_position_index: &'a HashMap<Uuid, DomainPosition>,
    pub offense_role_index: &'a HashMap<Uuid, SlotRole>,
    pub defenders: &'a [&'a Player],
    pub defense_position_index: &'a HashMap<Uuid, DomainPosition>,
    pub defense_instructions_index: &'a HashMap<Uuid, PlayerInstructions>,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub attribute_tables: &'a HashMap<Uuid, PlayerAttributeTable>,
    pub pitch: &'a Pitch,
    pub spatial_map: &'a DynamicSpatialMap,
    pub defense_team_id: Uuid,
    pub duel_context: &'a DuelContext,
    pub fatigue_for: &'a F,
    pub defense_pressing_multiplier: f64,
}

impl<'a, F> RacContext<'a, F>
where
    F: Fn(&Uuid) -> FatigueState,
{
    pub fn receiver_pos_vec(&self) -> VectorPosition {
        self.spatial_map
            .get_position(&self.receiver.id())
            .unwrap_or_else(VectorPosition::zero)
    }

    pub fn contest_radius(&self) -> Length {
        Length::new(
            PROXIMITY_CONTEST_RADIUS_MIRIM * self.defense_pressing_multiplier * MIRIM_TO_METERS,
        )
    }

    pub fn fatigue(&self, player_id: &Uuid) -> FatigueState {
        (self.fatigue_for)(player_id)
    }

    pub fn blocker_helpers(&self) -> Vec<&'a Player> {
        let blockers: Vec<&Player> = self
            .offense_helpers
            .iter()
            .copied()
            .filter(|p| self.offense_role_index.get(&p.id()) == Some(&SlotRole::Blocker))
            .collect();

        if blockers.is_empty() {
            self.offense_helpers.to_vec()
        } else {
            blockers
        }
    }
}