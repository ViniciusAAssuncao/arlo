use super::kicker::select_kicker;
use super::ratings::RatingIndex;
use super::shooting_model::sample_bonus_shot;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState, ScoreKind};
use crate::step::StepResult;
use arlo_events::{
    AddedTimeAwarded, FieldGoalScored, MatchEvent, PossessionTimeRecorded, ScoringAttemptMissed,
    ScoringPost,
};

pub(super) fn resolve_bonus_segment(
    input: &MatchInput,
    state: &mut MatchState,
) -> EngineResult<StepResult> {
    if state.phase() != MatchPhase::BonusPhase {
        return Err(EngineError::InvalidTransition(
            "no Bonus Phase is active".into(),
        ));
    }
    let scorer_team_id = state.possessor_team_id();
    let is_home = scorer_team_id == input.home().team_id();
    let (offense, defense) = if is_home {
        (input.home(), input.away())
    } else {
        (input.away(), input.home())
    };
    let ratings = RatingIndex::new(input, state);
    let scorer_id = select_kicker(&ratings, offense, None)?;
    let mut next = state.clone();
    let sample = sample_bonus_shot(
        &ratings,
        offense,
        defense,
        scorer_id,
        input.pitch().length_mirim(),
        next.rng_mut(),
    )?;
    let maximum_remaining =
        next.clock().maximum_period_seconds() - next.clock().seconds_in_period();
    let duration = sample.duration_seconds.min(maximum_remaining);
    let mut events = Vec::with_capacity(3);
    let extension =
        next.clock().seconds_in_period() + duration - next.clock().period_limit_seconds();
    if extension > 0.0 {
        let new_added = next.clock().added_seconds() + extension;
        next.grant_added_time(new_added)?;
        if next.clock().period() % 2 == 0 {
            events.push(
                next.emit(MatchEvent::AddedTimeAwarded(AddedTimeAwarded::new(
                    next.clock().period(),
                    extension,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0.0,
                )))?,
            );
        }
    }
    next.advance_bonus_playing_time(duration)?;
    events.push(next.emit(MatchEvent::PossessionTimeRecorded(
        PossessionTimeRecorded::new(scorer_team_id, duration),
    ))?);
    if sample.converted {
        let kind = match sample.post {
            ScoringPost::Goalpost => ScoreKind::BonusGoalpost,
            ScoringPost::Fieldpost => ScoreKind::BonusFieldpost,
        };
        next.apply_score(scorer_team_id, kind)?;
        events.push(next.emit(MatchEvent::FieldGoal(FieldGoalScored::new(
            scorer_team_id,
            scorer_id,
            sample.post,
        )))?);
    } else {
        next.finish_bonus_phase_without_score()?;
        events.push(
            next.emit(MatchEvent::ScoringAttemptMissed(ScoringAttemptMissed::new(
                scorer_team_id,
                scorer_id,
                sample.post,
            )))?,
        );
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
