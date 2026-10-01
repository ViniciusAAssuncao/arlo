use super::model::SampledCall;
use super::reception::ReceptionSample;
use crate::error::EngineResult;
use crate::state::MatchState;
use arlo_events::{DuelKind, DuelResolved, MatchEvent, MatchEventEnvelope};
use arlo_math::Probability;
use uuid::Uuid;

pub(super) fn emit_route_contest(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    receiver_id: Uuid,
    defender_id: Uuid,
    reception: ReceptionSample,
    attacker_won: bool,
) -> EngineResult<()> {
    if !reception.contested {
        return Ok(());
    }
    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        DuelKind::RouteContest,
        receiver_id,
        defender_id,
        attacker_won,
        Probability::new_clamped(reception.probability),
        reception.probability - 0.5,
    )))?);
    Ok(())
}

pub(super) fn emit_carry_contest(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    carrier_id: Uuid,
    defender_id: Uuid,
    call: SampledCall,
    gain_mirim: f64,
) -> EngineResult<()> {
    if !call.contested {
        return Ok(());
    }
    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        DuelKind::BallSecurityCarry,
        carrier_id,
        defender_id,
        call.successful,
        Probability::new_clamped(call.success_probability),
        gain_mirim,
    )))?);
    Ok(())
}
