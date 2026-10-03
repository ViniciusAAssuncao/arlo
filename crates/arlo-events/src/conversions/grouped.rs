use crate::*;

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
            ActionEvent::CarryResolved(e) => Self::CarryResolved(e),
            ActionEvent::DistributionCompleted(e) => Self::DistributionCompleted(e),
            ActionEvent::ReceptionResolved(e) => Self::ReceptionResolved(e),
            ActionEvent::PasserContactResolved(e) => Self::PasserContactResolved(e),
            ActionEvent::GoalguardRecoveryResolved(e) => Self::GoalguardRecoveryResolved(e),
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
            ManagerEvent::TacticalRealignmentMade(e) => Self::TacticalRealignmentMade(e),
            ManagerEvent::TacticalPlanActivated(e) => Self::TacticalPlanActivated(e),
            ManagerEvent::PlayCallSelected(e) => Self::PlayCallSelected(e),
        }
    }
}

impl From<OfficiatingEvent> for MatchEvent {
    fn from(ev: OfficiatingEvent) -> Self {
        match ev {
            OfficiatingEvent::FoulRaised(e) => Self::FoulRaised(e),
            OfficiatingEvent::RefereeDecisionResolved(e) => Self::RefereeDecisionResolved(e),
            OfficiatingEvent::PunishmentApplied(e) => Self::PunishmentApplied(e),
            OfficiatingEvent::PlayInvalidated(e) => Self::PlayInvalidated(e),
            OfficiatingEvent::AddedTimeAwarded(e) => Self::AddedTimeAwarded(e),
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
