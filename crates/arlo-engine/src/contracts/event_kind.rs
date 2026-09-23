use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchEventCategory {
    Action,
    Possession,
    Scoring,
    Physical,
    Psychological,
    Manager,
    Officiating,
    Availability,
    KickFoul,
    Injury,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchEventKind {
    CallToActionStarted,
    PassCompleted,
    DistributionCompleted,
    ReceptionResolved,
    ArtrineDecisionMade,
    DriveRecorded,
    DuelResolved,
    Turnover,
    OutOfBounds,
    CountdownToSizeStarted,
    DownAdvanced,
    GoalPoint,
    FieldPoint,
    FieldGoal,
    ScoringAttemptMissed,
    PhysicalStrainRecorded,
    RecoveryIntervalProcessed,
    ImpulseShiftRecorded,
    ImpulseCriticalReached,
    PossessionTimeRecorded,
    SubstitutionMade,
    TimeCallUsed,
    ChallengeResolved,
    TacticalProfileActivated,
    PlayCallSelected,
    FoulRaised,
    AddedTimeAwarded,
    PlayerAvailabilityChanged,
    KickFoulAwarded,
    KickFoulDecisionMade,
    InjuryIncidentRecorded,
}

impl MatchEventKind {
    pub const COUNT: usize = 31;

    pub fn category(&self) -> MatchEventCategory {
        match self {
            Self::CallToActionStarted
            | Self::PassCompleted
            | Self::DistributionCompleted
            | Self::ReceptionResolved
            | Self::ArtrineDecisionMade
            | Self::DriveRecorded
            | Self::DuelResolved => MatchEventCategory::Action,
            Self::Turnover
            | Self::OutOfBounds
            | Self::CountdownToSizeStarted
            | Self::DownAdvanced
            | Self::PossessionTimeRecorded => MatchEventCategory::Possession,
            Self::GoalPoint
            | Self::FieldPoint
            | Self::FieldGoal
            | Self::ScoringAttemptMissed => MatchEventCategory::Scoring,
            Self::PhysicalStrainRecorded | Self::RecoveryIntervalProcessed => {
                MatchEventCategory::Physical
            }
            Self::ImpulseShiftRecorded | Self::ImpulseCriticalReached => {
                MatchEventCategory::Psychological
            }
            Self::SubstitutionMade
            | Self::TimeCallUsed
            | Self::ChallengeResolved
            | Self::TacticalProfileActivated
            | Self::PlayCallSelected => MatchEventCategory::Manager,
            Self::FoulRaised | Self::AddedTimeAwarded => MatchEventCategory::Officiating,
            Self::PlayerAvailabilityChanged => MatchEventCategory::Availability,
            Self::KickFoulAwarded | Self::KickFoulDecisionMade => MatchEventCategory::KickFoul,
            Self::InjuryIncidentRecorded => MatchEventCategory::Injury,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CallToActionStarted => "CallToActionStarted",
            Self::PassCompleted => "PassCompleted",
            Self::DistributionCompleted => "DistributionCompleted",
            Self::ReceptionResolved => "ReceptionResolved",
            Self::ArtrineDecisionMade => "ArtrineDecisionMade",
            Self::DriveRecorded => "DriveRecorded",
            Self::DuelResolved => "DuelResolved",
            Self::Turnover => "Turnover",
            Self::OutOfBounds => "OutOfBounds",
            Self::CountdownToSizeStarted => "CountdownToSizeStarted",
            Self::DownAdvanced => "DownAdvanced",
            Self::GoalPoint => "GoalPoint",
            Self::FieldPoint => "FieldPoint",
            Self::FieldGoal => "FieldGoal",
            Self::ScoringAttemptMissed => "ScoringAttemptMissed",
            Self::PhysicalStrainRecorded => "PhysicalStrainRecorded",
            Self::RecoveryIntervalProcessed => "RecoveryIntervalProcessed",
            Self::ImpulseShiftRecorded => "ImpulseShiftRecorded",
            Self::ImpulseCriticalReached => "ImpulseCriticalReached",
            Self::PossessionTimeRecorded => "PossessionTimeRecorded",
            Self::SubstitutionMade => "SubstitutionMade",
            Self::TimeCallUsed => "TimeCallUsed",
            Self::ChallengeResolved => "ChallengeResolved",
            Self::TacticalProfileActivated => "TacticalProfileActivated",
            Self::PlayCallSelected => "PlayCallSelected",
            Self::FoulRaised => "FoulRaised",
            Self::AddedTimeAwarded => "AddedTimeAwarded",
            Self::PlayerAvailabilityChanged => "PlayerAvailabilityChanged",
            Self::KickFoulAwarded => "KickFoulAwarded",
            Self::KickFoulDecisionMade => "KickFoulDecisionMade",
            Self::InjuryIncidentRecorded => "InjuryIncidentRecorded",
        }
    }

    pub fn from_event(event: &MatchEvent) -> Self {
        match event {
            MatchEvent::CallToActionStarted(_) => Self::CallToActionStarted,
            MatchEvent::PassCompleted(_) => Self::PassCompleted,
            MatchEvent::DistributionCompleted(_) => Self::DistributionCompleted,
            MatchEvent::ReceptionResolved(_) => Self::ReceptionResolved,
            MatchEvent::ArtrineDecisionMade(_) => Self::ArtrineDecisionMade,
            MatchEvent::DriveRecorded(_) => Self::DriveRecorded,
            MatchEvent::DuelResolved(_) => Self::DuelResolved,
            MatchEvent::Turnover(_) => Self::Turnover,
            MatchEvent::OutOfBounds(_) => Self::OutOfBounds,
            MatchEvent::CountdownToSizeStarted(_) => Self::CountdownToSizeStarted,
            MatchEvent::DownAdvanced(_) => Self::DownAdvanced,
            MatchEvent::GoalPoint(_) => Self::GoalPoint,
            MatchEvent::FieldPoint(_) => Self::FieldPoint,
            MatchEvent::FieldGoal(_) => Self::FieldGoal,
            MatchEvent::ScoringAttemptMissed(_) => Self::ScoringAttemptMissed,
            MatchEvent::PhysicalStrainRecorded(_) => Self::PhysicalStrainRecorded,
            MatchEvent::RecoveryIntervalProcessed(_) => Self::RecoveryIntervalProcessed,
            MatchEvent::ImpulseShiftRecorded(_) => Self::ImpulseShiftRecorded,
            MatchEvent::ImpulseCriticalReached(_) => Self::ImpulseCriticalReached,
            MatchEvent::PossessionTimeRecorded(_) => Self::PossessionTimeRecorded,
            MatchEvent::SubstitutionMade(_) => Self::SubstitutionMade,
            MatchEvent::TimeCallUsed(_) => Self::TimeCallUsed,
            MatchEvent::ChallengeResolved(_) => Self::ChallengeResolved,
            MatchEvent::TacticalProfileActivated(_) => Self::TacticalProfileActivated,
            MatchEvent::PlayCallSelected(_) => Self::PlayCallSelected,
            MatchEvent::FoulRaised(_) => Self::FoulRaised,
            MatchEvent::AddedTimeAwarded(_) => Self::AddedTimeAwarded,
            MatchEvent::PlayerAvailabilityChanged(_) => Self::PlayerAvailabilityChanged,
            MatchEvent::KickFoulAwarded(_) => Self::KickFoulAwarded,
            MatchEvent::KickFoulDecisionMade(_) => Self::KickFoulDecisionMade,
            MatchEvent::InjuryIncidentRecorded(_) => Self::InjuryIncidentRecorded,
        }
    }
}