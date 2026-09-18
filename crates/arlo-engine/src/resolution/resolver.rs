use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::evaluation::evaluate_duel;
use crate::resolution::execution::execute_duel;
use crate::resolution::group_rating::{calculate_side_rating, RatingParticipants};
use crate::resolution::outcome::DuelOutcome;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{AttributeKey, Player};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct DuelResolutionRequest<'a> {
    pub kind: DuelKind,
    pub attacker_rating: f64,
    pub defender_rating: f64,
    pub attacker_primary: Option<&'a Player>,
    pub defender_primary: Option<&'a Player>,
    pub attacker_state: PhysicalState,
    pub defender_state: PhysicalState,
    pub attribute_keys: Option<&'a HashMap<Uuid, AttributeKey>>,
    pub context: &'a DuelContext,
    pub attacker_table: Option<&'a PlayerAttributeTable>,
    pub defender_table: Option<&'a PlayerAttributeTable>,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
    pub slope_override: Option<f64>,
    pub pressure: Option<GameStatePressure>,
}

pub type ContestRequest<'a> = DuelResolutionRequest<'a>;

impl<'a> DuelResolutionRequest<'a> {
    pub fn new(
        kind: DuelKind,
        attacker_rating: f64,
        defender_rating: f64,
        attacker_primary: &'a Player,
        defender_primary: &'a Player,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary: Some(attacker_primary),
            defender_primary: Some(defender_primary),
            attacker_state: PhysicalState::initial(),
            defender_state: PhysicalState::initial(),
            attribute_keys: Some(attribute_keys),
            context,
            attacker_table: None,
            defender_table: None,
            attacker_team_power: None,
            defender_team_power: None,
            slope_override: None,
            pressure: None,
        }
    }

    pub fn with_states(
        kind: DuelKind,
        attacker_rating: f64,
        defender_rating: f64,
        attacker_primary: &'a Player,
        defender_primary: &'a Player,
        attacker_state: PhysicalState,
        defender_state: PhysicalState,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary: Some(attacker_primary),
            defender_primary: Some(defender_primary),
            attacker_state,
            defender_state,
            attribute_keys: Some(attribute_keys),
            context,
            attacker_table: None,
            defender_table: None,
            attacker_team_power: None,
            defender_team_power: None,
            slope_override: None,
            pressure: None,
        }
    }

    pub fn from_participants(
        kind: DuelKind,
        attacker_primary: &'a Player,
        attackers: RatingParticipants<'a>,
        defender_primary: &'a Player,
        defenders: RatingParticipants<'a>,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        let (attacker_profile, defender_profile) = get_duel_profiles(kind);
        let attacker_rating = calculate_side_rating(attackers, attribute_keys, &attacker_profile);
        let defender_rating = calculate_side_rating(defenders, attribute_keys, &defender_profile);
        let default_state = PhysicalState::initial();
        let attacker_state = attackers
            .fatigue_lookup
            .map(|f| f(&attacker_primary.id()))
            .unwrap_or(default_state);
        let defender_state = defenders
            .fatigue_lookup
            .map(|f| f(&defender_primary.id()))
            .unwrap_or(default_state);

        let attacker_table = attackers
            .attribute_tables
            .and_then(|m| m.get(&attacker_primary.id()));
        let defender_table = defenders
            .attribute_tables
            .and_then(|m| m.get(&defender_primary.id()));

        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary: Some(attacker_primary),
            defender_primary: Some(defender_primary),
            attacker_state,
            defender_state,
            attribute_keys: Some(attribute_keys),
            context,
            attacker_table,
            defender_table,
            attacker_team_power: attackers.team_power,
            defender_team_power: defenders.team_power,
            slope_override: None,
            pressure: None,
        }
    }

    pub fn for_contest(
        kind: DuelKind,
        attacker_rating: f64,
        defender_rating: f64,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary: None,
            defender_primary: None,
            attacker_state: PhysicalState::initial(),
            defender_state: PhysicalState::initial(),
            attribute_keys: None,
            context,
            attacker_table: None,
            defender_table: None,
            attacker_team_power: None,
            defender_team_power: None,
            slope_override: None,
            pressure: None,
        }
    }

    pub fn with_tables(
        mut self,
        attacker_table: Option<&'a PlayerAttributeTable>,
        defender_table: Option<&'a PlayerAttributeTable>,
    ) -> Self {
        self.attacker_table = attacker_table;
        self.defender_table = defender_table;
        self
    }

    pub fn with_team_powers(
        mut self,
        attacker_team_power: Option<f64>,
        defender_team_power: Option<f64>,
    ) -> Self {
        self.attacker_team_power = attacker_team_power;
        self.defender_team_power = defender_team_power;
        self
    }

    pub fn with_slope(mut self, slope: f64) -> Self {
        self.slope_override = Some(slope);
        self
    }

    pub fn with_pressure(mut self, pressure: Option<GameStatePressure>) -> Self {
        self.pressure = pressure;
        self
    }
}

pub use crate::resolution::execution::calculate_velocity_mitigation;

pub fn resolve_duel<R: Rng + ?Sized>(
    request: DuelResolutionRequest<'_>,
    rng: &mut R,
) -> DuelOutcome {
    let evaluated = evaluate_duel(&request, rng);
    execute_duel(&evaluated, rng)
}

pub use resolve_duel as resolve_contest;