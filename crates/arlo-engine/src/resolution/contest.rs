use super::model::SampledCall;
use super::reception::ReceptionSample;
use crate::error::EngineResult;
use crate::state::MatchState;
use arlo_events::{DuelKind, DuelResolved, MatchEvent, MatchEventEnvelope, ScoringPost};
use arlo_math::Probability;
use uuid::Uuid;

pub(super) fn emit_reception_contest(
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

    emit_distribution_contest(
        state,
        events,
        receiver_id,
        defender_id,
        reception.intent.duel_kind(reception.is_aerial),
        attacker_won,
        reception.probability,
    )
}

pub(super) fn emit_distribution_contest(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    receiver_id: Uuid,
    defender_id: Uuid,
    kind: DuelKind,
    attacker_won: bool,
    success_probability: f64,
) -> EngineResult<()> {
    let net_advantage = outcome_advantage(attacker_won, success_probability, 0.0);

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        kind,
        receiver_id,
        defender_id,
        attacker_won,
        Probability::new_clamped(success_probability),
        net_advantage,
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
    } else if sample.breakthrough_attempted {
        DuelKind::RunBreakthrough
    } else {
        DuelKind::BallSecurityCarry
    };

    let gain_signal = (gain_mirim.abs() / 12.0).min(0.50);
    let net_advantage =
        outcome_advantage(sample.successful, sample.success_probability, gain_signal);

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        kind,
        carrier_id,
        defender_id,
        sample.successful,
        Probability::new_clamped(sample.success_probability),
        net_advantage,
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

fn outcome_advantage(attacker_won: bool, probability: f64, extra: f64) -> f64 {
    let surprise = if attacker_won {
        1.0 - probability
    } else {
        probability
    };
    let magnitude = surprise * 2.0 + extra.max(0.0);
    if attacker_won {
        magnitude
    } else {
        -magnitude
    }
}
