use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use crate::resolution::duel_noise::player_noise_distribution;
use crate::resolution::duel_profiles::DuelProfile;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position, SlotRole};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DecisionEvaluationContext<'a> {
    pub carrier: &'a Player,
    pub carrier_position: Position,
    pub carrier_role: SlotRole,
    pub carrier_instructions: PlayerInstructions,
    pub carrier_physical_state: PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub epv_model: DynamicEpvModel,
    pub current_epv: f64,
    pub normalized_proximity: f64,
    pub drives_in_series: u32,
    pub down: u8,
    pub remaining_advance_mirim: f64,
    pub pass_protection_net_advantage: f64,
    pub best_available_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub pitch_control_ahead: f64,
    pub distance_to_next_artro_mirim: f64,
    pub pitch_length_mirim: f64,
    pub pitch_width_mirim: f64,
    pub carrier_pos_vec: VectorPosition,
    pub offensive_gravity: f64,
    pub passing_range: PassingRange,
    pub risk_profile: RiskProfile,
    pub game_state_pressure: GameStatePressure,
    pub play_call_emphasis: DecisionEmphasis,
    pub is_true_artrine: bool,
}

impl<'a> DecisionEvaluationContext<'a> {
    pub fn carrier(&self) -> &'a Player {
        self.carrier
    }

    pub fn artrine(&self) -> &'a Player {
        self.carrier
    }

    pub fn target_quality(&self) -> f64 {
        (self.best_available_target_weight - 8.0) / 10.0
    }

    pub fn long_launch_target_quality(&self) -> f64 {
        (self.long_launch_target_weight - 8.0) / 10.0
    }

    pub fn pitch_control(&self) -> f64 {
        self.pitch_control_ahead.max(0.0).min(1.0)
    }

    pub fn carrier_rating(&self, profile: &DuelProfile) -> f64 {
        calculate_player_duel_rating_with_state(
            self.carrier,
            self.carrier_position,
            self.attribute_keys,
            profile,
            &self.carrier_physical_state,
        )
    }

    pub fn artrine_rating(&self, profile: &DuelProfile) -> f64 {
        self.carrier_rating(profile)
    }

    pub fn skill_multiplier(&self, intrinsic_rating: f64) -> f64 {
        intrinsic_rating / 10.0
    }

    pub fn consistency(&self) -> f64 {
        extract_effective_attribute_value(
            self.carrier,
            self.attribute_keys,
            AttributeKey::Consistency,
            &self.carrier_physical_state,
        )
    }

    pub fn probability_bounds(&self) -> (f64, f64) {
        let consistency = self.consistency();
        let noise_params = player_noise_distribution(
            self.carrier,
            self.attribute_keys,
            &self.carrier_physical_state,
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

    pub fn carrier_tactical_bias(&self, kind: ArtrineDecisionKind) -> f64 {
        let center_y_m = (self.pitch_width_mirim * 0.5) * MIRIM_TO_METERS;
        let dist_from_center_m = (self.carrier_pos_vec.raw().1 - center_y_m).abs();
        let is_lateral = dist_from_center_m > (self.pitch_width_mirim * 0.20 * MIRIM_TO_METERS);
        crate::open_play::CarrierTacticalBias::calculate_bias(
            self.carrier_position,
            self.carrier_role,
            &self.carrier_instructions,
            kind,
            self.normalized_proximity,
            is_lateral,
        )
    }
}