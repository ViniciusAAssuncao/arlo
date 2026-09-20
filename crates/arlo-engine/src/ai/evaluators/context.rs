use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::attributes::profiles::AttributeProfile as DuelProfile;
use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::resolution::duel_noise::player_consistency_noise_scale;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::scoring_model::ScoringDifficultyProfile;
use crate::scoring_regime::ScoringRegimePolicy;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, ArtroPlacement, AttributeKey, Player, Position, SlotRole};
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ContextSituation {
    pub drives_in_series: u32,
    pub down: u8,
    pub remaining_advance_mirim: f64,
    pub pass_protection_net_advantage: f64,
    pub target_quality: f64,
    pub long_launch_target_quality: f64,
    pub team_advantage: f64,
    pub channel: ArtroPlacement,
    pub pitch_control: f64,
    pub expected_free_path: f64,
    pub offensive_gravity: f64,
    pub passing_range: PassingRange,
    pub game_state_pressure: GameStatePressure,
    pub play_call_emphasis: DecisionEmphasis,
    pub is_true_artrine: bool,
    pub is_bonus_phase: bool,
    pub normalized_proximity: f64,
    pub pitch_length_mirim: f64,
}

#[derive(Debug, Clone)]
pub struct ContextCarrier<'a> {
    pub player: &'a Player,
    pub table: &'a PlayerAttributeTable,
    pub position: Position,
    pub role: SlotRole,
    pub instructions: PlayerInstructions,
    pub physical_state: PhysicalState,
    pub probability_bounds: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct DecisionEvaluationContext<'a> {
    pub carrier: ContextCarrier<'a>,
    pub situation: ContextSituation,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub epv_model: DynamicEpvModel,
    pub current_epv: f64,
    pub risk_profile: RiskProfile,
    pub scoring_difficulty: ScoringDifficultyProfile,
    pub scoring_regime: ScoringRegimePolicy,
}

impl<'a> DecisionEvaluationContext<'a> {
    pub fn new(
        carrier: ContextCarrier<'a>,
        situation: ContextSituation,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        epv_model: DynamicEpvModel,
        current_epv: f64,
        risk_profile: RiskProfile,
        scoring_regime: ScoringRegimePolicy,
    ) -> Self {
        let scoring_difficulty = epv_model.difficulty_profile();

        Self {
            carrier,
            situation,
            attribute_keys,
            epv_model,
            current_epv,
            risk_profile,
            scoring_difficulty,
            scoring_regime,
        }
    }

    pub fn with_scoring_difficulty(mut self, scoring_difficulty: ScoringDifficultyProfile) -> Self {
        self.scoring_difficulty = scoring_difficulty;
        self
    }

    pub fn calculate_probability_bounds(
        table: &PlayerAttributeTable,
        physical_state: &PhysicalState,
    ) -> (f64, f64) {
        let deg_ctx = DegradationContext::new(physical_state);
        let consistency =
            extract_effective_attribute_value(table, AttributeKey::Consistency, &deg_ctx);
        let profile = crate::caching::impulse_baseline_profile();
        let baseline = calculate_player_impulse_baseline(table, profile);
        let impulse_state = ImpulseState::from_baseline(baseline);
        let deg_ctx_impulse =
            DegradationContext::with_impulse(physical_state, &impulse_state, baseline);
        let scale = player_consistency_noise_scale(table, &deg_ctx_impulse);
        let norm_consistency = (consistency.clamp(0.0, 20.0)) / 20.0;
        let floor =
            (0.001 + 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.0001, 0.15);
        let ceiling =
            (0.999 - 0.049 * (1.0 - norm_consistency) * (1.0 + scale * 0.1)).clamp(0.85, 0.9999);
        (floor, ceiling)
    }

    pub fn carrier_rating(&self, profile: &DuelProfile) -> f64 {
        calculate_player_duel_rating_from_table(
            self.carrier.player,
            self.carrier.position,
            self.carrier.table,
            profile,
            &self.carrier.physical_state,
        )
    }

    pub fn bound_probability(&self, raw_p: f64) -> f64 {
        let (floor, ceiling) = self.carrier.probability_bounds;
        raw_p.clamp(floor, ceiling)
    }

    pub fn opponent_epa(&self) -> f64 {
        self.epv_model
            .opponent_epa(self.situation.normalized_proximity)
    }

    pub fn is_bonus_phase(&self) -> bool {
        self.situation.is_bonus_phase
    }

    pub fn calculate_epa(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        self.epv_model.calculate_epa(
            normalized_x,
            down,
            remaining_advance_mirim,
            drives_in_series,
            self.situation.is_bonus_phase,
            &self.scoring_regime,
        )
    }

    pub fn is_lateral(&self) -> bool {
        self.situation.channel != ArtroPlacement::Central
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

    pub fn emphasis_for(&self, kind: ArtrineDecisionKind) -> f64 {
        match kind {
            ArtrineDecisionKind::SelfCarry => {
                self.situation.play_call_emphasis.self_carry().value()
            }
            ArtrineDecisionKind::ShortPass => {
                self.situation.play_call_emphasis.short_pass().value()
            }
            ArtrineDecisionKind::LongLaunch => {
                self.situation.play_call_emphasis.long_launch().value()
            }
            ArtrineDecisionKind::Cross => self.situation.play_call_emphasis.cross().value(),
            ArtrineDecisionKind::SelfFinish => {
                self.situation.play_call_emphasis.self_finish().value()
            }
        }
    }

    pub fn carrier_tactical_bias(&self, kind: ArtrineDecisionKind) -> f64 {
        crate::open_play::CarrierTacticalBias::calculate_bias(
            self.carrier.position,
            self.carrier.role,
            &self.carrier.instructions,
            kind,
            self.situation.normalized_proximity,
            self.is_lateral(),
        )
    }
}
