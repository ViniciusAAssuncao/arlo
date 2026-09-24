use super::actors::{select_actor, ActorRole};
use super::artro::sample_artros;
use super::contest::emit_carry_contest;
use super::exchange::{resolve_exchange, ExchangeOutcome};
use super::model::{sample_call, SampledCall};
use super::ratings::{RatingIndex, TeamRatings};
use crate::error::EngineResult;
use crate::input::TeamInput;
use crate::state::MatchState;
use arlo_domain::sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS;
use arlo_events::{
    CarryResolved, DriveRecorded, MatchEvent, MatchEventEnvelope, PossessionTimeRecorded,
};
use arlo_tactics::PlayCall;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct SequenceOutcome {
    pub holder_id: Uuid,
    pub end_mirim: f64,
    pub gain_mirim: f64,
    pub interceptor_id: Option<Uuid>,
}

pub(super) struct SequenceContext<'a> {
    pub ratings: &'a RatingIndex,
    pub offense: &'a TeamInput,
    pub defense: &'a TeamInput,
    pub offense_rating: TeamRatings,
    pub defense_rating: TeamRatings,
    pub is_home: bool,
    pub selected_play_call: Option<&'a PlayCall>,
    pub artrine_id: Uuid,
    pub pitch_length_mirim: f64,
}

pub(super) fn resolve_sequence(
    context: SequenceContext<'_>,
    holder_id: Uuid,
    first_defender_id: Uuid,
    first_sample: SampledCall,
    duration_seconds: f64,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<SequenceOutcome> {
    let start_mirim = state.possession().ball_position_mirim();
    let duration_seconds = duration_seconds.min(
        state.clock().period_limit_seconds() - state.clock().seconds_in_period(),
    );
    if duration_seconds <= 0.0 {
        return Ok(SequenceOutcome {
            holder_id,
            end_mirim: start_mirim,
            gain_mirim: 0.0,
            interceptor_id: None,
        });
    }
    let action_count = ((duration_seconds / 3.5).floor() as usize).clamp(1, 3);
    let action_slice = duration_seconds / action_count as f64;
    let final_instant = (state.clock().seconds_in_period() + duration_seconds)
        .min(state.clock().period_limit_seconds());
    let direction = if context.is_home { 1.0 } else { -1.0 };
    let mut current_holder_id = holder_id;
    let mut previous_holder_id = None;
    let mut end_mirim = start_mirim;
    for action_index in 0..action_count {
        let action_duration = if action_index + 1 == action_count {
            (final_instant - state.clock().seconds_in_period()).max(0.0)
        } else {
            action_slice.min(final_instant - state.clock().seconds_in_period())
        };
        let defender_id = if action_index == 0 {
            first_defender_id
        } else {
            select_actor(
                context.ratings,
                context.defense,
                ActorRole::Defender,
                None,
                state.rng_mut(),
            )?
        };
        let sample = if action_index == 0 {
            first_sample
        } else {
            sample_call(
                context.ratings,
                context.offense,
                context.defense,
                context.offense_rating,
                context.defense_rating,
                current_holder_id,
                defender_id,
                context.is_home,
                context.selected_play_call,
                state.rng_mut(),
            )?
        };
        let next_mirim = (end_mirim + direction * sample.gain_mirim / action_count as f64)
            .clamp(0.0, context.pitch_length_mirim);
        let gain_mirim = direction * (next_mirim - end_mirim);
        state.advance_playing_time(action_duration)?;
        events.push(state.emit(MatchEvent::PossessionTimeRecorded(
            PossessionTimeRecorded::new(context.offense.team_id(), action_duration),
        ))?);
        state.move_live_ball(next_mirim)?;
        events.push(state.emit(MatchEvent::CarryResolved(CarryResolved::new(
            current_holder_id,
            gain_mirim,
        )))?);
        emit_carry_contest(
            state,
            events,
            current_holder_id,
            defender_id,
            sample,
            gain_mirim,
        )?;
        if current_holder_id == context.artrine_id
            && action_duration >= IMMEDIATE_POSSESSION_CONTROL_SECONDS
        {
            let artros = sample_artros(
                context.ratings,
                context.offense,
                context.defense,
                context.selected_play_call,
                action_duration,
                state.rng_mut(),
            )?;
            for placement in artros {
                if let Some(drives_in_series) = state.record_artro(
                    context.offense.team_id(),
                    context.artrine_id,
                    action_duration,
                )? {
                    events.push(state.emit(MatchEvent::DriveRecorded(DriveRecorded::new(
                        context.artrine_id,
                        drives_in_series,
                        placement,
                    )))?);
                }
            }
        }
        end_mirim = next_mirim;
        if action_index + 1 < action_count {
            let exchange = resolve_exchange(
                context.ratings,
                context.offense,
                context.defense,
                context.selected_play_call,
                current_holder_id,
                previous_holder_id,
                action_duration,
                state,
                events,
            )?;
            match exchange {
                ExchangeOutcome::Retained(next_holder_id) => {
                    if next_holder_id != current_holder_id {
                        previous_holder_id = Some(current_holder_id);
                        current_holder_id = next_holder_id;
                    }
                }
                ExchangeOutcome::Intercepted(interceptor_id) => {
                    return Ok(SequenceOutcome {
                        holder_id: current_holder_id,
                        end_mirim,
                        gain_mirim: direction * (end_mirim - start_mirim),
                        interceptor_id: Some(interceptor_id),
                    });
                }
            }
        }
    }
    Ok(SequenceOutcome {
        holder_id: current_holder_id,
        end_mirim,
        gain_mirim: direction * (end_mirim - start_mirim),
        interceptor_id: None,
    })
}
