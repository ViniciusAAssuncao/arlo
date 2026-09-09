use crate::officiating::ReviewableCallKind as EngineReviewableCallKind;
use arlo_events::{
    ChallengeResolved, MatchClockInstant, PlayCallCategory as EventPlayCallCategory,
    PlayCallSelected, ReviewableCallKind as EventReviewableCallKind, SubstitutionMade,
    SubstitutionReason, TacticalProfileActivated, TimeCallUsed,
};
use arlo_tactics::PlayCallCategory as TacticalPlayCallCategory;
use uuid::Uuid;

pub fn translate_reviewable_call_kind(kind: EngineReviewableCallKind) -> EventReviewableCallKind {
    match kind {
        EngineReviewableCallKind::TurnoverClassification => {
            EventReviewableCallKind::TurnoverClassification
        }
        EngineReviewableCallKind::OutOfBoundsClassification => {
            EventReviewableCallKind::OutOfBoundsClassification
        }
        EngineReviewableCallKind::DriveValidity => EventReviewableCallKind::DriveValidity,
    }
}

pub fn translate_play_call_category(category: TacticalPlayCallCategory) -> EventPlayCallCategory {
    match category {
        TacticalPlayCallCategory::OpenPlay => EventPlayCallCategory::OpenPlay,
        TacticalPlayCallCategory::BonusPhaseConversion => {
            EventPlayCallCategory::BonusPhaseConversion
        }
    }
}

pub fn translate_substitution_made(
    team_id: Uuid,
    player_out: Uuid,
    player_in: Uuid,
    match_clock: MatchClockInstant,
    reason: SubstitutionReason,
) -> SubstitutionMade {
    SubstitutionMade::new(team_id, player_out, player_in, match_clock, reason)
}

pub fn translate_time_call_used(team_id: Uuid, remaining_time_calls_after: u32) -> TimeCallUsed {
    TimeCallUsed::new(team_id, remaining_time_calls_after)
}

pub fn translate_challenge_resolved(
    team_id: Uuid,
    call_kind: EngineReviewableCallKind,
    success: bool,
    remaining_challenges_after: u32,
) -> ChallengeResolved {
    ChallengeResolved::new(
        team_id,
        translate_reviewable_call_kind(call_kind),
        success,
        remaining_challenges_after,
    )
}

pub fn translate_tactical_profile_activated(
    team_id: Uuid,
    profile_id: Uuid,
    profile_name: impl Into<String>,
) -> TacticalProfileActivated {
    TacticalProfileActivated::new(team_id, profile_id, profile_name)
}

pub fn translate_play_call_selected(
    team_id: Uuid,
    play_call_id: Uuid,
    play_call_name: impl Into<String>,
    category: TacticalPlayCallCategory,
) -> PlayCallSelected {
    PlayCallSelected::new(
        team_id,
        play_call_id,
        play_call_name,
        translate_play_call_category(category),
    )
}
