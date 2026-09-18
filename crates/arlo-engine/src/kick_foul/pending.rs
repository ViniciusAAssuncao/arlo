use arlo_domain::KickFoulScoringTier;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KickFoulPending {
    pub awarded_team_id: Uuid,
    pub spot_x_mirim: f64,
    pub spot_y_mirim: f64,
    pub scoring_tier: KickFoulScoringTier,
}

impl KickFoulPending {
    pub fn new(
        awarded_team_id: Uuid,
        spot_x_mirim: f64,
        spot_y_mirim: f64,
        scoring_tier: KickFoulScoringTier,
    ) -> Self {
        Self {
            awarded_team_id,
            spot_x_mirim,
            spot_y_mirim,
            scoring_tier,
        }
    }

    pub fn awarded_team_id(&self) -> Uuid {
        self.awarded_team_id
    }

    pub fn spot_x_mirim(&self) -> f64 {
        self.spot_x_mirim
    }

    pub fn spot_y_mirim(&self) -> f64 {
        self.spot_y_mirim
    }

    pub fn scoring_tier(&self) -> KickFoulScoringTier {
        self.scoring_tier
    }
}
