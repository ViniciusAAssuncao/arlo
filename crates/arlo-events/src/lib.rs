pub mod action;
pub mod availability;
pub mod envelope;
pub mod events;
pub mod in_memory_sink;
pub mod kick_foul;
pub mod officiating;
pub mod physical;
pub mod possession;
pub mod psychology;
pub mod scoring;
pub mod sink;

pub use action::{
    ActionEvent, ArtrineDecisionMade, CallToActionStarted, DistributionCompleted, DriveRecorded,
    DriveRegistered, DuelKind, DuelResolved, EventArtroPlacement, PassCompleted, ReceptionResolved,
};
pub use arlo_domain::pitch::ArtroPlacement;
pub use arlo_domain::PitchZone;
pub use availability::{AvailabilityStatus, PlayerAvailabilityChanged};
pub use envelope::{MatchClockInstant, MatchEventEnvelope};
pub use events::manager::*;
pub use in_memory_sink::InMemorySink;
pub use kick_foul::{KickFoulAwarded, KickFoulDecisionMade, KickFoulEvent};
pub use officiating::{FoulOrigin, FoulRaised, OfficiatingEvent};
pub use physical::{PhysicalEvent, PhysicalStrainRecorded, RecoveryIntervalProcessed};
pub use possession::{
    CountdownReason, CountdownToSizeStarted, DownAdvanced, OutOfBounds, PossessionEvent,
    PossessionTimeRecorded, Turnover,
};
pub use psychology::{
    ImpulseCriticalReached, ImpulseEventKind, ImpulseShiftRecorded, PsychologyEvent,
};
pub use scoring::{
    FieldGoalScored, FieldPointScored, GoalPointScored, ScoringAttemptMissed, ScoringEvent,
    ScoringPost,
};
pub use sink::EventSink;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchEvent {
    CallToActionStarted(CallToActionStarted),
    PassCompleted(PassCompleted),
    DistributionCompleted(DistributionCompleted),
    ReceptionResolved(ReceptionResolved),
    ArtrineDecisionMade(ArtrineDecisionMade),
    DriveRecorded(DriveRecorded),
    DuelResolved(DuelResolved),
    Turnover(Turnover),
    OutOfBounds(OutOfBounds),
    CountdownToSizeStarted(CountdownToSizeStarted),
    DownAdvanced(DownAdvanced),
    GoalPoint(GoalPointScored),
    FieldPoint(FieldPointScored),
    FieldGoal(FieldGoalScored),
    ScoringAttemptMissed(ScoringAttemptMissed),
    PhysicalStrainRecorded(PhysicalStrainRecorded),
    RecoveryIntervalProcessed(RecoveryIntervalProcessed),
    ImpulseShiftRecorded(ImpulseShiftRecorded),
    ImpulseCriticalReached(ImpulseCriticalReached),
    PossessionTimeRecorded(PossessionTimeRecorded),
    SubstitutionMade(SubstitutionMade),
    TimeCallUsed(TimeCallUsed),
    ChallengeResolved(ChallengeResolved),
    TacticalProfileActivated(TacticalProfileActivated),
    PlayCallSelected(PlayCallSelected),
    FoulRaised(FoulRaised),
    PlayerAvailabilityChanged(PlayerAvailabilityChanged),
    KickFoulAwarded(KickFoulAwarded),
    KickFoulDecisionMade(KickFoulDecisionMade),
}

impl MatchEvent {
    pub fn is_action(&self) -> bool {
        matches!(
            self,
            Self::CallToActionStarted(_)
                | Self::PassCompleted(_)
                | Self::DistributionCompleted(_)
                | Self::ReceptionResolved(_)
                | Self::ArtrineDecisionMade(_)
                | Self::DriveRecorded(_)
                | Self::DuelResolved(_)
                | Self::KickFoulDecisionMade(_)
        )
    }

    pub fn is_possession(&self) -> bool {
        matches!(
            self,
            Self::Turnover(_)
                | Self::OutOfBounds(_)
                | Self::CountdownToSizeStarted(_)
                | Self::DownAdvanced(_)
                | Self::PossessionTimeRecorded(_)
        )
    }

    pub fn is_scoring(&self) -> bool {
        matches!(
            self,
            Self::GoalPoint(_) | Self::FieldPoint(_) | Self::FieldGoal(_)
        )
    }

    pub fn is_missed_attempt(&self) -> bool {
        matches!(self, Self::ScoringAttemptMissed(_))
    }

    pub fn is_physical(&self) -> bool {
        matches!(
            self,
            Self::PhysicalStrainRecorded(_) | Self::RecoveryIntervalProcessed(_)
        )
    }

