use arlo_domain::{KickFoulDecisionKind, KickFoulScoringTier};
use arlo_math::units::Position as VectorPosition;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulAwarded {
    awarded_team_id: Uuid,
    offending_team_id: Uuid,
    spot: VectorPosition,
    scoring_tier: KickFoulScoringTier,
}

impl KickFoulAwarded {
    pub fn new(
        awarded_team_id: Uuid,
        offending_team_id: Uuid,
        spot: VectorPosition,
        scoring_tier: KickFoulScoringTier,
    ) -> Self {
        Self {
            awarded_team_id,
            offending_team_id,
            spot,
            scoring_tier,
        }
    }

    pub fn awarded_team_id(&self) -> Uuid {
        self.awarded_team_id
    }

    pub fn offending_team_id(&self) -> Uuid {
        self.offending_team_id
    }

    pub fn spot(&self) -> VectorPosition {
        self.spot
    }

    pub fn scoring_tier(&self) -> KickFoulScoringTier {
        self.scoring_tier
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulDecisionMade {
    taker_id: Uuid,
    decision_kind: KickFoulDecisionKind,
    scoring_tier: KickFoulScoringTier,
}

impl KickFoulDecisionMade {
    pub fn new(
        taker_id: Uuid,
        decision_kind: KickFoulDecisionKind,
        scoring_tier: KickFoulScoringTier,
    ) -> Self {
        Self {
            taker_id,
            decision_kind,
            scoring_tier,
        }
    }

    pub fn taker_id(&self) -> Uuid {
        self.taker_id
    }

    pub fn decision_kind(&self) -> KickFoulDecisionKind {
        self.decision_kind
    }

    pub fn scoring_tier(&self) -> KickFoulScoringTier {
        self.scoring_tier
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