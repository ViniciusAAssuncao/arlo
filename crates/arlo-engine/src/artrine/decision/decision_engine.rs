use crate::ai::cognitive::RiskProfile;
use crate::artrine::decision::available_decisions::available_decision_kinds;
use crate::artrine::decision::evaluator::calculate_decision_utilities;
use crate::artrine::decision::sampler::sample_artrine_decision;
pub use crate::open_play::CarrierDecisionResult as ArtrineDecisionResult;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::world_state::GameStatePressure;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{DecisionEmphasis, PassingRange};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ArtrineDecisionRequest<'a> {
    pub artrine: &'a Player,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub normalized_proximity: f64,
    pub drives_in_current_series: u32,
    pub remaining_downs: u8,
    pub pass_protection_net_advantage: f64,
    pub is_last_down: bool,
    pub is_bonus_phase: bool,
    pub territory_advance_mirim: f64,
    pub best_available_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub artrine_pos: VectorPosition,
    pub next_artro_pos: VectorPosition,
    pub pitch_control_ahead: f64,
    pub pitch_length_mirim: f64,
    pub offensive_gravity: f64,
    pub passing_range: PassingRange,
    pub risk_profile: RiskProfile,
    pub game_state_pressure: GameStatePressure,
    pub play_call_emphasis: DecisionEmphasis,
    pub artrine_physical_state: PhysicalState,
    pub artrine_impulse_state: ImpulseState,
    pub expected_free_path_mirim: f64,
}

impl<'a> ArtrineDecisionRequest<'a> {
    pub fn new(
        artrine: &'a Player,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        normalized_proximity: f64,
        drives_in_current_series: u32,
        remaining_downs: u8,
        pass_protection_net_advantage: f64,
        is_last_down: bool,
        is_bonus_phase: bool,
        territory_advance_mirim: f64,
        best_available_target_weight: f64,
        long_launch_target_weight: f64,
        artrine_pos: VectorPosition,
        next_artro_pos: VectorPosition,
        pitch_control_ahead: f64,
        pitch_length_mirim: f64,
        offensive_gravity: f64,
        passing_range: PassingRange,
        play_call_emphasis: DecisionEmphasis,
        artrine_physical_state: PhysicalState,
    ) -> Self {
        let baseline = calculate_player_impulse_baseline(artrine, attribute_keys);
        let artrine_impulse_state = ImpulseState::from_baseline(baseline);
        let risk_profile = RiskProfile::from_player_with_impulse(
            artrine,
            attribute_keys,
            &artrine_physical_state,
            &artrine_impulse_state,
        );
        let game_state_pressure = GameStatePressure::default();
        Self {
            artrine,
            attribute_keys,
            normalized_proximity,
            drives_in_current_series,
            remaining_downs,
            pass_protection_net_advantage,
            is_last_down,
            is_bonus_phase,
            territory_advance_mirim,
            best_available_target_weight,
            long_launch_target_weight,
            artrine_pos,
            next_artro_pos,
            pitch_control_ahead,
            pitch_length_mirim,
            offensive_gravity,
            passing_range,
            risk_profile,
            game_state_pressure,
            play_call_emphasis,
            artrine_physical_state,
            artrine_impulse_state,
            expected_free_path_mirim: 0.0,
        }
    }

    pub fn with_impulse_state(mut self, impulse_state: ImpulseState) -> Self {
        self.risk_profile = RiskProfile::from_player_with_impulse(
            self.artrine,
            self.attribute_keys,
            &self.artrine_physical_state,
            &impulse_state,
        );
        self.artrine_impulse_state = impulse_state;
        self
    }

    pub fn with_risk_profile(mut self, risk_profile: RiskProfile) -> Self {
        self.risk_profile = risk_profile;
        self
    }

    pub fn with_game_state_pressure(mut self, game_state_pressure: GameStatePressure) -> Self {
        self.game_state_pressure = game_state_pressure;
        self
    }

    pub fn with_expected_free_path_mirim(mut self, expected_free_path_mirim: f64) -> Self {
        self.expected_free_path_mirim = expected_free_path_mirim;
        self
    }
}

pub fn resolve_artrine_decision<R: Rng + ?Sized>(
    request: ArtrineDecisionRequest<'_>,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let available_kinds = available_decision_kinds(
        request.drives_in_current_series,
        request.territory_advance_mirim,
        request.is_last_down,
        request.is_bonus_phase,
    );

    let utilities = calculate_decision_utilities(
        request.artrine,
        request.attribute_keys,
        &available_kinds,
        request.normalized_proximity,
        request.drives_in_current_series,
        request.remaining_downs,
        request.pass_protection_net_advantage,
        request.is_last_down,
        request.territory_advance_mirim,
        request.best_available_target_weight,
        request.long_launch_target_weight,
        request.artrine_pos,
        request.next_artro_pos,
        request.pitch_control_ahead,
        request.pitch_length_mirim,
        request.offensive_gravity,
        request.passing_range,
        request.risk_profile,
        request.game_state_pressure,
        request.play_call_emphasis,
        &request.artrine_physical_state,
        request.expected_free_path_mirim,
    );

    let result = sample_artrine_decision(
        request.artrine,
        request.attribute_keys,
        &utilities,
        &request.artrine_physical_state,
        &request.artrine_impulse_state,
        rng,
    );

    crate::psychology::systems::instrumentation::instrument_artrine_decision(
        request.artrine.id(),
        &result,
    );

    result
}