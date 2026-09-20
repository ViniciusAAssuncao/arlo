use crate::attributes::PlayerAttributeTable;
use crate::physical::models::energy_decay::calculate_event_energy_decay;
use crate::physical::tuning::EnergyTuningProfile;
use crate::physical::{fatigue_multiplier, FatigueState};
use arlo_domain::{AttributeKey, Player, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct FatigueLookup<'a> {
    home_fatigue: &'a HashMap<Uuid, FatigueState>,
    away_fatigue: &'a HashMap<Uuid, FatigueState>,
}

impl<'a> FatigueLookup<'a> {
    pub fn new(
        home_fatigue: &'a HashMap<Uuid, FatigueState>,
        away_fatigue: &'a HashMap<Uuid, FatigueState>,
    ) -> Self {
        Self {
            home_fatigue,
            away_fatigue,
        }
    }

    pub fn get(&self, player_id: &Uuid) -> FatigueState {
        self.home_fatigue
            .get(player_id)
            .or_else(|| self.away_fatigue.get(player_id))
            .copied()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FatigueTracker {
    home_fatigue: HashMap<Uuid, FatigueState>,
    away_fatigue: HashMap<Uuid, FatigueState>,
}

impl FatigueTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup(&self) -> FatigueLookup<'_> {
        FatigueLookup::new(&self.home_fatigue, &self.away_fatigue)
    }

    pub fn home_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        &self.home_fatigue
    }

    pub fn away_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        &self.away_fatigue
    }

    pub fn fatigue_for(&self, player_id: &Uuid) -> FatigueState {
        self.home_fatigue
            .get(player_id)
            .or_else(|| self.away_fatigue.get(player_id))
            .copied()
            .unwrap_or_default()
    }

    pub fn set_player_fatigue(&mut self, player_id: Uuid, is_home: bool, state: FatigueState) {
        if is_home {
            self.home_fatigue.insert(player_id, state);
        } else {
            self.away_fatigue.insert(player_id, state);
        }
    }

    pub fn substitute_player(&mut self, _outgoing: Uuid, incoming: Uuid, is_home: bool) {
        let map = if is_home {
            &mut self.home_fatigue
        } else {
            &mut self.away_fatigue
        };
        if !map.contains_key(&incoming) {
            map.insert(incoming, FatigueState::default());
        }
    }

    pub fn apply_event_energy_decay(
        &mut self,
        player_id: Uuid,
        live_duration_seconds: f64,
        is_home: bool,
        table: &PlayerAttributeTable,
        age_years: f64,
        position: Position,
        tempo_mult: f64,
        pressing_mult: f64,
        participated: bool,
        profile: &EnergyTuningProfile,
    ) -> (f64, f64) {
        let stamina = table.get(AttributeKey::Stamina);
        let natural_fitness = table.get(AttributeKey::NaturalFitness);
        let workload = crate::injury::exposure::position_workload_multiplier(position);
        let fatigue = if is_home {
            self.home_fatigue.entry(player_id).or_default()
        } else {
            self.away_fatigue.entry(player_id).or_default()
        };

        let decay = calculate_event_energy_decay(
            live_duration_seconds,
            stamina,
            natural_fitness,
            age_years,
            workload,
            tempo_mult,
            pressing_mult,
            participated,
            profile,
        );

        crate::physical::models::energy_decay::apply_event_energy_decay(fatigue, decay);
        (fatigue.energy(), fatigue.w_prime_balance())
    }

    pub fn apply_contest_strain(
        &mut self,
        player_id: Uuid,
        intensity: f64,
        is_home: bool,
        table: &PlayerAttributeTable,
    ) -> (f64, f64) {
        let strength = table.get(AttributeKey::Strength);
        let acceleration = table.get(AttributeKey::Acceleration);
        let fatigue = if is_home {
            self.home_fatigue.entry(player_id).or_default()
        } else {
            self.away_fatigue.entry(player_id).or_default()
        };
        crate::physical::models::anaerobic::apply_contest_reserve_cost(
            fatigue,
            intensity,
            strength,
            acceleration,
        );
        (fatigue.energy(), fatigue.w_prime_balance())
    }

    pub fn apply_dead_ball_recovery(
        &mut self,
        dead_ball_seconds: f64,
        home_players: &[&Player],
        away_players: &[&Player],
        attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
        is_time_call: bool,
        profile: &EnergyTuningProfile,
    ) -> Vec<(Uuid, f64, f64)> {
        let mut previous_balances = HashMap::new();
        for p in home_players.iter().chain(away_players.iter()) {
            previous_balances.insert(p.id(), self.fatigue_for(&p.id()).w_prime_balance());
        }

        crate::physical::systems::recovery::apply_intra_match_recovery(
            &mut self.home_fatigue,
            &mut self.away_fatigue,
            home_players,
            away_players,
            attribute_tables,
            dead_ball_seconds,
            is_time_call,
            profile,
        );

        let mut results = Vec::new();
        for p in home_players.iter().chain(away_players.iter()) {
            let pid = p.id();
            let new_bal = self.fatigue_for(&pid).w_prime_balance();
            let old_bal = previous_balances.get(&pid).copied().unwrap_or(1.0);
            let recovery_amount = (new_bal - old_bal).max(0.0);
            results.push((pid, recovery_amount, new_bal));
        }
        results
    }

    pub fn player_fatigue_multiplier(&self, player_id: &Uuid) -> f64 {
        let fatigue = self.fatigue_for(player_id);
        fatigue_multiplier(&fatigue)
    }
}
