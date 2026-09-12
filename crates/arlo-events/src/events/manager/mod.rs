pub mod challenge_resolved;
pub mod play_call_selected;
pub mod substitution_made;
pub mod tactical_profile_activated;
pub mod time_call_used;

pub use challenge_resolved::{ChallengeResolved, ReviewableCallKind};
pub use play_call_selected::{PlayCallCategory, PlayCallSelected};
pub use substitution_made::{SubstitutionMade, SubstitutionReason};
pub use tactical_profile_activated::TacticalProfileActivated;
pub use time_call_used::{TimeCallReason, TimeCallUsed};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ManagerEvent {
    SubstitutionMade(SubstitutionMade),
    TimeCallUsed(TimeCallUsed),
    ChallengeResolved(ChallengeResolved),
    TacticalProfileActivated(TacticalProfileActivated),
    PlayCallSelected(PlayCallSelected),
}

impl From<SubstitutionMade> for ManagerEvent {
    fn from(ev: SubstitutionMade) -> Self {
        Self::SubstitutionMade(ev)
    }
}

impl From<TimeCallUsed> for ManagerEvent {
    fn from(ev: TimeCallUsed) -> Self {
        Self::TimeCallUsed(ev)
    }
}

impl From<ChallengeResolved> for ManagerEvent {
    fn from(ev: ChallengeResolved) -> Self {
        Self::ChallengeResolved(ev)
    }
}

impl From<TacticalProfileActivated> for ManagerEvent {
    fn from(ev: TacticalProfileActivated) -> Self {
        Self::TacticalProfileActivated(ev)
    }
}

impl From<PlayCallSelected> for ManagerEvent {
    fn from(ev: PlayCallSelected) -> Self {
        Self::PlayCallSelected(ev)
    }
}