use super::valuation::manager_value;
use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::state::MatchState;
use arlo_domain::AttributeKey;
use arlo_events::{ChallengeResolved, RefereeDecisionResolved, ReviewableCallKind};
use rand::Rng;

pub(in crate::resolution) fn review_decision(
    input: &MatchInput,
    state: &mut MatchState,
    decision: RefereeDecisionResolved,
) -> EngineResult<(RefereeDecisionResolved, Option<ChallengeResolved>)> {
    if decision.peace_referee_intervened() { return Ok((decision, None)); }
    let team_id = if decision.final_call() { decision.offending_team_id() }
        else { decision.opposing_team_id() };
    let team = if team_id == input.home().team_id() { input.home() } else { input.away() };
    if team.manager().is_human_controlled()
        || state.challenges_used(team_id)? >= input.format().challenges_per_match() {
        return Ok((decision, None));
    }
    let judgment = manager_value(input, team.manager(), AttributeKey::ChallengeJudgment);
    let discipline = manager_value(input, team.manager(), AttributeKey::Discipline);
    let signal = if decision.final_call_correct() {
        (0.15 - judgment * 0.004 + (10.0 - discipline) * 0.003).clamp(0.02, 0.20)
    } else {
        (0.38 + judgment * 0.015 + (10.0 - discipline) * 0.003).clamp(0.30, 0.72)
    };
    let time = state.clock().total_elapsed_seconds() / 7200.0;
    let late_weight = (0.55 + time * 0.45).clamp(0.55, 1.0);
    let missed_weight = if decision.final_call() { 1.0 } else { 0.72 };
    let remaining = input.format().challenges_per_match() - state.challenges_used(team_id)?;
    let scarcity = if remaining == 1 && time < 0.75 { 0.68 } else { 1.0 };
    if state.rng_mut().gen_range(0.0..1.0) >= signal * late_weight * missed_weight * scarcity {
        return Ok((decision, None));
    }
    let remaining_after = state.record_challenge(team_id, input.format().challenges_per_match())?;
    let success = !decision.final_call_correct();
    let event = ChallengeResolved::new(team_id, ReviewableCallKind::FoulClassification, success, remaining_after);
    let reviewed = if success {
        RefereeDecisionResolved::new(
            decision.offending_player_id(), decision.offending_team_id(),
            decision.opposing_player_id(), decision.opposing_team_id(),
            decision.origin(), decision.fault_definition_id(),
            decision.factual_foul(), decision.original_call(), true,
        )
    } else { decision };
    Ok((reviewed, Some(event)))
}
