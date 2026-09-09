use crate::physical::FatigueState;
use crate::resolution::DuelContext;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition, SlotRole};
use arlo_math::units::{Length, MIRIM_TO_METERS};
use arlo_tactics::{PlayerInstructions, RouteAssignment, TeamInstructions};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ActionExecutionContext<'a, F> {
    pub pitch: &'a Pitch,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub attacking_positive_x: bool,
    pub drives_in_series: u32,
    pub accumulated_advance_mirim: f64,
    pub is_last_down: bool,
    pub is_bonus_phase: bool,
    pub defense_pressing_multiplier: f64,
    pub offense_tempo_value: f64,
    pub duel_context: &'a DuelContext,
    pub fatigue_for: &'a F,
    pub offense_helpers: &'a [&'a Player],
    pub offense_position_index: &'a HashMap<Uuid, DomainPosition>,
    pub offense_role_index: &'a HashMap<Uuid, SlotRole>,
    pub offense_instructions_index: &'a HashMap<Uuid, PlayerInstructions>,
    pub offense_instructions: &'a TeamInstructions,
    pub defenders: &'a [&'a Player],
    pub defense_position_index: &'a HashMap<Uuid, DomainPosition>,
    pub defense_instructions_index: &'a HashMap<Uuid, PlayerInstructions>,
    pub goalguard: &'a Player,
    pub openness_by_player: &'a HashMap<Uuid, f64>,
    pub offense_route_index: &'a HashMap<Uuid, RouteAssignment>,
}

impl<'a, F> ActionExecutionContext<'a, F>
where
    F: Fn(&Uuid) -> FatigueState,
{
    pub fn fatigue(&self, player_id: &Uuid) -> FatigueState {
        (self.fatigue_for)(player_id)
    }

    pub fn contest_radius(&self) -> Length {
        Length::new(
            PROXIMITY_CONTEST_RADIUS_MIRIM * self.defense_pressing_multiplier * MIRIM_TO_METERS,
        )
    }
}
