use super::actors::{select_actor, select_receiver, ActorRole};
use super::contest::emit_route_contest;
use super::ratings::RatingIndex;
use super::reception::sample_reception;
use crate::error::EngineResult;
use crate::input::TeamInput;
use crate::state::MatchState;
use arlo_domain::AttributeKey;
use arlo_events::{MatchEvent, MatchEventEnvelope, PassCompleted, ReceptionResolved};
use arlo_tactics::PlayCall;
use rand::Rng;
use uuid::Uuid;

pub(super) enum ExchangeOutcome {
    Retained(Uuid),
    Intercepted(Uuid),
}

pub(super) fn resolve_exchange(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    selected_play_call: Option<&PlayCall>,
    holder_id: Uuid,
    previous_holder_id: Option<Uuid>,
    duration_seconds: f64,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<ExchangeOutcome> {
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let directness = offense.tactics().instructions().in_possession().directness().value();
    let probability = (0.55
        + (0.5 - emphasis.self_carry().value()) * 0.3
        + (emphasis.short_pass().value() - 0.5) * 0.20
        + (emphasis.long_launch().value() - 0.5) * 0.12
        + (emphasis.cross().value() - 0.5) * 0.12
        - directness * 0.08)
        .clamp(0.2, 0.85);
    if duration_seconds < 1.4 || state.rng_mut().gen_range(0.0..1.0) >= probability {
        return Ok(ExchangeOutcome::Retained(holder_id));
    }
    let receiver_id = select_receiver(
        ratings,
        offense,
        holder_id,
        previous_holder_id,
        selected_play_call,
        if offense.team_id() == state.home().team_id() {
            state.home().drive_progress().completed_drives() > 0
        } else {
            state.away().drive_progress().completed_drives() > 0
        },
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
        let pressing = defense
            .tactics()
            .instructions()
            .out_of_possession()
            .pressing_intensity()
            .value();
        let defender_ability =
            ratings.player_value(defense, defender_id, AttributeKey::Anticipation)?;
        let interception_probability = (0.18
            + 0.30 * pressing
            + 0.015 * (defender_ability - 10.0)
            + if reception.contested { 0.18 } else { 0.0 })
        .clamp(0.05, 0.80);
        return Ok(if state.rng_mut().gen_range(0.0..1.0) < interception_probability {
            ExchangeOutcome::Intercepted(defender_id)
        } else {
            ExchangeOutcome::Retained(holder_id)
        });
    }
    events.push(state.emit(MatchEvent::PassCompleted(PassCompleted::new(
        holder_id,
        receiver_id,
        false,
        reception.distance_mirim,
    )))?);
    state.complete_pass(holder_id, receiver_id)?;
    Ok(ExchangeOutcome::Retained(receiver_id))
}
