use super::model::sample_call;
use super::ratings::RatingIndex;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState, SeriesAdvance};
use crate::step::StepResult;
use arlo_events::{
    CallToActionStarted, DownAdvanced, MatchEvent, OutOfBounds, PossessionTimeRecorded,
};
use arlo_manager_control::RequiredManagerDecision;
use arlo_tactics::{validate_play_call, PlayCall, PlayCallCategory};

pub fn resolve_next_segment(
    input: &MatchInput,
    state: &mut MatchState,
    selected_play_call: Option<&PlayCall>,
) -> EngineResult<StepResult> {
    if input.match_id() != state.match_id()
        || input.home().team_id() != state.home().team_id()
        || input.away().team_id() != state.away().team_id()
    {
        return Err(EngineError::InvalidInput(
            "state and match input differ".into(),
        ));
    }
    if state.phase() == MatchPhase::Finished {
        return Ok(StepResult::finished(Vec::new()));
    }
    if state.phase() == MatchPhase::PeriodBreak {
        let mut next = state.clone();
        next.start_next_quarter()?;
        *state = next;
        return Ok(StepResult::resolved(Vec::new()));
    }
    if !matches!(state.phase(), MatchPhase::Ready | MatchPhase::Stopped) {
        return Err(EngineError::InvalidTransition(
            "statistical segment requires a ready Call-to-Action".into(),
        ));
    }
    if state.clock().seconds_in_period() >= state.clock().period_limit_seconds() {
        let mut next = state.clone();
        next.finish_quarter()?;
        let finished = next.phase() == MatchPhase::Finished;
        *state = next;
        return Ok(if finished {
            StepResult::finished(Vec::new())
        } else {
            StepResult::resolved(Vec::new())
        });
    }

    let offense_id = state.next_call_team_id();
    let is_home = offense_id == input.home().team_id();
    let (offense, defense) = if is_home {
        (input.home(), input.away())
    } else if offense_id == input.away().team_id() {
        (input.away(), input.home())
    } else {
        return Err(EngineError::InvalidTransition(
            "unknown offense team".into(),
        ));
    };
    if offense.manager().is_human_controlled() && selected_play_call.is_none() {
        return Ok(StepResult::awaiting_decision(vec![
            RequiredManagerDecision::PlayCall {
                team_id: offense_id,
            },
        ]));
    }
    if let Some(call) = selected_play_call {
        if call.team_id() != offense_id
            || call.tactical_lineup_id() != offense.lineup().id()
            || call.category() != PlayCallCategory::OpenPlay
        {
            return Err(EngineError::InvalidInput(
                "PlayCall does not match the offense lineup".into(),
            ));
        }
        validate_play_call(
            call.routes(),
            call.role_overrides(),
            call.misdirection(),
            call.category(),
            offense.lineup(),
        )
        .map_err(|error| EngineError::InvalidInput(error.to_string()))?;
    }

    let ratings = RatingIndex::new(input);
    let offense_rating = ratings.team_ratings(offense)?;
    let defense_rating = ratings.team_ratings(defense)?;
    let mut next = state.clone();
    let prior_down = next.series().down();
    let prior_advance = next.series().valid_advance_mirim();
    let start_mirim = next.possession().ball_position_mirim();
    let target_advance_mirim = next.series().distance_to_gain_mirim();
    let (passer_id, artrine_id) = if is_home {
        (next.home().passer_id(), next.home().artrine_id())
    } else {
        (next.away().passer_id(), next.away().artrine_id())
    };
    let remaining_time = next.clock().period_limit_seconds() - next.clock().seconds_in_period();
    let sample = sample_call(
        offense,
        offense_rating,
        defense_rating,
        is_home,
        selected_play_call,
        next.rng_mut(),
    );
    let duration = sample.duration_seconds.min(remaining_time);
    let direction = if is_home { 1.0 } else { -1.0 };
    let end_mirim =
        (start_mirim + direction * sample.gain_mirim).clamp(0.0, input.pitch().length_mirim());
    let gain_mirim = direction * (end_mirim - start_mirim);

    next.begin_call_to_action()?;
    let mut events = Vec::with_capacity(4);
    events.push(
        next.emit(MatchEvent::CallToActionStarted(CallToActionStarted::new(
            offense_id,
            defense.team_id(),
            passer_id,
            artrine_id,
            u32::from(prior_down),
            start_mirim,
            target_advance_mirim,
        )))?,
    );
    next.advance_playing_time(duration)?;
    let advance = next.record_valid_advance(gain_mirim, end_mirim)?;
    let first_down = matches!(advance, SeriesAdvance::FirstDown);
    next.resolve_out(offense_id, end_mirim)?;
    events.push(next.emit(MatchEvent::PossessionTimeRecorded(
        PossessionTimeRecorded::new(offense_id, duration),
    ))?);
    events.push(next.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
        offense_id, None, false,
    )))?);
    events.push(next.emit(MatchEvent::DownAdvanced(DownAdvanced::new(
        u32::from(prior_down),
        u32::from(next.series().down()),
        gain_mirim,
        prior_advance + gain_mirim,
        first_down,
        end_mirim,
    )))?);

    *state = next;
    Ok(StepResult::resolved(events))
}
