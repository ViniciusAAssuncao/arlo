use super::artro::sample_artros;
use super::down::emit_down_advanced;
use super::model::sample_call;
use super::ratings::RatingIndex;
use super::shooting::resolve_regular_attempt;
use super::tuning::{OPEN_PLAY_OUT_PROBABILITY, OPEN_PLAY_TURNOVER_PROBABILITY};
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_domain::sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS;
use arlo_events::{DriveRecorded, MatchEvent, OutOfBounds, PossessionTimeRecorded, Turnover};
use rand::Rng;

pub(super) fn resolve_open_play_segment(
    input: &MatchInput,
    state: &mut MatchState,
) -> EngineResult<StepResult> {
    if state.phase() != MatchPhase::Live {
        return Err(EngineError::InvalidTransition(
            "open play requires live play".into(),
        ));
    }
    let possessor_id = state.possessor_team_id();
    let is_home = possessor_id == input.home().team_id();
    let (offense, defense) = if is_home {
        (input.home(), input.away())
    } else {
        (input.away(), input.home())
    };
    let ratings = RatingIndex::new(input);
    let offense_rating = ratings.team_ratings(offense)?;
    let defense_rating = ratings.team_ratings(defense)?;
    let mut next = state.clone();
    let pending = next.pending_call_outcome();
    let start_mirim = next.possession().ball_position_mirim();
    let remaining_time = next.clock().period_limit_seconds() - next.clock().seconds_in_period();
    let sample = sample_call(
        offense,
        offense_rating,
        defense_rating,
        is_home,
        None,
        next.rng_mut(),
    );
    let duration = sample.duration_seconds.min(remaining_time);
    let artros = if duration >= IMMEDIATE_POSSESSION_CONTROL_SECONDS {
        sample_artros(&ratings, offense, defense, None, duration, next.rng_mut())?
    } else {
        Vec::new()
    };
    let direction = if is_home { 1.0 } else { -1.0 };
    let end_mirim =
        (start_mirim + direction * sample.gain_mirim).clamp(0.0, input.pitch().length_mirim());
    let turnover = duration < remaining_time
        && next.rng_mut().gen_range(0.0..1.0) < OPEN_PLAY_TURNOVER_PROBABILITY;
    let ends_out = !turnover
        && (duration >= remaining_time
            || next.rng_mut().gen_range(0.0..1.0) < OPEN_PLAY_OUT_PROBABILITY);
    next.advance_playing_time(duration)?;
    next.move_live_ball(end_mirim)?;
    let mut events = vec![next.emit(MatchEvent::PossessionTimeRecorded(
        PossessionTimeRecorded::new(possessor_id, duration),
    ))?];
    let artrine_id = if is_home {
        next.home().artrine_id()
    } else {
        next.away().artrine_id()
    };
    for placement in artros {
        if let Some(drives_in_series) = next.record_artro(possessor_id, artrine_id, duration)? {
            events.push(next.emit(MatchEvent::DriveRecorded(DriveRecorded::new(
                artrine_id,
                drives_in_series,
                placement,
            )))?);
        }
    }
    if resolve_regular_attempt(
        input,
        &ratings,
        offense,
        defense,
        is_home,
        None,
        &mut next,
        &mut events,
    )? {
        *state = next;
        return Ok(StepResult::resolved(events));
    }
    if turnover {
        next.turnover(defense.team_id())?;
        events.push(next.emit(MatchEvent::Turnover(Turnover::new(
            possessor_id,
            defense.team_id(),
            None,
            None,
            true,
        )))?);
    }
    if ends_out {
        let next_team = next.possessor_team_id();
        next.resolve_out(next_team, end_mirim)?;
        events.push(next.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
            next_team, None, false,
        )))?);
        if let Some(outcome) = pending {
            emit_down_advanced(&mut next, &mut events, outcome, end_mirim)?;
        }
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
