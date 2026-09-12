use crate::kick_foul::resolution::restart_phase::KickFoulRestartResult;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::AttributedDuelOutcome;
use arlo_domain::KickFoulDecisionKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulResolutionOutcome {
    pub scoring_decision: ScoringDecision,
    pub restart: Option<KickFoulRestartResult>,
    pub duels: Vec<AttributedDuelOutcome>,
    pub blocked: bool,
    pub decision: KickFoulDecisionKind,
    pub taker_id: Uuid,
}

impl KickFoulResolutionOutcome {
    pub fn new(
        scoring_decision: ScoringDecision,
        restart: Option<KickFoulRestartResult>,
        duels: Vec<AttributedDuelOutcome>,
        blocked: bool,
        decision: KickFoulDecisionKind,
        taker_id: Uuid,
    ) -> Self {
        Self {
            scoring_decision,
            restart,
            duels,
            blocked,
            decision,
            taker_id,
        }
    }
}
