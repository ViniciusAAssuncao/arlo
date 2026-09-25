use super::actors::{select_actor, ActorRole};
use super::bonus::resolve_bonus_segment;
use super::context::validate_match_state;
use super::contest::emit_route_contest;
use super::down::emit_down_advanced;
use super::kick_foul::resolve_kick_foul_segment_inner;
use super::model::sample_call;
use super::officiating::resolve_officiating;
use super::open_play::resolve_open_play_segment;
use super::ratings::RatingIndex;
use super::reception::sample_reception;
use super::sequence::{resolve_sequence, SequenceContext};
use super::shooting::resolve_regular_attempt;
use super::tuning::CTA_OUT_PROBABILITY;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState, PendingCallOutcome, SeriesAdvance};
use crate::step::StepResult;
use arlo_domain::sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS;
use arlo_events::{
    CallToActionStarted, MatchEvent, OutOfBounds, PassCompleted, PossessionTimeRecorded,
    ReceptionResolved, Turnover,
};
use arlo_manager_control::RequiredManagerDecision;
use arlo_tactics::{validate_play_call, PlayCall, PlayCallCategory};
use rand::Rng;

pub fn resolve_next_segment(
    input: &MatchInput,
    state: &mut MatchState,
    selected_play_call: Option<&PlayCall>,
) -> EngineResult<StepResult> {
    let prior = state.clone();
    let result = resolve_next_segment_inner(input, state, selected_play_call)?;
    resolve_officiating(input, state, result, Some(&prior))
}

fn resolve_next_segment_inner(
    input: &MatchInput,
    state: &mut MatchState,
    selected_play_call: Option<&PlayCall>,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    if state.phase() == MatchPhase::Finished {
        return Ok(StepResult::finished(Vec::new()));
    }
    if state.phase() == MatchPhase::PeriodBreak {
        let mut next = state.clone();
        next.start_next_quarter()?;
        *state = next;
        return Ok(StepResult::resolved(Vec::new()));
    }
    if state.phase() == MatchPhase::BonusPhase {
        if selected_play_call.is_some() {
            return Err(EngineError::InvalidInput(
                "Bonus Phase does not accept an open-play Call-to-Action".into(),
            ));
        }
        return resolve_bonus_segment(input, state);
    }
    if state.phase() == MatchPhase::KickFoul {
        if selected_play_call.is_some() {
            return Err(EngineError::InvalidInput(
                "Kick Foul does not accept an open-play Call-to-Action".into(),
            ));
        }
        return resolve_kick_foul_segment_inner(input, state, None, None);
    }
    if state.phase() == MatchPhase::Live {
        if selected_play_call.is_some() {
            return Err(EngineError::InvalidInput(
                "open play cannot begin another Call-to-Action".into(),
            ));
        }
        return resolve_open_play_segment(input, state);
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

    let ratings = RatingIndex::new(input, state);
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
    let carry_defender_id = select_actor(
        &ratings,
        defense,
        ActorRole::Defender,
        None,
        next.rng_mut(),
    )?;
    let sample = sample_call(
        &ratings,
        offense,
        defense,
        offense_rating,
        defense_rating,
        artrine_id,
        carry_defender_id,
        is_home,
        selected_play_call,
        next.rng_mut(),
    )?;
    let duration = sample.duration_seconds.min(remaining_time);
    let defender_id = select_actor(
        &ratings,
        defense,
        ActorRole::Defender,
        None,
        next.rng_mut(),
    )?;
    let reception = sample_reception(
        &ratings,
        offense,
        defense,
        passer_id,
        artrine_id,
        defender_id,
        next.rng_mut(),
    )?;
    let controlled_reception = reception.caught && duration >= IMMEDIATE_POSSESSION_CONTROL_SECONDS;
    next.begin_call_to_action()?;
    let mut events = Vec::with_capacity(7);
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
    let reception_time = duration.min(IMMEDIATE_POSSESSION_CONTROL_SECONDS);
    next.advance_playing_time(reception_time)?;
    events.push(next.emit(MatchEvent::PossessionTimeRecorded(
        PossessionTimeRecorded::new(offense_id, reception_time),
    ))?);
    events.push(
        next.emit(MatchEvent::ReceptionResolved(ReceptionResolved::new(
            artrine_id,
            passer_id,
            controlled_reception,
            false,
        )))?,
    );
    emit_route_contest(
        &mut next,
        &mut events,
        artrine_id,
        defender_id,
        reception,
        controlled_reception,
    )?;
    if controlled_reception {
        next.complete_pass(passer_id, artrine_id)?;
        events.push(next.emit(MatchEvent::PassCompleted(PassCompleted::new(
            passer_id,
            artrine_id,
            false,
            reception.distance_mirim,
        )))?);
    }
    let (holder_id, end_mirim, gain_mirim, interceptor_id) = if controlled_reception {
        let outcome = resolve_sequence(
            SequenceContext {
                ratings: &ratings,
                offense,
                defense,
                offense_rating,
                defense_rating,
                is_home,
                selected_play_call,
                artrine_id,
                pitch_length_mirim: input.pitch().length_mirim(),
            },
            artrine_id,
            carry_defender_id,
            sample,
            duration - reception_time,
            &mut next,
            &mut events,
        )?;
        (
            outcome.holder_id,
            outcome.end_mirim,
            outcome.gain_mirim,
            outcome.interceptor_id,
        )
    } else {
        let remaining_duration = duration - reception_time;
        if remaining_duration > 0.0 {
            next.advance_playing_time(remaining_duration)?;
            events.push(next.emit(MatchEvent::PossessionTimeRecorded(
                PossessionTimeRecorded::new(offense_id, remaining_duration),
            ))?);
        }
        (artrine_id, start_mirim, 0.0, None)
    };
    let advance = next.record_valid_advance(gain_mirim, end_mirim)?;
    let first_down = matches!(advance, SeriesAdvance::FirstDown);
    let call_outcome = PendingCallOutcome {
        prior_down,
        gain_mirim,
        total_advance_mirim: prior_advance + gain_mirim,
        first_down,
    };
    next.record_call_outcome(call_outcome);
    if let Some(interceptor_id) = interceptor_id {
        next.turnover(defense.team_id())?;
        next.set_carrier(interceptor_id)?;
        events.push(next.emit(MatchEvent::Turnover(Turnover::new(
            offense_id,
            defense.team_id(),
            Some(interceptor_id),
            Some(holder_id),
            true,
        )))?);
        *state = next;
        return Ok(StepResult::resolved(events));
    }
    if controlled_reception
        && resolve_regular_attempt(
            input,
            &ratings,
            offense,
            defense,
            is_home,
            holder_id,
            selected_play_call,
            &mut next,
            &mut events,
        )?
    {
        *state = next;
        return Ok(StepResult::resolved(events));
    }
    let ends_out =
        duration >= remaining_time || next.rng_mut().gen_range(0.0..1.0) < CTA_OUT_PROBABILITY;
    if ends_out {
        next.resolve_out(offense_id, end_mirim)?;
        events.push(next.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
            offense_id, None, false,
        )))?);
        emit_down_advanced(&mut next, &mut events, call_outcome, end_mirim)?;
    }

    *state = next;
    Ok(StepResult::resolved(events))
}
