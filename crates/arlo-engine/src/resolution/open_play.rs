use super::actors::{select_actor, ActorRole};
use super::down::emit_down_advanced;
use super::model::sample_call;
use super::ratings::RatingIndex;
use super::sequence::{resolve_sequence, SequenceContext};
use super::shooting::resolve_regular_attempt;
use super::tuning::{OPEN_PLAY_OUT_PROBABILITY, OPEN_PLAY_TURNOVER_PROBABILITY};
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_events::{MatchEvent, OutOfBounds, Turnover};
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
    let ratings = RatingIndex::new(input, state);
    let offense_rating = ratings.team_ratings(offense)?;
    let defense_rating = ratings.team_ratings(defense)?;
    let mut next = state.clone();
    let carrier_id = match next.carrier_id() {
        Some(player_id) => player_id,
        None => select_actor(
            &ratings,
            offense,
            ActorRole::Carrier,
            None,
            next.rng_mut(),
        )?,
    };
    next.set_carrier(carrier_id)?;
    let carry_defender_id = select_actor(
        &ratings,
        defense,
        ActorRole::Defender,
        None,
        next.rng_mut(),
    )?;
    let pending = next.pending_call_outcome();
    let remaining_time = next.clock().period_limit_seconds() - next.clock().seconds_in_period();
    let sample = sample_call(
        &ratings,
        offense,
        defense,
        offense_rating,
        defense_rating,
        carrier_id,
        carry_defender_id,
        is_home,
        None,
        next.rng_mut(),
    )?;
    let duration = sample.duration_seconds.min(remaining_time);
    let artrine_id = if is_home {
        next.home().artrine_id()
    } else {
        next.away().artrine_id()
    };
    let turnover = duration < remaining_time
        && next.rng_mut().gen_range(0.0..1.0) < OPEN_PLAY_TURNOVER_PROBABILITY;
    let ends_out = !turnover
        && (duration >= remaining_time
            || next.rng_mut().gen_range(0.0..1.0) < OPEN_PLAY_OUT_PROBABILITY);
    let mut events = Vec::new();
    let outcome = resolve_sequence(
        SequenceContext {
            ratings: &ratings,
            offense,
            defense,
            offense_rating,
            defense_rating,
            is_home,
            selected_play_call: None,
            artrine_id,
            pitch_length_mirim: input.pitch().length_mirim(),
        },
        carrier_id,
        carry_defender_id,
        sample,
        duration,
        &mut next,
        &mut events,
    )?;
    let holder_id = outcome.holder_id;
    let end_mirim = outcome.end_mirim;
    if let Some(interceptor_id) = outcome.interceptor_id {
        next.turnover(defense.team_id())?;
        next.set_carrier(interceptor_id)?;
        events.push(next.emit(MatchEvent::Turnover(Turnover::new(
            possessor_id,
            defense.team_id(),
            Some(interceptor_id),
            Some(holder_id),
            true,
        )))?);
        *state = next;
        return Ok(StepResult::resolved(events));
    }
    if resolve_regular_attempt(
        input,
        &ratings,
        offense,
        defense,
        is_home,
        holder_id,
        None,
        &mut next,
        &mut events,
    )? {
        *state = next;
        return Ok(StepResult::resolved(events));
    }
    if turnover {
        let recovering_id = select_actor(
            &ratings,
            defense,
            ActorRole::Defender,
            None,
            next.rng_mut(),
        )?;
        next.turnover(defense.team_id())?;
        next.set_carrier(recovering_id)?;
        events.push(next.emit(MatchEvent::Turnover(Turnover::new(
            possessor_id,
            defense.team_id(),
            Some(recovering_id),
            Some(holder_id),
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
