use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::Lineup;
use crate::physical::systems::degradation::{
    calculate_physical_exhaustion, extract_effective_attribute_value, DegradationContext,
};
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::{
    calculate_captaincy_influence, calculate_player_contextual_baseline, find_active_captain,
};
use crate::psychology::systems::dynamics::{calculate_impulse_recovery_tau, update_impulse};
use crate::psychology::systems::events::{
    apply_impulse_event, ImpulseEvent, ImpulseShift, PlayerImpulseContext,
};
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::{AttributeKey, Player};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpulseTracker {
    home_impulse: HashMap<Uuid, ImpulseState>,
    away_impulse: HashMap<Uuid, ImpulseState>,
}

impl ImpulseTracker {
    pub fn new(
        home_lineup: &Lineup,
        away_lineup: &Lineup,
        tables: &HashMap<Uuid, PlayerAttributeTable>,
    ) -> Self {
        let home_players: Vec<&Player> = home_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        let away_players: Vec<&Player> = away_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();

        let home_captain = find_active_captain(&home_players);
        let away_captain = find_active_captain(&away_players);

        let home_captain_influence = home_captain
            .and_then(|c| tables.get(&c.id()))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);

        let away_captain_influence = away_captain
            .and_then(|c| tables.get(&c.id()))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);

        let mut home_impulse = HashMap::with_capacity(home_lineup.len());
        for p in &home_players {
            let is_cap = home_captain.map(|c| c.id() == p.id()).unwrap_or(false);
            let table = tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let base = calculate_player_contextual_baseline(table, home_captain_influence, is_cap, true);
            home_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }

        let mut away_impulse = HashMap::with_capacity(away_lineup.len());
        for p in &away_players {
            let is_cap = away_captain.map(|c| c.id() == p.id()).unwrap_or(false);
            let table = tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let base = calculate_player_contextual_baseline(table, away_captain_influence, is_cap, false);
            away_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }

        Self {
            home_impulse,
            away_impulse,
        }
    }

    pub fn home_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        &self.home_impulse
    }

    pub fn away_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        &self.away_impulse
    }

    pub fn home_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        &mut self.home_impulse
    }

    pub fn away_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        &mut self.away_impulse
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.home_impulse
            .get(player_id)
            .or_else(|| self.away_impulse.get(player_id))
            .copied()
            .unwrap_or_default()
    }

    pub fn set_player_impulse(&mut self, player_id: Uuid, is_home: bool, state: ImpulseState) {
        if is_home {
            self.home_impulse.insert(player_id, state);
        } else {
            self.away_impulse.insert(player_id, state);
        }
    }

    pub fn substitute_player(
        &mut self,
        _outgoing: Uuid,
        incoming: Uuid,
        is_home: bool,
        table: &PlayerAttributeTable,
        captain_influence: f64,
        is_captain: bool,
    ) {
        let map = if is_home {
            &mut self.home_impulse
        } else {
            &mut self.away_impulse
        };

        if !map.contains_key(&incoming) {
            let base = calculate_player_contextual_baseline(table, captain_influence, is_captain, is_home);
            map.insert(incoming, ImpulseState::from_baseline(base));
        }
    }

    pub fn set_all_to_initial(&mut self) {
        for state in self.home_impulse.values_mut() {
            *state = ImpulseState::initial();
        }
        for state in self.away_impulse.values_mut() {
            *state = ImpulseState::initial();
        }
    }

    pub fn reset_all_to_baseline(&mut self, teams: &TeamRegistry) {
        let home_captain_id = teams.home_captain_id();
        let home_captain_influence = home_captain_id
            .and_then(|id| teams.player_attribute_table(&id))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);

        let away_captain_id = teams.away_captain_id();
        let away_captain_influence = away_captain_id
            .and_then(|id| teams.player_attribute_table(&id))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);

        for a in teams.home_lineup().assignments() {
            let pid = a.player().id();
            let table = teams.player_attribute_table(&pid).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let is_cap = Some(pid) == home_captain_id;
            let base = calculate_player_contextual_baseline(table, home_captain_influence, is_cap, true);
            self.home_impulse.insert(pid, ImpulseState::from_baseline(base));
        }

        for a in teams.away_lineup().assignments() {
            let pid = a.player().id();
            let table = teams.player_attribute_table(&pid).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let is_cap = Some(pid) == away_captain_id;
            let base = calculate_player_contextual_baseline(table, away_captain_influence, is_cap, false);
            self.away_impulse.insert(pid, ImpulseState::from_baseline(base));
        }
    }

    pub fn advance_impulse_dynamics(
        &mut self,
        dt_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
    ) {
        if dt_seconds <= 0.0 {
            return;
        }

        for a in teams.home_lineup().assignments() {
            let pid = a.player().id();
            let phys = fatigue.fatigue_for(&pid);
            let table = teams.player_attribute_table(&pid).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let stamina = table.get(AttributeKey::Stamina);
            let fitness = table.get(AttributeKey::NaturalFitness);
            let tau = calculate_impulse_recovery_tau(stamina, fitness) * 0.95;
            let exhaustion = calculate_physical_exhaustion(&phys);
            if let Some(state) = self.home_impulse.get_mut(&pid) {
                update_impulse(state, exhaustion, dt_seconds, tau);
            }
        }

        for a in teams.away_lineup().assignments() {
            let pid = a.player().id();
            let phys = fatigue.fatigue_for(&pid);
            let table = teams.player_attribute_table(&pid).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let stamina = table.get(AttributeKey::Stamina);
            let fitness = table.get(AttributeKey::NaturalFitness);
            let tau = calculate_impulse_recovery_tau(stamina, fitness);
            let exhaustion = calculate_physical_exhaustion(&phys);
            if let Some(state) = self.away_impulse.get_mut(&pid) {
                update_impulse(state, exhaustion, dt_seconds, tau);
            }
        }
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        score_deficit: i32,
    ) -> Option<ImpulseShift> {
        let is_home = teams.is_home_player(&player_id);
        let table = teams.player_attribute_table(&player_id).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let captain_id = if is_home {
            teams.home_captain_id()
        } else {
            teams.away_captain_id()
        };
        let captain_influence = captain_id
            .and_then(|id| teams.player_attribute_table(&id))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);
        let is_captain = Some(player_id) == captain_id;
        let physical_state = fatigue.fatigue_for(&player_id);
        let exhaustion = calculate_physical_exhaustion(&physical_state);

        let deg_ctx = DegradationContext::new(&physical_state);
        let determination = extract_effective_attribute_value(table, AttributeKey::Determination, &deg_ctx);
        let bravery = extract_effective_attribute_value(table, AttributeKey::Bravery, &deg_ctx);
        let composure = extract_effective_attribute_value(table, AttributeKey::Composure, &deg_ctx);
        let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, &deg_ctx);

        let context = PlayerImpulseContext::with_deficit(
            determination,
            bravery,
            composure,
            consistency,
            exhaustion,
            captain_influence,
            is_captain,
            is_home,
            score_deficit,
        );

        let state = if is_home {
            self.home_impulse.entry(player_id).or_insert_with(|| ImpulseState::from_baseline(50.0))
        } else {
            self.away_impulse.entry(player_id).or_insert_with(|| ImpulseState::from_baseline(50.0))
        };

        Some(apply_impulse_event(state, &context, event))
    }
}