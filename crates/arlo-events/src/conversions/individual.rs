use crate::*;

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

impl From<CarryResolved> for MatchEvent {
    fn from(ev: CarryResolved) -> Self {
        Self::CarryResolved(ev)
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

impl From<PasserContactResolved> for MatchEvent {
    fn from(ev: PasserContactResolved) -> Self {
        Self::PasserContactResolved(ev)
    }
}

impl From<GoalguardRecoveryResolved> for MatchEvent {
    fn from(ev: GoalguardRecoveryResolved) -> Self {
        Self::GoalguardRecoveryResolved(ev)
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

impl From<TacticalRealignmentMade> for MatchEvent {
    fn from(ev: TacticalRealignmentMade) -> Self {
        Self::TacticalRealignmentMade(ev)
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

impl From<AddedTimeAwarded> for MatchEvent {
    fn from(ev: AddedTimeAwarded) -> Self {
        Self::AddedTimeAwarded(ev)
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

impl From<InjuryIncidentRecorded> for MatchEvent {
    fn from(ev: InjuryIncidentRecorded) -> Self {
        Self::InjuryIncidentRecorded(ev)
    }
}

impl From<TacticalPlanActivated> for MatchEvent {
    fn from(event: TacticalPlanActivated) -> Self {
        Self::TacticalPlanActivated(event)
    }
}
