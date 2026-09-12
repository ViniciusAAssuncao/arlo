use arlo_domain::{KickFoulDecisionKind, KickFoulScoringTier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulAwarded {
    awarded_team_id: Uuid,
    offending_team_id: Uuid,
    scoring_tier: KickFoulScoringTier,
    spot_x_mirim: f64,
    spot_y_mirim: f64,
}

impl KickFoulAwarded {
    pub fn new(
        awarded_team_id: Uuid,
        offending_team_id: Uuid,
        scoring_tier: KickFoulScoringTier,
        spot_x_mirim: f64,
        spot_y_mirim: f64,
    ) -> Self {
        Self {
            awarded_team_id,
            offending_team_id,
            scoring_tier,
            spot_x_mirim,
            spot_y_mirim,
        }
    }

    pub fn awarded_team_id(&self) -> Uuid {
        self.awarded_team_id
    }

    pub fn offending_team_id(&self) -> Uuid {
        self.offending_team_id
    }

    pub fn scoring_tier(&self) -> KickFoulScoringTier {
        self.scoring_tier
    }

    pub fn spot_x_mirim(&self) -> f64 {
        self.spot_x_mirim
    }

    pub fn spot_y_mirim(&self) -> f64 {
        self.spot_y_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KickFoulDecisionMade {
    taker_id: Uuid,
    decision: KickFoulDecisionKind,
}

impl KickFoulDecisionMade {
    pub fn new(taker_id: Uuid, decision: KickFoulDecisionKind) -> Self {
        Self { taker_id, decision }
    }

    pub fn taker_id(&self) -> Uuid {
        self.taker_id
    }

    pub fn decision(&self) -> KickFoulDecisionKind {
        self.decision
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KickFoulEvent {
    Awarded(KickFoulAwarded),
    DecisionMade(KickFoulDecisionMade),
}

impl From<KickFoulAwarded> for KickFoulEvent {
    fn from(ev: KickFoulAwarded) -> Self {
        Self::Awarded(ev)
    }
}

impl From<KickFoulDecisionMade> for KickFoulEvent {
    fn from(ev: KickFoulDecisionMade) -> Self {
        Self::DecisionMade(ev)
    }
}
