use crate::physical::{compute_player_fatigue_multiplier, FatigueState};
use arlo_domain::{AttributeKey, Player};
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

    pub fn record_distance(
        &mut self,
        player_id: Uuid,
        mirim: f64,
        is_home: bool,
        player: Option<&Player>,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> (f64, f64) {
        let fatigue = if is_home {
            self.home_fatigue.entry(player_id).or_default()
        } else {
            self.away_fatigue.entry(player_id).or_default()
        };
        fatigue.add_distance(mirim);
        if let Some(p) = player {
            crate::physical::models::aerobic::update_physical_state_aerobic(
                fatigue,
                p,
                attribute_keys,
                0,
            );
        }
        (fatigue.energy(), fatigue.w_prime_balance())
    }

    pub fn apply_duel_anaerobic_cost(
        &mut self,
        player_id: Uuid,
        duration_seconds: f64,
        intensity: f64,
        is_home: bool,
        player: Option<&Player>,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> (f64, f64) {
        if let Some(p) = player {
            let max_w = crate::physical::models::metabolic_power::calculate_player_max_w_prime(
                p,
                attribute_keys,
            );
            let crit_speed =
                crate::physical::models::metabolic_power::calculate_player_critical_speed(
                    p,
                    attribute_keys,
                    0,
                )
                .value();
            let cost = crate::physical::models::anaerobic::calculate_player_anaerobic_cost(
                p,
                attribute_keys,
                duration_seconds,
                crit_speed + 2.0,
                crit_speed,
                intensity,
            );
            let fatigue = if is_home {
                self.home_fatigue.entry(player_id).or_default()
            } else {
                self.away_fatigue.entry(player_id).or_default()
            };
            crate::physical::models::anaerobic::apply_anaerobic_cost_to_state(fatigue, cost, max_w);
            (fatigue.energy(), fatigue.w_prime_balance())
        } else {
            let fatigue = self.fatigue_for(&player_id);
            (fatigue.energy(), fatigue.w_prime_balance())
        }
    }

    pub fn apply_dead_ball_recovery(
        &mut self,
        dead_ball_seconds: f64,
        home_players: &[&Player],
        away_players: &[&Player],
        attribute_keys: &HashMap<Uuid, AttributeKey>,
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
            attribute_keys,
            dead_ball_seconds,
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

    pub fn player_fatigue_multiplier(
        &self,
        player: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> f64 {
        let fatigue = self.fatigue_for(&player.id());
        compute_player_fatigue_multiplier(player, &fatigue, attribute_keys)
    }
}