use arlo_domain::sport_constants::ARTRINE_DECISION_LOGIT_STEEPNESS;
use arlo_domain::ArtrineDecisionKind;
use arlo_math::stats::contrast::logistic_scaled;
use arlo_tactics::TeamInstructions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamIdentityBias {
    self_carry: f64,
    short_pass: f64,
    long_launch: f64,
    cross: f64,
    self_finish: f64,
}

impl TeamIdentityBias {
    pub fn new(
        self_carry: f64,
        short_pass: f64,
        long_launch: f64,
        cross: f64,
        self_finish: f64,
    ) -> Self {
        Self {
            self_carry,
            short_pass,
            long_launch,
            cross,
            self_finish,
        }
    }

    pub fn bias_for(&self, kind: ArtrineDecisionKind) -> f64 {
        match kind {
            ArtrineDecisionKind::SelfCarry => self.self_carry,
            ArtrineDecisionKind::ShortPass => self.short_pass,
            ArtrineDecisionKind::LongLaunch => self.long_launch,
            ArtrineDecisionKind::Cross => self.cross,
            ArtrineDecisionKind::SelfFinish => self.self_finish,
        }
    }

    pub fn self_carry(&self) -> f64 {
        self.self_carry
    }

    pub fn short_pass(&self) -> f64 {
        self.short_pass
    }

    pub fn long_launch(&self) -> f64 {
        self.long_launch
    }

    pub fn cross(&self) -> f64 {
        self.cross
    }

    pub fn self_finish(&self) -> f64 {
        self.self_finish
    }
}

impl Default for TeamIdentityBias {
    fn default() -> Self {
        Self {
            self_carry: 1.0,
            short_pass: 1.0,
            long_launch: 1.0,
            cross: 1.0,
            self_finish: 1.0,
        }
    }
}

pub fn decision_signature(kind: ArtrineDecisionKind) -> (f64, f64) {
    match kind {
        ArtrineDecisionKind::ShortPass => (-1.0, -1.0),
        ArtrineDecisionKind::SelfCarry => (0.0, 0.0),
        ArtrineDecisionKind::LongLaunch => (0.5, 1.0),
        ArtrineDecisionKind::Cross => (0.5, 1.0),
        ArtrineDecisionKind::SelfFinish => (1.0, 0.0),
    }
}

pub fn derive(instructions: &TeamInstructions) -> TeamIdentityBias {
    let mentality = instructions.in_possession().mentality().value();
    let directness = instructions.in_possession().directness().value();

    let bias_for_kind = |kind: ArtrineDecisionKind| -> f64 {
        let (risk, directivity) = decision_signature(kind);
        let dot_product = mentality * risk + directness * directivity;
        2.0 * logistic_scaled(dot_product, ARTRINE_DECISION_LOGIT_STEEPNESS)
    };

    TeamIdentityBias::new(
        bias_for_kind(ArtrineDecisionKind::SelfCarry),
        bias_for_kind(ArtrineDecisionKind::ShortPass),
        bias_for_kind(ArtrineDecisionKind::LongLaunch),
        bias_for_kind(ArtrineDecisionKind::Cross),
        bias_for_kind(ArtrineDecisionKind::SelfFinish),
    )
}