use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::fatigue::FatigueTracker;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SquadFatigueSummary {
    pub mean_energy: f64,
    pub mean_w_prime_balance: f64,
    pub most_fatigued_on_field: Vec<(Uuid, f64)>,
}

impl SquadFatigueSummary {
    pub fn new(
        mean_energy: f64,
        mean_w_prime_balance: f64,
        most_fatigued_on_field: Vec<(Uuid, f64)>,
    ) -> Self {
        Self {
            mean_energy,
            mean_w_prime_balance,
            most_fatigued_on_field,
        }
    }

    pub fn from_lineup_and_fatigue(lineup: &Lineup, fatigue: &FatigueTracker) -> Self {
        let players = lineup.players();
        if players.is_empty() {
            return Self {
                mean_energy: 1.0,
                mean_w_prime_balance: 1.0,
                most_fatigued_on_field: Vec::new(),
            };
        }

        let mut total_energy = 0.0;
        let mut total_w_prime = 0.0;
        let mut player_w_primes = Vec::with_capacity(players.len());

        for player in players {
            let pid = player.id();
            let state = fatigue.fatigue_for(&pid);
            total_energy += state.energy();
            total_w_prime += state.w_prime_balance();
            player_w_primes.push((pid, state.w_prime_balance()));
        }

        let count = players.len() as f64;
        let mean_energy = total_energy / count;
        let mean_w_prime_balance = total_w_prime / count;

        player_w_primes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        Self {
            mean_energy,
            mean_w_prime_balance,
            most_fatigued_on_field: player_w_primes,
        }
    }
}