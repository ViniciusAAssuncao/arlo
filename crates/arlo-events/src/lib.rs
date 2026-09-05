pub mod action;
pub mod envelope;
pub mod in_memory_sink;
pub mod possession;
pub mod scoring;
pub mod sink;

pub use action::{
    ActionEvent, CallToActionStarted, DriveRecorded, DriveRegistered, DuelKind, DuelResolved,
    EventArtroPlacement, PassCompleted,
};
pub use arlo_domain::pitch::ArtroPlacement;
pub use envelope::{MatchClockInstant, MatchEventEnvelope};
pub use in_memory_sink::InMemorySink;
pub use possession::{
    CountdownReason, CountdownToSizeStarted, DownAdvanced, OutOfBounds, PossessionEvent, Turnover,
};
pub use scoring::{FieldGoalScored, FieldPointScored, GoalPointScored, ScoringEvent, ScoringPost};
pub use sink::EventSink;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchEvent {
    CallToActionStarted(CallToActionStarted),
    PassCompleted(PassCompleted),
    DriveRecorded(DriveRecorded),
    DuelResolved(DuelResolved),
    Turnover(Turnover),
    OutOfBounds(OutOfBounds),
    CountdownToSizeStarted(CountdownToSizeStarted),
    DownAdvanced(DownAdvanced),
    GoalPoint(GoalPointScored),
    FieldPoint(FieldPointScored),
    FieldGoal(FieldGoalScored),
}

impl MatchEvent {
    pub fn is_action(&self) -> bool {
        matches!(
            self,
            Self::CallToActionStarted(_)
                | Self::PassCompleted(_)
                | Self::DriveRecorded(_)
                | Self::DuelResolved(_)
        )
    }

    pub fn is_possession(&self) -> bool {
        matches!(
            self,
            Self::Turnover(_)
                | Self::OutOfBounds(_)
                | Self::CountdownToSizeStarted(_)
                | Self::DownAdvanced(_)
        )
    }

    pub fn is_scoring(&self) -> bool {
        matches!(
            self,
            Self::GoalPoint(_) | Self::FieldPoint(_) | Self::FieldGoal(_)
        )
    }

    pub fn event_type_name(&self) -> &'static str {
        match self {
            Self::CallToActionStarted(_) => "CallToActionStarted",
            Self::PassCompleted(_) => "PassCompleted",
            Self::DriveRecorded(_) => "DriveRecorded",
            Self::DuelResolved(_) => "DuelResolved",
            Self::Turnover(_) => "Turnover",
            Self::OutOfBounds(_) => "OutOfBounds",
            Self::CountdownToSizeStarted(_) => "CountdownToSizeStarted",
            Self::DownAdvanced(_) => "DownAdvanced",
            Self::GoalPoint(_) => "GoalPoint",
            Self::FieldPoint(_) => "FieldPoint",
            Self::FieldGoal(_) => "FieldGoal",
        }
    }
}

impl From<CallToActionStarted> for MatchEvent {
    fn from(ev: CallToActionStarted) -> Self {
        Self::CallToActionStarted(ev)
    }
}

impl From<PassCompleted> for MatchEvent {
    fn from(ev: PassCompleted) -> Self {
        Self::PassCompleted(ev)
    }
}

impl From<DriveRecorded> for MatchEvent {
    fn from(ev: DriveRecorded) -> Self {
        Self::DriveRecorded(ev)
    }
}

impl From<DuelResolved> for MatchEvent {
    fn from(ev: DuelResolved) -> Self {
        Self::DuelResolved(ev)
    }
}

impl From<Turnover> for MatchEvent {
    fn from(ev: Turnover) -> Self {
        Self::Turnover(ev)
    }
}

impl From<OutOfBounds> for MatchEvent {
    fn from(ev: OutOfBounds) -> Self {
        Self::OutOfBounds(ev)
    }
}

impl From<CountdownToSizeStarted> for MatchEvent {
    fn from(ev: CountdownToSizeStarted) -> Self {
        Self::CountdownToSizeStarted(ev)
    }
}

impl From<DownAdvanced> for MatchEvent {
    fn from(ev: DownAdvanced) -> Self {
        Self::DownAdvanced(ev)
    }
}

impl From<GoalPointScored> for MatchEvent {
    fn from(ev: GoalPointScored) -> Self {
        Self::GoalPoint(ev)
    }
}

impl From<FieldPointScored> for MatchEvent {
    fn from(ev: FieldPointScored) -> Self {
        Self::FieldPoint(ev)
    }
}

impl From<FieldGoalScored> for MatchEvent {
    fn from(ev: FieldGoalScored) -> Self {
        Self::FieldGoal(ev)
    }
}

impl From<ActionEvent> for MatchEvent {
    fn from(ev: ActionEvent) -> Self {
        match ev {
            ActionEvent::CallToActionStarted(e) => Self::CallToActionStarted(e),
            ActionEvent::PassCompleted(e) => Self::PassCompleted(e),
            ActionEvent::DriveRecorded(e) => Self::DriveRecorded(e),
            ActionEvent::DuelResolved(e) => Self::DuelResolved(e),
        }
    }
}

impl From<PossessionEvent> for MatchEvent {
    fn from(ev: PossessionEvent) -> Self {
        match ev {
            PossessionEvent::Turnover(e) => Self::Turnover(e),
            PossessionEvent::OutOfBounds(e) => Self::OutOfBounds(e),
            PossessionEvent::CountdownToSizeStarted(e) => Self::CountdownToSizeStarted(e),
            PossessionEvent::DownAdvanced(e) => Self::DownAdvanced(e),
        }
    }
}

impl From<ScoringEvent> for MatchEvent {
    fn from(ev: ScoringEvent) -> Self {
        match ev {
            ScoringEvent::GoalPoint(e) => Self::GoalPoint(e),
            ScoringEvent::FieldPoint(e) => Self::FieldPoint(e),
            ScoringEvent::FieldGoal(e) => Self::FieldGoal(e),
        }
    }
}
