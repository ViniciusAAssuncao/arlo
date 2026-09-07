use crate::physical::systems::degradation::{
    calculate_physical_exhaustion, extract_effective_attribute_value,
};
use crate::physical::PhysicalState;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RiskProfile {
    tolerance_index: f64,
    loss_aversion_lambda: f64,
    gain_diminishing_alpha: f64,
    loss_diminishing_beta: f64,
    probability_distortion_gamma: f64,
    physical_exhaustion: f64,
}

impl RiskProfile {
    pub fn new(
        tolerance_index: f64,
        loss_aversion_lambda: f64,
        gain_diminishing_alpha: f64,
        loss_diminishing_beta: f64,
        probability_distortion_gamma: f64,
    ) -> Self {
        Self {
            tolerance_index: tolerance_index.clamp(0.40, 2.50),
            loss_aversion_lambda: loss_aversion_lambda.clamp(1.10, 4.50),
            gain_diminishing_alpha: gain_diminishing_alpha.clamp(0.60, 1.00),
            loss_diminishing_beta: loss_diminishing_beta.clamp(0.60, 1.00),
            probability_distortion_gamma: probability_distortion_gamma.clamp(0.40, 1.00),
            physical_exhaustion: 0.0,
        }
    }

    pub fn with_exhaustion(
        tolerance_index: f64,
        loss_aversion_lambda: f64,
        gain_diminishing_alpha: f64,
        loss_diminishing_beta: f64,
        probability_distortion_gamma: f64,
        physical_exhaustion: f64,
    ) -> Self {
        Self {
            tolerance_index: tolerance_index.clamp(0.40, 2.50),
            loss_aversion_lambda: loss_aversion_lambda.clamp(1.10, 4.50),
            gain_diminishing_alpha: gain_diminishing_alpha.clamp(0.60, 1.00),
            loss_diminishing_beta: loss_diminishing_beta.clamp(0.60, 1.00),
            probability_distortion_gamma: probability_distortion_gamma.clamp(0.40, 1.00),
            physical_exhaustion: physical_exhaustion.clamp(0.0, 1.0),
        }
    }

    pub fn from_player(
        player: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        physical_state: &PhysicalState,
    ) -> Self {
        let flair = extract_effective_attribute_value(
            player,
            attribute_keys,
            AttributeKey::Flair,
            physical_state,
        );
        let bravery = extract_effective_attribute_value(
            player,
            attribute_keys,
            AttributeKey::Bravery,
            physical_state,
        );
        let vision = extract_effective_attribute_value(
            player,
            attribute_keys,
            AttributeKey::Vision,
            physical_state,
        );
        let decisions = extract_effective_attribute_value(
            player,
            attribute_keys,
            AttributeKey::Decisions,
            physical_state,
        );

        let norm_flair = (flair.clamp(0.0, 20.0)) / 10.0;
        let norm_bravery = (bravery.clamp(0.0, 20.0)) / 10.0;
        let norm_vision = (vision.clamp(0.0, 20.0)) / 10.0;
        let norm_decisions = (decisions.clamp(0.0, 20.0)) / 10.0;

        let physical_exhaustion = calculate_physical_exhaustion(physical_state);

        let base_tolerance = 0.50
            + 0.35 * norm_flair
            + 0.30 * norm_bravery
            + 0.20 * norm_vision
            + 0.15 * norm_decisions;

        let tolerance_index = (base_tolerance * (1.0 - 0.40 * physical_exhaustion)).clamp(0.40, 2.50);

        let loss_aversion_lambda = (2.25
            - 0.45 * (norm_bravery - 1.0)
            - 0.35 * (norm_flair - 1.0)
            + 0.20 * (1.0 - norm_decisions)
            + 0.60 * physical_exhaustion)
            .clamp(1.10, 4.50);

        let gain_diminishing_alpha = (0.88 + 0.06 * (norm_vision - 1.0)).clamp(0.70, 1.00);
        let loss_diminishing_beta = (0.88 + 0.06 * (norm_decisions - 1.0)).clamp(0.70, 1.00);

        let probability_distortion_gamma = (0.65
            - 0.12 * (norm_flair - 1.0)
            + 0.15 * (norm_decisions - 1.0)
            - 0.10 * physical_exhaustion)
            .clamp(0.40, 0.95);

        Self {
            tolerance_index,
            loss_aversion_lambda,
            gain_diminishing_alpha,
            loss_diminishing_beta,
            probability_distortion_gamma,
            physical_exhaustion,
        }
    }

    pub fn from_player_unfatigued(
        player: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Self {
        Self::from_player(player, attribute_keys, &PhysicalState::initial())
    }

    pub fn tolerance_index(&self) -> f64 {
        self.tolerance_index
    }

    pub fn loss_aversion_lambda(&self) -> f64 {
        self.loss_aversion_lambda
    }

    pub fn gain_diminishing_alpha(&self) -> f64 {
        self.gain_diminishing_alpha
    }

    pub fn loss_diminishing_beta(&self) -> f64 {
        self.loss_diminishing_beta
    }

    pub fn probability_distortion_gamma(&self) -> f64 {
        self.probability_distortion_gamma
    }

    pub fn physical_exhaustion(&self) -> f64 {
        self.physical_exhaustion
    }

    pub fn transform_value(&self, delta_epv: f64) -> f64 {
        if delta_epv >= 0.0 {
            delta_epv.powf(self.gain_diminishing_alpha)
        } else {
            -self.loss_aversion_lambda * (-delta_epv).powf(self.loss_diminishing_beta)
        }
    }

    pub fn weight_probability(&self, p: f64) -> f64 {
        let p_clamped = p.clamp(0.0001, 0.9999);
        let g = self.probability_distortion_gamma;
        let num = p_clamped.powf(g);
        let den = (p_clamped.powf(g) + (1.0 - p_clamped).powf(g)).powf(1.0 / g);
        if den > 1e-9 {
            (num / den).clamp(0.0, 1.0)
        } else {
            p_clamped
        }
    }

    pub fn risk_multiplier_for_action(&self, kind: ArtrineDecisionKind) -> f64 {
        let delta = self.tolerance_index - 1.0;
        let ex = self.physical_exhaustion;
        let raw = match kind {
            ArtrineDecisionKind::SelfCarry => (1.0 + delta * 0.15) * (1.0 - 0.65 * ex),
            ArtrineDecisionKind::ShortPass => (1.0 - delta * 0.20) * (1.0 + 0.45 * ex),
            ArtrineDecisionKind::LongLaunch => (1.0 + delta * 0.35) * (1.0 + 0.35 * ex),
            ArtrineDecisionKind::Cross => (1.0 + delta * 0.30) * (1.0 + 0.10 * ex),
            ArtrineDecisionKind::SelfFinish => (1.0 + delta * 0.25) * (1.0 - 0.50 * ex),
        };
        raw.clamp(0.20, 2.50)
    }
}

impl Default for RiskProfile {
    fn default() -> Self {
        Self {
            tolerance_index: 1.0,
            loss_aversion_lambda: 2.25,
            gain_diminishing_alpha: 0.88,
            loss_diminishing_beta: 0.88,
            probability_distortion_gamma: 0.65,
            physical_exhaustion: 0.0,
        }
    }
}