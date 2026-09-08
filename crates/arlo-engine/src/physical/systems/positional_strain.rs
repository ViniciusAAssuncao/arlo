use crate::current_ability::profiles::get_profile_for_position;
use crate::current_ability::weights::PositionWeightProfile;
use arlo_domain::{AttributeKey, Player, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PositionalStrainProfile {
    transit_strain_multiplier: f64,
    duel_strain_multiplier: f64,
    mobility_ratio: f64,
    static_ratio: f64,
}

impl PositionalStrainProfile {
    pub fn new(
        transit_strain_multiplier: f64,
        duel_strain_multiplier: f64,
        mobility_ratio: f64,
        static_ratio: f64,
    ) -> Self {
        Self {
            transit_strain_multiplier,
            duel_strain_multiplier,
            mobility_ratio,
            static_ratio,
        }
    }

    pub fn transit_strain_multiplier(&self) -> f64 {
        self.transit_strain_multiplier
    }

    pub fn duel_strain_multiplier(&self) -> f64 {
        self.duel_strain_multiplier
    }

    pub fn mobility_ratio(&self) -> f64 {
        self.mobility_ratio
    }

    pub fn static_ratio(&self) -> f64 {
        self.static_ratio
    }
}

impl Default for PositionalStrainProfile {
    fn default() -> Self {
        Self {
            transit_strain_multiplier: 1.0,
            duel_strain_multiplier: 1.0,
            mobility_ratio: 0.25,
            static_ratio: 0.25,
        }
    }
}

pub fn is_mobility_attribute(key: AttributeKey) -> bool {
    matches!(
        key,
        AttributeKey::Pace
            | AttributeKey::Acceleration
            | AttributeKey::Agility
            | AttributeKey::Stamina
            | AttributeKey::WorkRate
            | AttributeKey::Dribbling
    )
}

pub fn is_static_attribute(key: AttributeKey) -> bool {
    matches!(
        key,
        AttributeKey::Strength
            | AttributeKey::ControlledAggression
            | AttributeKey::Bravery
            | AttributeKey::OffensiveBlocking
            | AttributeKey::DefensiveContainment
            | AttributeKey::PasserPressure
            | AttributeKey::Balance
            | AttributeKey::JumpingReach
    )
}

pub fn calculate_positional_strain(profile: &PositionWeightProfile) -> PositionalStrainProfile {
    let mut total_weight = 0.0;
    let mut mobility_weight = 0.0;
    let mut static_weight = 0.0;

    for w in &profile.weights {
        if w.weight > 0.0 {
            total_weight += w.weight;
            if is_mobility_attribute(w.key) {
                mobility_weight += w.weight;
            }
            if is_static_attribute(w.key) {
                static_weight += w.weight;
            }
        }
    }

    if total_weight <= 0.0 {
        return PositionalStrainProfile::default();
    }

    let mobility_ratio = mobility_weight / total_weight;
    let static_ratio = static_weight / total_weight;

    let transit_strain_multiplier = (0.75 + 0.60 * mobility_ratio).clamp(0.70, 1.35);
    let duel_strain_multiplier = (0.75 + 0.60 * static_ratio).clamp(0.70, 1.35);

    PositionalStrainProfile::new(
        transit_strain_multiplier,
        duel_strain_multiplier,
        mobility_ratio,
        static_ratio,
    )
}

pub fn calculate_strain_for_position(position: Position) -> PositionalStrainProfile {
    let profile = get_profile_for_position(position);
    calculate_positional_strain(&profile)
}

pub fn calculate_player_positional_strain(
    _player: &Player,
    position: Position,
) -> PositionalStrainProfile {
    calculate_strain_for_position(position)
}

pub fn calculate_transit_strain_multiplier(position: Position) -> f64 {
    calculate_strain_for_position(position).transit_strain_multiplier()
}

pub fn calculate_duel_strain_multiplier(position: Position) -> f64 {
    calculate_strain_for_position(position).duel_strain_multiplier()
}
