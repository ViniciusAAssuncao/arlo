pub mod added_time_awarded;
pub mod foul_origin;
pub mod foul_raised;
mod referee_decision_resolved;
mod punishment_applied;

pub use added_time_awarded::AddedTimeAwarded;
pub use foul_origin::FoulOrigin;
pub use foul_raised::FoulRaised;
pub use referee_decision_resolved::RefereeDecisionResolved;
pub use punishment_applied::PunishmentApplied;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfficiatingEvent {
    FoulRaised(FoulRaised),
    RefereeDecisionResolved(RefereeDecisionResolved),
    PunishmentApplied(PunishmentApplied),
    AddedTimeAwarded(AddedTimeAwarded),
}

impl From<FoulRaised> for OfficiatingEvent {
    fn from(ev: FoulRaised) -> Self {
        Self::FoulRaised(ev)
    }
}

impl From<RefereeDecisionResolved> for OfficiatingEvent {
    fn from(ev: RefereeDecisionResolved) -> Self {
        Self::RefereeDecisionResolved(ev)
    }
}

impl From<PunishmentApplied> for OfficiatingEvent {
    fn from(ev: PunishmentApplied) -> Self {
        Self::PunishmentApplied(ev)
    }
}

impl From<AddedTimeAwarded> for OfficiatingEvent {
    fn from(ev: AddedTimeAwarded) -> Self {
        Self::AddedTimeAwarded(ev)
    }
}