    pub fn is_psychological(&self) -> bool {
        matches!(
            self,
            Self::ImpulseShiftRecorded(_) | Self::ImpulseCriticalReached(_)
        )
    }

    pub fn is_manager(&self) -> bool {
        matches!(
            self,
            Self::SubstitutionMade(_)
                | Self::TimeCallUsed(_)
                | Self::ChallengeResolved(_)
                | Self::TacticalProfileActivated(_)
                | Self::PlayCallSelected(_)
        )
    }

    pub fn is_officiating(&self) -> bool {
        matches!(self, Self::FoulRaised(_) | Self::KickFoulAwarded(_))
    }

    pub fn is_availability(&self) -> bool {
        matches!(self, Self::PlayerAvailabilityChanged(_))
    }

    pub fn event_type_name(&self) -> &'static str {
        match self {
            Self::CallToActionStarted(_) => "CallToActionStarted",
            Self::PassCompleted(_) => "PassCompleted",
            Self::DistributionCompleted(_) => "DistributionCompleted",
            Self::ReceptionResolved(_) => "ReceptionResolved",
            Self::ArtrineDecisionMade(_) => "ArtrineDecisionMade",
            Self::DriveRecorded(_) => "DriveRecorded",
            Self::DuelResolved(_) => "DuelResolved",
            Self::Turnover(_) => "Turnover",
            Self::OutOfBounds(_) => "OutOfBounds",
            Self::CountdownToSizeStarted(_) => "CountdownToSizeStarted",
            Self::DownAdvanced(_) => "DownAdvanced",
            Self::GoalPoint(_) => "GoalPoint",
            Self::FieldPoint(_) => "FieldPoint",
            Self::FieldGoal(_) => "FieldGoal",
            Self::ScoringAttemptMissed(_) => "ScoringAttemptMissed",
            Self::PhysicalStrainRecorded(_) => "PhysicalStrainRecorded",
            Self::RecoveryIntervalProcessed(_) => "RecoveryIntervalProcessed",
            Self::ImpulseShiftRecorded(_) => "ImpulseShiftRecorded",
            Self::ImpulseCriticalReached(_) => "ImpulseCriticalReached",
            Self::PossessionTimeRecorded(_) => "PossessionTimeRecorded",
            Self::SubstitutionMade(_) => "SubstitutionMade",
            Self::TimeCallUsed(_) => "TimeCallUsed",
            Self::ChallengeResolved(_) => "ChallengeResolved",
            Self::TacticalProfileActivated(_) => "TacticalProfileActivated",
            Self::PlayCallSelected(_) => "PlayCallSelected",
            Self::FoulRaised(_) => "FoulRaised",
            Self::PlayerAvailabilityChanged(_) => "PlayerAvailabilityChanged",
            Self::KickFoulAwarded(_) => "KickFoulAwarded",
            Self::KickFoulDecisionMade(_) => "KickFoulDecisionMade",
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

impl From<DistributionCompleted> for MatchEvent {
    fn from(ev: DistributionCompleted) -> Self {
        Self::DistributionCompleted(ev)
    }
}

impl From<ReceptionResolved> for MatchEvent {
    fn from(ev: ReceptionResolved) -> Self {
        Self::ReceptionResolved(ev)
    }
}

impl From<ArtrineDecisionMade> for MatchEvent {
    fn from(ev: ArtrineDecisionMade) -> Self {
        Self::ArtrineDecisionMade(ev)
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

impl From<ScoringAttemptMissed> for MatchEvent {
    fn from(ev: ScoringAttemptMissed) -> Self {
        Self::ScoringAttemptMissed(ev)
    }
}

impl From<PhysicalStrainRecorded> for MatchEvent {
    fn from(ev: PhysicalStrainRecorded) -> Self {
        Self::PhysicalStrainRecorded(ev)
    }
}

impl From<RecoveryIntervalProcessed> for MatchEvent {
    fn from(ev: RecoveryIntervalProcessed) -> Self {
        Self::RecoveryIntervalProcessed(ev)
    }
}

impl From<ImpulseShiftRecorded> for MatchEvent {
    fn from(ev: ImpulseShiftRecorded) -> Self {
        Self::ImpulseShiftRecorded(ev)
    }
}

impl From<ImpulseCriticalReached> for MatchEvent {
    fn from(ev: ImpulseCriticalReached) -> Self {
        Self::ImpulseCriticalReached(ev)
    }
}

impl From<PossessionTimeRecorded> for MatchEvent {
    fn from(ev: PossessionTimeRecorded) -> Self {
        Self::PossessionTimeRecorded(ev)
    }
}

impl From<SubstitutionMade> for MatchEvent {
    fn from(ev: SubstitutionMade) -> Self {
        Self::SubstitutionMade(ev)
    }
}

impl From<TimeCallUsed> for MatchEvent {
    fn from(ev: TimeCallUsed) -> Self {
        Self::TimeCallUsed(ev)
    }
}

impl From<ChallengeResolved> for MatchEvent {
    fn from(ev: ChallengeResolved) -> Self {
        Self::ChallengeResolved(ev)
    }
}

impl From<TacticalProfileActivated> for MatchEvent {
    fn from(ev: TacticalProfileActivated) -> Self {
        Self::TacticalProfileActivated(ev)
    }
}

impl From<PlayCallSelected> for MatchEvent {
    fn from(ev: PlayCallSelected) -> Self {
        Self::PlayCallSelected(ev)
    }
}

impl From<FoulRaised> for MatchEvent {
    fn from(ev: FoulRaised) -> Self {
        Self::FoulRaised(ev)
    }
}

impl From<PlayerAvailabilityChanged> for MatchEvent {
    fn from(ev: PlayerAvailabilityChanged) -> Self {
        Self::PlayerAvailabilityChanged(ev)
    }
}

impl From<KickFoulAwarded> for MatchEvent {
    fn from(ev: KickFoulAwarded) -> Self {
        Self::KickFoulAwarded(ev)
    }
}

impl From<KickFoulDecisionMade> for MatchEvent {
    fn from(ev: KickFoulDecisionMade) -> Self {
        Self::KickFoulDecisionMade(ev)
    }
}

impl From<PhysicalEvent> for MatchEvent {
    fn from(ev: PhysicalEvent) -> Self {
        match ev {
            PhysicalEvent::PhysicalStrainRecorded(e) => Self::PhysicalStrainRecorded(e),
            PhysicalEvent::RecoveryIntervalProcessed(e) => Self::RecoveryIntervalProcessed(e),
        }
    }
}

impl From<PsychologyEvent> for MatchEvent {
    fn from(ev: PsychologyEvent) -> Self {
        match ev {
            PsychologyEvent::ImpulseShiftRecorded(e) => Self::ImpulseShiftRecorded(e),
            PsychologyEvent::ImpulseCriticalReached(e) => Self::ImpulseCriticalReached(e),
        }
    }
}

impl From<ActionEvent> for MatchEvent {
    fn from(ev: ActionEvent) -> Self {
        match ev {
            ActionEvent::CallToActionStarted(e) => Self::CallToActionStarted(e),
            ActionEvent::PassCompleted(e) => Self::PassCompleted(e),
            ActionEvent::DistributionCompleted(e) => Self::DistributionCompleted(e),
            ActionEvent::ReceptionResolved(e) => Self::ReceptionResolved(e),
            ActionEvent::ArtrineDecisionMade(e) => Self::ArtrineDecisionMade(e),
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
            PossessionEvent::PossessionTimeRecorded(e) => Self::PossessionTimeRecorded(e),
        }
    }
}

impl From<ScoringEvent> for MatchEvent {
    fn from(ev: ScoringEvent) -> Self {
        match ev {
            ScoringEvent::GoalPoint(e) => Self::GoalPoint(e),
            ScoringEvent::FieldPoint(e) => Self::FieldPoint(e),
            ScoringEvent::FieldGoal(e) => Self::FieldGoal(e),
            ScoringEvent::AttemptMissed(e) => Self::ScoringAttemptMissed(e),
        }
    }
}

impl From<ManagerEvent> for MatchEvent {
    fn from(ev: ManagerEvent) -> Self {
        match ev {
            ManagerEvent::SubstitutionMade(e) => Self::SubstitutionMade(e),
            ManagerEvent::TimeCallUsed(e) => Self::TimeCallUsed(e),
            ManagerEvent::ChallengeResolved(e) => Self::ChallengeResolved(e),
            ManagerEvent::TacticalProfileActivated(e) => Self::TacticalProfileActivated(e),
            ManagerEvent::PlayCallSelected(e) => Self::PlayCallSelected(e),
        }
    }
}

impl From<OfficiatingEvent> for MatchEvent {
    fn from(ev: OfficiatingEvent) -> Self {
        match ev {
            OfficiatingEvent::FoulRaised(e) => Self::FoulRaised(e),
        }
    }
}

impl From<KickFoulEvent> for MatchEvent {
    fn from(ev: KickFoulEvent) -> Self {
        match ev {
            KickFoulEvent::Awarded(e) => Self::KickFoulAwarded(e),
            KickFoulEvent::DecisionMade(e) => Self::KickFoulDecisionMade(e),
        }
    }
}