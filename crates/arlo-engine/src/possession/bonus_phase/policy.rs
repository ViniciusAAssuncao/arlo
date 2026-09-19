use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BonusPhasePolicy {
    pub max_plays: u8,
}

impl Default for BonusPhasePolicy {
    fn default() -> Self {
        Self {
            max_plays: arlo_domain::sport_constants::DEFAULT_BONUS_PHASE_MAX_PLAYS as u8,
        }
    }
}