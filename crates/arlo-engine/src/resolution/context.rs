use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DuelContext {
    attacker_is_home: bool,
    defender_is_home: bool,
}

impl DuelContext {
    pub fn new(attacker_is_home: bool, defender_is_home: bool) -> Self {
        Self {
            attacker_is_home,
            defender_is_home,
        }
    }

    pub fn neutral() -> Self {
        Self {
            attacker_is_home: false,
            defender_is_home: false,
        }
    }

    pub fn attacker_home() -> Self {
        Self {
            attacker_is_home: true,
            defender_is_home: false,
        }
    }

    pub fn defender_home() -> Self {
        Self {
            attacker_is_home: false,
            defender_is_home: true,
        }
    }

    pub fn attacker_is_home(&self) -> bool {
        self.attacker_is_home
    }

    pub fn defender_is_home(&self) -> bool {
        self.defender_is_home
    }
}