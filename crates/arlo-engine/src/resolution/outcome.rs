use crate::resolution::duel_kind::DuelKind;
use arlo_math::Probability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DuelOutcome {
    kind: DuelKind,
    attacker_won: bool,
    attacker_rating: f64,
    defender_rating: f64,
    win_probability: Probability,
    net_advantage: f64,
    velocity_mitigation_factor: f64,
}

impl DuelOutcome {
    pub fn new(
        kind: DuelKind,
        attacker_won: bool,
        attacker_rating: f64,
        defender_rating: f64,
        win_probability: Probability,
        net_advantage: f64,
    ) -> Self {
        let velocity_mitigation_factor = if attacker_won {
            (0.75 + (net_advantage * 0.03)).clamp(0.40, 1.00)
        } else {
            (0.30 + (net_advantage * 0.03)).clamp(0.00, 0.40)
        };
        Self {
            kind,
            attacker_won,
            attacker_rating,
            defender_rating,
            win_probability,
            net_advantage,
            velocity_mitigation_factor,
        }
    }

    pub fn with_mitigation(
        kind: DuelKind,
        attacker_won: bool,
        attacker_rating: f64,
        defender_rating: f64,
        win_probability: Probability,
        net_advantage: f64,
        velocity_mitigation_factor: f64,
    ) -> Self {
        Self {
            kind,
            attacker_won,
            attacker_rating,
            defender_rating,
            win_probability,
            net_advantage,
            velocity_mitigation_factor,
        }
    }

    pub fn kind(&self) -> DuelKind {
        self.kind
    }

    pub fn attacker_won(&self) -> bool {
        self.attacker_won
    }

    pub fn defender_won(&self) -> bool {
        !self.attacker_won
    }

    pub fn attacker_rating(&self) -> f64 {
        self.attacker_rating
    }

    pub fn defender_rating(&self) -> f64 {
        self.defender_rating
    }

    pub fn win_probability(&self) -> Probability {
        self.win_probability
    }

    pub fn net_advantage(&self) -> f64 {
        self.net_advantage
    }

    pub fn velocity_mitigation_factor(&self) -> f64 {
        self.velocity_mitigation_factor
    }
}
