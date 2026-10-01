use crate::error::{EngineError, EngineResult};
use crate::input::{MatchInput, TeamInput};
use crate::state::MatchState;
use crate::step::StepResult;
use arlo_domain::{Position, PunishmentKind, SlotRole};
use arlo_events::{
    AvailabilityStatus, FoulOrigin, FoulRaised, MatchEvent, PlayerAvailabilityChanged,
    PunishmentApplied, RefereeDecisionResolved,
};
use arlo_tactics::max_concurrent_count;
use std::collections::HashSet;
use uuid::Uuid;

pub(super) fn review_initial_lineups(input: &MatchInput, state: &mut MatchState) -> EngineResult<StepResult> {
    let mut next = state.clone();
    next.mark_initial_lineup_reviewed();
    let definition = input.fault_catalog().definitions_by_id().values()
        .find(|definition| definition.code() == "illegal_substitution")
        .ok_or_else(|| EngineError::InvalidInput("illegal_substitution is absent from fault catalog".into()))?;
    let definition_id = definition.id();
    let mut events = Vec::new();
    for (team, opponent) in [(input.home(), input.away()), (input.away(), input.home())] {
        let offenders = excess_designations(team);
        let artrines: Vec<_> = team.lineup().assignments().iter()
            .filter(|assignment| assignment.position() == Position::Artrine)
            .map(|assignment| assignment.player_id()).collect();
        let opposing_player = opponent.lineup().assignments()[0].player_id();
        for player_id in offenders {
            next.apply_punishment(team.team_id(), player_id, PunishmentKind::Expulsion, None)?;
            events.push(next.emit(MatchEvent::RefereeDecisionResolved(RefereeDecisionResolved::new(
                player_id, team.team_id(), opposing_player, opponent.team_id(), FoulOrigin::Lineup,
                Some(definition_id), true, true, false,
            )))?);
            events.push(next.emit(MatchEvent::FoulRaised(FoulRaised::new(
                player_id, team.team_id(), opposing_player, opponent.team_id(), FoulOrigin::Lineup,
                true, false, Some(definition_id), Some(PunishmentKind::Expulsion), None,
            )))?);
            events.push(next.emit(MatchEvent::PunishmentApplied(PunishmentApplied::new(
                player_id, team.team_id(), Some(definition_id), PunishmentKind::Expulsion, None,
            )))?);
            events.push(next.emit(MatchEvent::PlayerAvailabilityChanged(PlayerAvailabilityChanged::new(
                player_id, team.team_id(), AvailabilityStatus::Active, AvailabilityStatus::Expelled, None,
            )))?);
        }
        if !artrines.iter().any(|player_id| {
            let active = if team.team_id() == input.home().team_id() { next.home() } else { next.away() };
            active.active_player_ids().contains(player_id)
        }) {
            next.disable_team_drives(team.team_id())?;
        }
    }
    *state = next;
    Ok(StepResult::resolved(events))
}

fn excess_designations(team: &TeamInput) -> Vec<Uuid> {
    let assignments = team.lineup().assignments();
    let mut offenders = HashSet::new();
    for position in [Position::Artrine, Position::Passer, Position::Goalguard] {
        let designated: Vec<_> = assignments.iter().filter(|assignment| assignment.position() == position)
            .map(|assignment| assignment.player_id()).collect();
        if designated.len() > 1 { offenders.extend(designated); }
    }
    for role in [SlotRole::FalseArtrine, SlotRole::Launcher, SlotRole::Safeguard, SlotRole::Kicker, SlotRole::Blocker] {
        let Some(limit) = max_concurrent_count(role) else { continue };
        let designated: Vec<_> = assignments.iter().filter(|assignment| assignment.slot_role() == role)
            .map(|assignment| assignment.player_id()).collect();
        if designated.len() > limit as usize { offenders.extend(designated); }
    }
    assignments.iter().map(|assignment| assignment.player_id())
        .filter(|id| offenders.contains(id)).collect()
}
