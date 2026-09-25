mod candidate;
mod penalty;

use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use crate::step::{StepOutcome, StepResult};
use arlo_domain::{FaultSeverity, PunishmentKind};
use arlo_events::{FoulOrigin, FoulRaised, KickFoulAwarded, MatchEvent, PlayerAvailabilityChanged, PunishmentApplied, Turnover};
use arlo_domain::{KickFoulScoringTier, SecondZone};
use uuid::Uuid;
use rand::Rng;

pub(super) fn resolve_officiating(input: &MatchInput, state: &mut MatchState, result: StepResult, prior: Option<&MatchState>) -> EngineResult<StepResult> {
    let (mut events, outcome) = result.into_parts();
    state.release_expired_players();
    let mut scored_teams: Vec<_> = events.iter().filter_map(|envelope| match envelope.event() {
        MatchEvent::GoalPoint(score) => Some(score.team_id()),
        MatchEvent::FieldPoint(score) => Some(score.team_id()),
        MatchEvent::FieldGoal(score) => Some(score.team_id()),
        _ => None,
    }).collect();
    for change in state.take_availability_events() {
        events.push(state.emit(MatchEvent::PlayerAvailabilityChanged(change))?);
    }
    if matches!(outcome, StepOutcome::AwaitingDecision(_)) {
        return Ok(StepResult::awaiting_decision(match outcome { StepOutcome::AwaitingDecision(decisions) => decisions, _ => unreachable!() }));
    }
    let mut serious_stop = false;
    for envelope in &events {
        let sampled = match envelope.event() {
            MatchEvent::DuelResolved(duel) => candidate::sample_decision(input, state, duel)?,
            MatchEvent::CallToActionStarted(call) => {
                let attacker = pick_active(input, state, call.offense_team_id());
                let defender = pick_active(input, state, call.defense_team_id());
                match (attacker, defender) {
                    (Some(a), Some(d)) => candidate::sample_context_decision(input, state, "CallToAction", a, d, FoulOrigin::CallToAction)?,
                    _ => None,
                }
            }
            MatchEvent::DriveRecorded(drive) => {
                let defender_team = other_team(input, team_of(input, drive.artrine_id()));
                pick_active(input, state, defender_team).map(|defender| candidate::sample_context_decision(
                    input, state, "Drive", drive.artrine_id(), defender, FoulOrigin::Drive)).transpose()?.flatten()
            }
            MatchEvent::ScoringAttemptMissed(attempt) => {
                let defender_team = other_team(input, attempt.team_id());
                pick_active(input, state, defender_team).map(|defender| candidate::sample_context_decision(
                    input, state, "ShotAttempt", attempt.scorer_id(), defender, FoulOrigin::ShotAttempt)).transpose()?.flatten()
            }
            MatchEvent::OutOfBounds(out) => {
                let attacker = out.last_player().or_else(|| pick_active(input, state, out.last_possession_team()));
                let defender = pick_active(input, state, other_team(input, out.last_possession_team()));
                match (attacker, defender) {
                    (Some(a), Some(d)) => candidate::sample_context_decision(input, state, "OutOfBounds", a, d, FoulOrigin::OutOfBounds)?,
                    _ => None,
                }
            }
            MatchEvent::ReceptionResolved(reception) if reception.caught() => {
                let defender_team = other_team(input, team_of(input, reception.receiver_id()));
                pick_active(input, state, defender_team).map(|defender| candidate::sample_line_fault(
                    input, state, reception.receiver_id(), defender)).transpose()?.flatten()
            }
            _ => None,
        };
        if let Some(decision) = sampled {
            if decision.final_call() && state.phase() == MatchPhase::Live {
                if let Some(severity) = decision.fault_definition_id().and_then(|id| input.fault_catalog().definition(&id)).map(|def| def.severity()) {
                    if matches!(severity, FaultSeverity::Severe | FaultSeverity::Flagrant) && state.rng_mut().gen_range(0.0..1.0) < 0.18 {
                        serious_stop = true;
                    }
                }
            }
            state.queue_referee_decision(decision);
        }
    }
    let annulled_score = prior.is_some() && !scored_teams.is_empty() && state.has_pending_referee_decisions()
        && state.pending_referee_decisions().iter().any(|decision| {
            decision.origin() == FoulOrigin::LineFault && decision.final_call()
                && scored_teams.contains(&decision.offending_team_id())
        });
    if annulled_score {
        if let Some(prior) = prior {
            state.rollback_scoring_segment(prior)?;
            events.clear();
            scored_teams.clear();
            for change in state.take_availability_events() {
                events.push(state.emit(MatchEvent::PlayerAvailabilityChanged(change))?);
            }
        }
    }
    if serious_stop && !annulled_score {
        let pending = state.pending_call_outcome();
        let position = state.possession().ball_position_mirim();
        state.stop_for_serious_foul()?;
        if let Some(pending) = pending {
            super::down::emit_down_advanced(state, &mut events, pending, position)?;
        }
    }
    if !state.has_pending_referee_decisions() || state.phase() == MatchPhase::Live {
        return Ok(match outcome { StepOutcome::Resolved => StepResult::resolved(events), StepOutcome::Finished => StepResult::finished(events), StepOutcome::AwaitingDecision(_) => unreachable!() });
    }
    for decision in state.take_referee_decisions() {
        events.push(state.emit(MatchEvent::RefereeDecisionResolved(decision.clone()))?);
        if !decision.final_call() { continue; }
        let mut punishments = decision.fault_definition_id().map(|id| penalty::select_punishments(input, state, id))
            .unwrap_or_else(|| if decision.origin() == arlo_events::FoulOrigin::LineFault { vec![(PunishmentKind::LossOfDown, None)] } else { Vec::new() });
        if scored_teams.contains(&decision.opposing_team_id()) {
            punishments.retain(|(kind, _)| *kind != PunishmentKind::KickFoulAwarded);
        }
        let first = punishments.first().copied();
        events.push(state.emit(MatchEvent::FoulRaised(FoulRaised::new(
            decision.offending_player_id(), decision.offending_team_id(), decision.opposing_player_id(), decision.opposing_team_id(),
            decision.origin(), decision.original_call_correct(), decision.peace_referee_intervened(), decision.fault_definition_id(),
            first.map(|(kind, _)| kind), first.and_then(|(_, magnitude)| magnitude),
        )))?);
        for (kind, magnitude) in punishments {
            let prior_possessor = state.possessor_team_id();
            let previous_status = state.availability_status(decision.offending_team_id(), decision.offending_player_id());
            state.apply_punishment(decision.offending_team_id(), decision.offending_player_id(), kind, magnitude)?;
            events.push(state.emit(MatchEvent::PunishmentApplied(PunishmentApplied::new(
                decision.offending_player_id(), decision.offending_team_id(), decision.fault_definition_id(), kind, magnitude,
            )))?);
            let new_status = state.availability_status(decision.offending_team_id(), decision.offending_player_id());
            if let (Some(previous), Some(new)) = (previous_status, new_status) {
                if previous != new {
                    let remaining = if kind == PunishmentKind::TimePenalty { Some(f64::from(magnitude.unwrap_or(2).max(0)) * 60.0) } else { None };
                    events.push(state.emit(MatchEvent::PlayerAvailabilityChanged(PlayerAvailabilityChanged::new(
                        decision.offending_player_id(), decision.offending_team_id(), previous, new, remaining,
                    )))?);
                }
            }
            if state.possessor_team_id() != prior_possessor {
                events.push(state.emit(MatchEvent::Turnover(Turnover::new(
                    prior_possessor, state.possessor_team_id(), None, Some(decision.offending_player_id()), false,
                )))?);
            }
            if kind == PunishmentKind::KickFoulAwarded && state.phase() == MatchPhase::Stopped
                && state.clock().seconds_in_period() < state.clock().maximum_period_seconds() {
                let awarded_team = decision.opposing_team_id();
                state.award_kick_foul(awarded_team)?;
                let position = state.possession().ball_position_mirim();
                let distance = if awarded_team == input.home().team_id() { input.pitch().length_mirim() - position } else { position };
                let zone = input.pitch().zone_at_distance_to_goal(distance, SecondZone::default_awc().depth_mirim());
                events.push(state.emit(MatchEvent::KickFoulAwarded(KickFoulAwarded::new(
                    awarded_team, decision.offending_team_id(), KickFoulScoringTier::from_zone(zone),
                )))?);
            }
        }
    }
    Ok(match outcome { StepOutcome::Resolved => StepResult::resolved(events), StepOutcome::Finished if !annulled_score => StepResult::finished(events), StepOutcome::Finished => StepResult::resolved(events), StepOutcome::AwaitingDecision(_) => unreachable!() })
}

fn team_of(input: &MatchInput, player_id: Uuid) -> Uuid {
    if input.home().roster().iter().any(|player| player.id() == player_id) { input.home().team_id() }
    else { input.away().team_id() }
}

fn other_team(input: &MatchInput, team_id: Uuid) -> Uuid {
    if team_id == input.home().team_id() { input.away().team_id() } else { input.home().team_id() }
}

fn pick_active(input: &MatchInput, state: &mut MatchState, team_id: Uuid) -> Option<Uuid> {
    let team = if team_id == input.home().team_id() { input.home() } else { input.away() };
    let active = if team_id == input.home().team_id() { state.home().active_player_ids() } else { state.away().active_player_ids() };
    let team_state = if team_id == input.home().team_id() { state.home() } else { state.away() };
    let candidates: Vec<_> = team.lineup().assignments().iter()
        .filter(|assignment| assignment.position().line() != arlo_domain::PositionLine::Goalguard
            && active.contains(&team_state.slot_player_id(assignment.player_id())))
        .map(|assignment| team_state.slot_player_id(assignment.player_id())).collect();
    if candidates.is_empty() { None } else { Some(candidates[state.rng_mut().gen_range(0..candidates.len())]) }
}
