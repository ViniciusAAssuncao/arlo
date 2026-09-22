use crate::possession_flow::termination_reason::TerminationReason;
use crate::resolution::AttributedDuelOutcome;
use arlo_domain::ArtrineDecisionKind;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchOutcome {
    pub carrier_id: Uuid,
    pub decision_kind: ArtrineDecisionKind,
    pub duels: SmallVec<[AttributedDuelOutcome; 2]>,
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub is_scoring_attempt: bool,
    pub termination_reason: Option<TerminationReason>,
}

impl TouchOutcome {
    pub fn new(
        carrier_id: Uuid,
        decision_kind: ArtrineDecisionKind,
        duels: SmallVec<[AttributedDuelOutcome; 2]>,
        mirins_advanced: f64,
        drives_recorded: u32,
        is_scoring_attempt: bool,
        termination_reason: Option<TerminationReason>,
    ) -> Self {
        Self {
            carrier_id,
            decision_kind,
            duels,
            mirins_advanced,
            drives_recorded,
            is_scoring_attempt,
            termination_reason,
        }
    }

    pub fn carrier_id(&self) -> Uuid {
        self.carrier_id
    }

    pub fn decision_kind(&self) -> ArtrineDecisionKind {
        self.decision_kind
    }

    pub fn duels(&self) -> &[AttributedDuelOutcome] {
        &self.duels
    }

    pub fn mirins_advanced(&self) -> f64 {
        self.mirins_advanced
    }

    pub fn drives_recorded(&self) -> u32 {
        self.drives_recorded
    }

    pub fn drive_registered(&self) -> bool {
        self.drives_recorded > 0
    }

    pub fn is_scoring_attempt(&self) -> bool {
        self.is_scoring_attempt
    }

    pub fn termination_reason(&self) -> Option<TerminationReason> {
        self.termination_reason
    }

    pub fn is_terminal(&self) -> bool {
        self.termination_reason.is_some()
    }
}
