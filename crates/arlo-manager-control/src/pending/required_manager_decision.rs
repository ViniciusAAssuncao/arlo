use crate::decision_category::ManagerDecisionCategory;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredManagerDecision {
    PlayCall {
        team_id: Uuid,
    },
    KickFoulDecision {
        team_id: Uuid,
    },
    ForcedSubstitution {
        team_id: Uuid,
        outgoing_player_ids: Vec<Uuid>,
    },
}

impl RequiredManagerDecision {
    pub fn team_id(&self) -> Uuid {
        match self {
            Self::PlayCall { team_id } => *team_id,
            Self::KickFoulDecision { team_id } => *team_id,
            Self::ForcedSubstitution { team_id, .. } => *team_id,
        }
    }

    pub fn category(&self) -> ManagerDecisionCategory {
        match self {
            Self::PlayCall { .. } => ManagerDecisionCategory::PlayCall,
            Self::KickFoulDecision { .. } => ManagerDecisionCategory::KickFoulDecision,
            Self::ForcedSubstitution { .. } => ManagerDecisionCategory::ForcedSubstitution,
        }
    }
}
