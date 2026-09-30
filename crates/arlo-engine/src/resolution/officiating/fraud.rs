use super::candidate;
use crate::error::EngineResult;
use crate::input::{MatchInput, TeamInput};
use crate::resolution::ratings::RatingIndex;
use crate::state::MatchState;
use arlo_domain::{AttributeKey, SlotRole};
use arlo_events::{FoulOrigin, RefereeDecisionResolved};
use rand::Rng;
use uuid::Uuid;

pub(super) fn sample_communicator_use(
    input: &MatchInput,
    state: &mut MatchState,
    offense_id: Uuid,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    sample(input, state, offense_id, "working_communicator_non_artrine", FoulOrigin::CallToAction)
}

pub(super) fn sample_false_artro_claim(
    input: &MatchInput,
    state: &mut MatchState,
    receiver_id: Uuid,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let offense_id = if input.home().roster().iter().any(|player| player.id() == receiver_id) {
        input.home().team_id()
    } else {
        input.away().team_id()
    };
    let offense = if offense_id == input.home().team_id() { input.home() } else { input.away() };
    if false_artrine(offense, state) != Some(receiver_id) { return Ok(None) }
    sample(input, state, offense_id, "false_artrine_fraud", FoulOrigin::Drive)
}

fn sample(
    input: &MatchInput,
    state: &mut MatchState,
    offense_id: Uuid,
    code: &str,
    origin: FoulOrigin,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let (offense, defense) = if offense_id == input.home().team_id() {
        (input.home(), input.away())
    } else {
        (input.away(), input.home())
    };
    let Some(offender_id) = false_artrine(offense, state) else { return Ok(None) };
    let Some(defender_id) = defense.lineup().assignments().iter()
        .map(|assignment| current_player(defense, state, assignment.player_id()))
        .find(|player_id| active(defense, state, *player_id)) else { return Ok(None) };
    let Some(definition_id) = input.fault_catalog().definitions_by_id().values()
        .find(|definition| definition.code() == code).map(|definition| definition.id()) else { return Ok(None) };
    let ratings = RatingIndex::new(input, state);
    let decisions = ratings.player_value(offense, offender_id, AttributeKey::Decisions)?;
    let teamwork = ratings.player_value(offense, offender_id, AttributeKey::Teamwork)?;
    let probability = (0.0004 + (20.0 - decisions).max(0.0) * 0.00012
        + (20.0 - teamwork).max(0.0) * 0.00007).clamp(0.0004, 0.004);
    let factual = state.rng_mut().gen_range(0.0..1.0) < probability;
    candidate::resolve_specific_decision(input, state, offender_id, offense.team_id(),
        defender_id, defense.team_id(), origin, Some(definition_id), &[], factual, 0.05)
}

fn false_artrine(team: &TeamInput, state: &MatchState) -> Option<Uuid> {
    team.lineup().assignments().iter()
        .filter(|assignment| assignment.slot_role() == SlotRole::FalseArtrine)
        .map(|assignment| current_player(team, state, assignment.player_id()))
        .find(|player_id| active(team, state, *player_id))
}

fn current_player(team: &TeamInput, state: &MatchState, player_id: Uuid) -> Uuid {
    if team.team_id() == state.home().team_id() {
        state.home().slot_player_id(player_id)
    } else {
        state.away().slot_player_id(player_id)
    }
}

fn active(team: &TeamInput, state: &MatchState, player_id: Uuid) -> bool {
    if team.team_id() == state.home().team_id() {
        state.home().active_player_ids().contains(&player_id)
    } else {
        state.away().active_player_ids().contains(&player_id)
    }
}
