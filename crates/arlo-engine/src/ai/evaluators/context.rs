use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::attributes::profiles::AttributeProfile as DuelProfile;
use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::resolution::duel_noise::player_noise_distribution_from_table_with_impulse;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, ArtroPlacement, AttributeKey, Player, Position, SlotRole};
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DecisionEvaluationContext<'a> {
    pub carrier: &'a Player,
    pub carrier_table: &'a PlayerAttributeTable,
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
    pub team_advantage: f64,
    pub channel: ArtroPlacement,
    pub pitch_control_ahead: f64,
    pub pitch_length_mirim: f64,
    pub pitch_width_mirim: f64,
    pub offensive_gravity: f64,
    pub passing_range: PassingRange,
    pub risk_profile: RiskProfile,
    pub game_state_pressure: GameStatePressure,
    pub play_call_emphasis: DecisionEmphasis,
    pub is_true_artrine: bool,
    pub expected_free_path_mirim: f64,
    pub opponent_epa_at_proximity: f64,
    pub cached_probability_bounds: (f64, f64),
}

impl<'a> DecisionEvaluationContext<'a> {
    pub fn new(
        carrier: &'a Player,
        carrier_table: &'a PlayerAttributeTable,
        carrier_position: Position,
        carrier_role: SlotRole,
        carrier_instructions: PlayerInstructions,
        carrier_physical_state: PhysicalState,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        epv_model: DynamicEpvModel,
        current_epv: f64,
        normalized_proximity: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
        pass_protection_net_advantage: f64,
        best_available_target_weight: f64,
        long_launch_target_weight: f64,
        team_advantage: f64,
        channel: ArtroPlacement,
        offensive_gravity: f64,
        passing_range: PassingRange,
        risk_profile: RiskProfile,
        game_state_pressure: GameStatePressure,
        play_call_emphasis: DecisionEmphasis,
        is_true_artrine: bool,
    ) -> Self {
        let pitch_length_mirim = 145.0;
        let pitch_width_mirim = 85.0;

        let control_val = (0.50 + 0.04 * team_advantage - 0.08 * normalized_proximity
            + 0.04 * game_state_pressure.urgency_index())
        .clamp(0.15, 0.85);
        let pitch_control_ahead = control_val;

        let expected_free_path_mirim =
            (pitch_control_ahead * (1.0 - normalized_proximity) * 35.0).clamp(1.5, 25.0);

        let opponent_epa_at_proximity = epv_model.opponent_epa(normalized_proximity);
        let cached_probability_bounds = Self::calculate_probability_bounds(
            carrier,
            carrier_table,
            &carrier_physical_state,
        );

        Self {
            carrier,
            carrier_table,
            carrier_position,
            carrier_role,
            carrier_instructions,
            carrier_physical_state,
            attribute_keys,
            epv_model,
            current_epv,
            normalized_proximity,
            drives_in_series,
            down,
            remaining_advance_mirim,
            pass_protection_net_advantage,
            best_available_target_weight,
            long_launch_target_weight,
            team_advantage,
            channel,
            pitch_control_ahead,
            pitch_length_mirim,
            pitch_width_mirim,
            offensive_gravity,
            passing_range,
            risk_profile,
            game_state_pressure,
            play_call_emphasis,
            is_true_artrine,
            expected_free_path_mirim,
            opponent_epa_at_proximity,
            cached_probability_bounds,
        }
    }

    pub fn calculate_probability_bounds(
        carrier: &Player,
        table: &PlayerAttributeTable,
        physical_state: &PhysicalState,
    ) -> (f64, f64) {
        let deg_ctx = DegradationContext::new(physical_state);
        let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, &deg_ctx);
        let profile = crate::caching::impulse_baseline_profile();
        let baseline = calculate_player_impulse_baseline(table, profile);
        let impulse_state = ImpulseState::from_baseline(baseline);
        let noise_params = player_noise_distribution_from_table_with_impulse(
            carrier,
            table,
            physical_state,
            &impulse_state,
            baseline,
        );
        let scale = noise_params.scale();
        let norm_consistency = (consistency.clamp(0.0, 20.0)) / 20.0;
        let floor =
            (0.001 + 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.0001, 0.15);
        let ceiling =
            (0.999 - 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.85, 0.9999);
        (floor, ceiling)
    }

    pub fn carrier(&self) -> &'a Player {
        self.carrier
    }

    pub fn carrier_table(&self) -> &'a PlayerAttributeTable {
        self.carrier_table
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
        self.pitch_control_ahead.clamp(0.0, 1.0)
    }

    pub fn expected_free_path(&self) -> f64 {
        self.expected_free_path_mirim
    }

    pub fn carrier_rating(&self, profile: &DuelProfile) -> f64 {
        calculate_player_duel_rating_from_table(
            self.carrier,
            self.carrier_position,
            self.carrier_table,
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
        let deg_ctx = DegradationContext::new(&self.carrier_physical_state);
        extract_effective_attribute_value(
            self.carrier_table,
            AttributeKey::Consistency,
            &deg_ctx,
        )
    }

    pub fn probability_bounds(&self) -> (f64, f64) {
        self.cached_probability_bounds
    }

    pub fn bound_probability(&self, raw_p: f64) -> f64 {
        let (floor, ceiling) = self.probability_bounds();
        raw_p.clamp(floor, ceiling)
    }

    pub fn opponent_epa(&self) -> f64 {
        self.opponent_epa_at_proximity
    }

    pub fn is_lateral(&self) -> bool {
        self.channel != ArtroPlacement::Central
    }

    pub fn lateral_ratio(&self) -> f64 {
        if self.is_lateral() {
            0.85
        } else {
            0.0
        }
    }

    pub fn shooting_angle_factor(&self) -> f64 {
        if self.is_lateral() {
            0.75
        } else {
            1.0
        }
    }

    pub fn carrier_tactical_bias(&self, kind: ArtrineDecisionKind) -> f64 {
        crate::open_play::CarrierTacticalBias::calculate_bias(
            self.carrier_position,
            self.carrier_role,
            &self.carrier_instructions,
            kind,
            self.normalized_proximity,
            self.is_lateral(),
        )
    }
}
