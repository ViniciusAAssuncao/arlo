use arlo_domain::KickFoulScoringTier;
use arlo_math::units::Position as VectorPosition;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KickFoulPending {
    pub awarded_team_id: Uuid,
    pub spot: VectorPosition,
    pub scoring_tier: KickFoulScoringTier,
}

impl KickFoulPending {
    pub fn new(
        awarded_team_id: Uuid,
        spot: VectorPosition,
        scoring_tier: KickFoulScoringTier,
    ) -> Self {
        Self {
            awarded_team_id,
            spot,
            scoring_tier,
        }
    }

    pub fn awarded_team_id(&self) -> Uuid {
        self.awarded_team_id
    }

    pub fn spot(&self) -> VectorPosition {
        self.spot
    }

    pub fn scoring_tier(&self) -> KickFoulScoringTier {
        self.scoring_tier
    }
}
