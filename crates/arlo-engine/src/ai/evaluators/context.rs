use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::resolution::duel_profiles::DuelProfile;
use crate::resolution::group_rating::calculate_player_duel_rating;
use crate::world_state::GameStatePressure;
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DecisionEvaluationContext<'a> {
    pub artrine: &'a Player,
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
}

impl<'a> DecisionEvaluationContext<'a> {
    pub fn target_quality(&self) -> f64 {
        ((self.best_available_target_weight - 8.0) / 10.0).clamp(-0.5, 1.0)
    }

    pub fn pitch_control(&self) -> f64 {
        self.pitch_control_ahead.clamp(0.05, 0.95)
    }

    pub fn artrine_rating(&self, profile: &DuelProfile) -> f64 {
        calculate_player_duel_rating(
            self.artrine,
            Position::Artrine,
            self.attribute_keys,
            profile,
        )
    }

    pub fn skill_multiplier(&self, intrinsic_rating: f64) -> f64 {
        (intrinsic_rating / 10.0).clamp(0.5, 1.5)
    }
}