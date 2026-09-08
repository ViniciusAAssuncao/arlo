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
        Self {
            kind,
            attacker_won,
            attacker_rating,
            defender_rating,
            win_probability,
            net_advantage,
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
}
