use crate::resolution::duel_kind::DuelKind;
use crate::world_state::context_analyzer::GameStatePressure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DuelContext {
    attacker_is_home: bool,
    defender_is_home: bool,
    aggression_logit_offset: f64,
    misdirection_logit_offset: f64,
    physicality_logit_offset: f64,
    pressure: Option<GameStatePressure>,
}

impl DuelContext {
    pub fn new(attacker_is_home: bool, defender_is_home: bool) -> Self {
        Self {
            attacker_is_home,
            defender_is_home,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
            pressure: None,
        }
    }

    pub fn with_aggression_offset(
        attacker_is_home: bool,
        defender_is_home: bool,
        aggression_logit_offset: f64,
    ) -> Self {
        Self {
            attacker_is_home,
            defender_is_home,
            aggression_logit_offset,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
            pressure: None,
        }
    }

    pub fn with_offsets(
        attacker_is_home: bool,
        defender_is_home: bool,
        aggression_logit_offset: f64,
        misdirection_logit_offset: f64,
        physicality_logit_offset: f64,
    ) -> Self {
        Self {
            attacker_is_home,
            defender_is_home,
            aggression_logit_offset,
            misdirection_logit_offset,
            physicality_logit_offset,
            pressure: None,
        }
    }

    pub fn with_pressure(mut self, pressure: GameStatePressure) -> Self {
        self.pressure = Some(pressure);
        self
    }

    pub fn neutral() -> Self {
        Self {
            attacker_is_home: false,
            defender_is_home: false,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
            pressure: None,
        }
    }

    pub fn attacker_home() -> Self {
        Self {
            attacker_is_home: true,
            defender_is_home: false,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
            pressure: None,
        }
    }

    pub fn defender_home() -> Self {
        Self {
            attacker_is_home: false,
            defender_is_home: true,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
            pressure: None,
        }
    }

    pub fn attacker_is_home(&self) -> bool {
        self.attacker_is_home
    }

    pub fn defender_is_home(&self) -> bool {
        self.defender_is_home
    }

    pub fn aggression_logit_offset(&self) -> f64 {
        self.aggression_logit_offset
    }

    pub fn misdirection_logit_offset(&self) -> f64 {
        self.misdirection_logit_offset
    }

    pub fn physicality_logit_offset(&self) -> f64 {
        self.physicality_logit_offset
    }

    pub fn pressure(&self) -> Option<&GameStatePressure> {
        self.pressure.as_ref()
    }

    pub fn for_duel_kind(&self, kind: DuelKind) -> Self {
        if kind.is_contact_duel() {
            *self
        } else {
            Self {
                attacker_is_home: self.attacker_is_home,
                defender_is_home: self.defender_is_home,
                aggression_logit_offset: 0.0,
                misdirection_logit_offset: self.misdirection_logit_offset,
                physicality_logit_offset: 0.0,
                pressure: self.pressure,
            }
        }
    }
}