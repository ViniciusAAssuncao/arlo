use crate::attributes::PlayerAttributeTable;
use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::fit_calculator::calculate_fit_for_position;
use crate::physical::PhysicalState;
use arlo_domain::sport_constants::MIN_CURRENT_ABILITY;
use arlo_domain::{Player, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerContribution {
    current_ability: f64,
    positional_efficiency: f64,
    fatigue_efficiency: f64,
    value: f64,
}

impl PlayerContribution {
    pub fn new(ca: f64, fit: f64, state: &PhysicalState) -> Self {
        let positional_efficiency = 0.35 + 0.65 * fit.clamp(0.0, 1.0);
        let energy = state.energy().clamp(0.0, 1.0);
        let w_prime = state.w_prime_balance().clamp(0.0, 1.0);
        let combined_vitality = 0.65 * energy + 0.35 * w_prime;
        let fatigue_efficiency = (0.50 + 0.50 * combined_vitality).clamp(0.40, 1.00);
        let value = ca * positional_efficiency * fatigue_efficiency;

        Self {
            current_ability: ca,
            positional_efficiency,
            fatigue_efficiency,
            value,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn current_ability(&self) -> f64 {
        self.current_ability
    }

    pub fn positional_efficiency(&self) -> f64 {
        self.positional_efficiency
    }

    pub fn fatigue_efficiency(&self) -> f64 {
        self.fatigue_efficiency
    }
}

pub fn calculate_player_contribution(
    player: &Player,
    table: &PlayerAttributeTable,
    target_position: Position,
    state: &PhysicalState,
) -> PlayerContribution {
    let ca = calculate_player_ca(player, table).unwrap_or(MIN_CURRENT_ABILITY) as f64;
    let fit = calculate_fit_for_position(player, target_position).efficiency_multiplier();
    PlayerContribution::new(ca, fit, state)
}