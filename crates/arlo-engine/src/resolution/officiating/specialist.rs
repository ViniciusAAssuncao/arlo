use super::candidate;
use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::state::MatchState;
use arlo_domain::PitchZone;
use arlo_events::{FoulOrigin, GoalguardRecoveryResolved, PasserContactResolved, RefereeDecisionResolved};

pub(super) fn review_passer_contact(
    input: &MatchInput,
    state: &mut MatchState,
    contact: &PasserContactResolved,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let defender_is_home = input.home().roster().iter().any(|player| player.id() == contact.defender_id());
    let (defense, offense) = if defender_is_home { (input.home(), input.away()) } else { (input.away(), input.home()) };
    let code = if contact.violent() {
        "roughing_passer_late"
    } else if contact.rough() {
        "roughing_the_passer"
    } else {
        "passer_contact_late"
    };
    let Some(definition_id) = definition_id(input, code) else { return Ok(None) };
    candidate::resolve_specific_decision(input, state, contact.defender_id(), defense.team_id(),
        contact.passer_id(), offense.team_id(), FoulOrigin::CallToAction,
        Some(definition_id), &[], contact.late(), 0.5)
}

pub(super) fn review_goalguard_handling(
    input: &MatchInput,
    state: &mut MatchState,
    recovery: &GoalguardRecoveryResolved,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let Some(definition_id) = definition_id(input, "illegal_goalguard_handling") else { return Ok(None) };
    let opponent = if recovery.team_id() == input.home().team_id() { input.away() } else { input.home() };
    let opponent_state = if opponent.team_id() == input.home().team_id() { state.home() } else { state.away() };
    let Some(opposing_id) = opponent.lineup().assignments().iter()
        .map(|assignment| opponent_state.slot_player_id(assignment.player_id()))
        .find(|player_id| opponent_state.active_player_ids().contains(player_id)) else { return Ok(None) };
    let factual = recovery.zone() != PitchZone::FirstZone && recovery.used_hands();
    let false_call_scale = if recovery.zone() == PitchZone::FirstZone { 0.03 } else { 0.25 };
    candidate::resolve_specific_decision(input, state, recovery.goalguard_id(), recovery.team_id(),
        opposing_id, opponent.team_id(), FoulOrigin::ShotAttempt,
        Some(definition_id), &[], factual, false_call_scale)
}

fn definition_id(input: &MatchInput, code: &str) -> Option<uuid::Uuid> {
    input.fault_catalog().definitions_by_id().values()
        .find(|definition| definition.code() == code).map(|definition| definition.id())
}
