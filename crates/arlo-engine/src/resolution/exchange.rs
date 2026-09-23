use super::actors::{select_actor, ActorRole};
use super::contest::emit_route_contest;
use super::ratings::RatingIndex;
use super::reception::sample_reception;
use crate::error::EngineResult;
use crate::input::TeamInput;
use crate::state::MatchState;
use arlo_events::{MatchEvent, MatchEventEnvelope, PassCompleted, ReceptionResolved};
use arlo_tactics::PlayCall;
use rand::Rng;
use uuid::Uuid;

pub(super) fn resolve_exchange(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    selected_play_call: Option<&PlayCall>,
    holder_id: Uuid,
    duration_seconds: f64,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<Uuid> {
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let directness = offense.tactics().instructions().in_possession().directness().value();
    let probability = (0.55 + (0.5 - emphasis.self_carry().value()) * 0.3
        - directness * 0.15)
        .clamp(0.2, 0.85);
    if duration_seconds < 1.4 || state.rng_mut().gen_range(0.0..1.0) >= probability {
        return Ok(holder_id);
    }
    let receiver_id = select_actor(
        ratings,
        offense,
        ActorRole::Receiver,
        Some(holder_id),
        state.rng_mut(),
    )?;
    let defender_id = select_actor(
        ratings,
        defense,
        ActorRole::Defender,
        None,
        state.rng_mut(),
    )?;
    let reception = sample_reception(
        ratings,
        offense,
        defense,
        holder_id,
        receiver_id,
        defender_id,
        state.rng_mut(),
    )?;
    events.push(state.emit(MatchEvent::ReceptionResolved(ReceptionResolved::new(
        receiver_id,
        holder_id,
        reception.caught,
        false,
    )))?);
    emit_route_contest(state, events, receiver_id, defender_id, reception, reception.caught)?;
    if !reception.caught {
        return Ok(holder_id);
    }
    events.push(state.emit(MatchEvent::PassCompleted(PassCompleted::new(
        holder_id,
        receiver_id,
        false,
        reception.distance_mirim,
    )))?);
    state.set_carrier(receiver_id)?;
    Ok(receiver_id)
}
