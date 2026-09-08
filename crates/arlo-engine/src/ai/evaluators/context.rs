use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use crate::resolution::duel_noise::player_noise_distribution;
use crate::resolution::duel_profiles::DuelProfile;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::team_identity::TeamIdentityBias;
use crate::world_state::GameStatePressure;
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DecisionEvaluationContext<'a> {
    pub artrine: &'a Player,
    pub artrine_physical_state: PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub epv_model: DynamicEpvModel,
    pub current_epv: f64,
    pub normalized_proximity: f64,
    pub drives_in_series: u32,
    pub down: u8,
    pub remaining_advance_mirim: f64,
    pub pass_protection_net_advantage: f64,
    pub best_available_target_weight: f64,
    pub pitch_control_ahead: f64,
    pub distance_to_next_artro_mirim: f64,
    pub pitch_length_mirim: f64,
    pub offensive_gravity: f64,
    pub risk_profile: RiskProfile,
    pub game_state_pressure: GameStatePressure,
    pub team_identity_bias: TeamIdentityBias,
}

impl<'a> DecisionEvaluationContext<'a> {
    pub fn target_quality(&self) -> f64 {
        (self.best_available_target_weight - 8.0) / 10.0
    }

    pub fn pitch_control(&self) -> f64 {
        self.pitch_control_ahead.max(0.0).min(1.0)
    }

    pub fn artrine_rating(&self, profile: &DuelProfile) -> f64 {
        calculate_player_duel_rating_with_state(
            self.artrine,
            Position::Artrine,
            self.attribute_keys,
            profile,
            &self.artrine_physical_state,
        )
    }

    pub fn skill_multiplier(&self, intrinsic_rating: f64) -> f64 {
        intrinsic_rating / 10.0
    }

    pub fn consistency(&self) -> f64 {
        extract_effective_attribute_value(
            self.artrine,
            self.attribute_keys,
            AttributeKey::Consistency,
            &self.artrine_physical_state,
        )
    }

    pub fn probability_bounds(&self) -> (f64, f64) {
        let consistency = self.consistency();
        let noise_params = player_noise_distribution(
            self.artrine,
            self.attribute_keys,
            &self.artrine_physical_state,
        );
        let scale = noise_params.scale();
        let norm_consistency = (consistency.clamp(0.0, 20.0)) / 20.0;
        let floor =
            (0.001 + 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.0001, 0.15);
        let ceiling =
            (0.999 - 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.85, 0.9999);
        (floor, ceiling)
    }

    pub fn bound_probability(&self, raw_p: f64) -> f64 {
        let (floor, ceiling) = self.probability_bounds();
        raw_p.clamp(floor, ceiling)
    }
}
