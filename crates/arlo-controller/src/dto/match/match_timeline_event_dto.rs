use super::timeline::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MatchTimelineEventDto {
    Foul(FoulTimelineEntryDto),
    Scoring(ScoringTimelineEntryDto),
    Substitution(SubstitutionTimelineEntryDto),
    Turnover(TurnoverTimelineEntryDto),
    Injury(InjuryTimelineEntryDto),
    KickFoul(KickFoulTimelineEntryDto),
    TimeCall(TimeCallTimelineEntryDto),
    Challenge(ChallengeTimelineEntryDto),
    TacticalProfile(TacticalProfileTimelineEntryDto),
    PlayCall(PlayCallTimelineEntryDto),
    AvailabilityChange(AvailabilityChangeTimelineEntryDto),
    ImpulseCritical(ImpulseCriticalTimelineEntryDto),
    AddedTime(AddedTimeTimelineEntryDto),
}
