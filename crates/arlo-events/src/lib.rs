mod conversions;

pub mod action;
pub mod availability;
pub mod envelope;
pub mod events;
pub mod in_memory_sink;
pub mod injury;
pub mod kick_foul;
pub mod officiating;
pub mod physical;
pub mod possession;
pub mod psychology;
pub mod scoring;
pub mod sink;

pub use action::{
    ActionEvent, ArtrineDecisionMade, CallToActionStarted, CarryResolved, DistributionCompleted,
    DriveRecorded, DriveRegistered, DuelKind, DuelResolved, EventArtroPlacement,
    GoalguardRecoveryResolved, PassCompleted, PasserContactResolved, ReceptionResolved,
};
pub use arlo_domain::pitch::ArtroPlacement;
pub use arlo_domain::PitchZone;
pub use availability::{AvailabilityStatus, PlayerAvailabilityChanged};
pub use envelope::{MatchClockInstant, MatchEventEnvelope};
pub use events::manager::*;
pub use in_memory_sink::InMemorySink;
pub use injury::InjuryIncidentRecorded;
pub use kick_foul::{KickFoulAwarded, KickFoulDecisionMade, KickFoulEvent};
pub use officiating::{
    AddedTimeAwarded, FoulOrigin, FoulRaised, OfficiatingEvent, PlayInvalidated, PunishmentApplied,
    RefereeDecisionResolved,
};
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
    CarryResolved(CarryResolved),
    DistributionCompleted(DistributionCompleted),
    ReceptionResolved(ReceptionResolved),
    PasserContactResolved(PasserContactResolved),
    GoalguardRecoveryResolved(GoalguardRecoveryResolved),
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
    TacticalRealignmentMade(TacticalRealignmentMade),
    TacticalPlanActivated(TacticalPlanActivated),
    PlayCallSelected(PlayCallSelected),
    FoulRaised(FoulRaised),
    RefereeDecisionResolved(RefereeDecisionResolved),
    PunishmentApplied(PunishmentApplied),
    PlayInvalidated(PlayInvalidated),
    AddedTimeAwarded(AddedTimeAwarded),
    PlayerAvailabilityChanged(PlayerAvailabilityChanged),
    KickFoulAwarded(KickFoulAwarded),
    KickFoulDecisionMade(KickFoulDecisionMade),
    InjuryIncidentRecorded(InjuryIncidentRecorded),
}

impl MatchEvent {
    pub fn is_action(&self) -> bool {
        matches!(
            self,
            Self::CallToActionStarted(_)
                | Self::PassCompleted(_)
                | Self::CarryResolved(_)
                | Self::DistributionCompleted(_)
                | Self::ReceptionResolved(_)
                | Self::PasserContactResolved(_)
                | Self::GoalguardRecoveryResolved(_)
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
                | Self::TacticalRealignmentMade(_)
                | Self::TacticalPlanActivated(_)
                | Self::PlayCallSelected(_)
        )
    }

    pub fn is_officiating(&self) -> bool {
        matches!(
            self,
            Self::FoulRaised(_)
                | Self::RefereeDecisionResolved(_)
                | Self::PunishmentApplied(_)
                | Self::PlayInvalidated(_)
                | Self::KickFoulAwarded(_)
                | Self::AddedTimeAwarded(_)
        )
    }

    pub fn is_availability(&self) -> bool {
        matches!(self, Self::PlayerAvailabilityChanged(_))
    }

    pub fn is_kick_foul(&self) -> bool {
        matches!(
            self,
            Self::KickFoulAwarded(_) | Self::KickFoulDecisionMade(_)
        )
    }

    pub fn is_injury(&self) -> bool {
        matches!(self, Self::InjuryIncidentRecorded(_))
    }

    pub fn event_type_name(&self) -> &'static str {
        match self {
            Self::CallToActionStarted(_) => "CallToActionStarted",
            Self::PassCompleted(_) => "PassCompleted",
            Self::CarryResolved(_) => "CarryResolved",
            Self::DistributionCompleted(_) => "DistributionCompleted",
            Self::ReceptionResolved(_) => "ReceptionResolved",
            Self::PasserContactResolved(_) => "PasserContactResolved",
            Self::GoalguardRecoveryResolved(_) => "GoalguardRecoveryResolved",
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
            Self::TacticalRealignmentMade(_) => "TacticalRealignmentMade",
            Self::TacticalPlanActivated(_) => "TacticalPlanActivated",
            Self::PlayCallSelected(_) => "PlayCallSelected",
            Self::FoulRaised(_) => "FoulRaised",
            Self::RefereeDecisionResolved(_) => "RefereeDecisionResolved",
            Self::PunishmentApplied(_) => "PunishmentApplied",
            Self::PlayInvalidated(_) => "PlayInvalidated",
            Self::AddedTimeAwarded(_) => "AddedTimeAwarded",
            Self::PlayerAvailabilityChanged(_) => "PlayerAvailabilityChanged",
            Self::KickFoulAwarded(_) => "KickFoulAwarded",
            Self::KickFoulDecisionMade(_) => "KickFoulDecisionMade",
            Self::InjuryIncidentRecorded(_) => "InjuryIncidentRecorded",
        }
    }
}
