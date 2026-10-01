use super::model::SampledCall;
use super::reception::ReceptionSample;
use crate::error::EngineResult;
use crate::state::MatchState;
use arlo_events::{DuelKind, DuelResolved, MatchEvent, MatchEventEnvelope, ScoringPost};
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

    let kind = if reception.is_aerial {
        DuelKind::AerialDuel
    } else if reception.distance_mirim >= 18.0 {
        DuelKind::LongDistribution
    } else if reception.distance_mirim <= 8.0 {
        DuelKind::ShortDistribution
    } else {
        DuelKind::RouteContest
    };

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        kind,
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
    sample: SampledCall,
    gain_mirim: f64,
    carrier_is_artrine: bool,
) -> EngineResult<()> {
    if !sample.contested {
        return Ok(());
    }

    let kind = if carrier_is_artrine {
        DuelKind::ArtroBreakthrough
    } else if gain_mirim >= 4.0 {
        DuelKind::RunBreakthrough
    } else {
        DuelKind::BallSecurityCarry
    };

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        kind,
        carrier_id,
        defender_id,
        sample.successful,
        Probability::new_clamped(sample.success_probability),
        gain_mirim,
    )))?);
    Ok(())
}

pub(super) fn emit_shot_contest(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    shooter_id: Uuid,
    defender_id: Uuid,
    post: ScoringPost,
    converted: bool,
    out_of_bounds: bool,
    conversion_probability: f64,
) -> EngineResult<()> {
    if out_of_bounds {
        return Ok(());
    }

    let kind = match post {
        ScoringPost::Goalpost => DuelKind::FinishingAttempt,
        ScoringPost::Fieldpost => DuelKind::FieldGoalAttempt,
    };
    let surprise = if converted {
        1.0 - conversion_probability
    } else {
        conversion_probability
    };
    let net_advantage = if converted {
        surprise * 2.0
    } else {
        -surprise * 2.0
    };

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        kind,
        shooter_id,
        defender_id,
        converted,
        Probability::new_clamped(conversion_probability),
        net_advantage,
    )))?);
    Ok(())
}
