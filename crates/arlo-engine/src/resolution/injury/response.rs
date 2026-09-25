use crate::error::EngineResult;
use crate::input::{MatchInput, TeamInput};
use crate::state::{MatchState, TeamState};
use arlo_events::{
    AvailabilityStatus, InjuryIncidentRecorded, MatchClockInstant, MatchEvent,
    MatchEventEnvelope, PlayerAvailabilityChanged, SubstitutionMade, SubstitutionReason,
};
use uuid::Uuid;

pub(super) fn apply_injury(
    input: &MatchInput,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    incident: InjuryIncidentRecorded,
) -> EngineResult<()> {
    let team_id = incident.team_id();
    let (team, team_state) = if team_id == input.home().team_id() {
        (input.home(), state.home())
    } else {
        (input.away(), state.away())
    };
    let withdraw = input.injury_catalog()
        .requires_withdrawal(incident.injury_definition_id(), incident.severity_grade());
    let replacement_id = if withdraw {
        select_replacement(team, team_state, incident.player_id())
    } else {
        None
    };
    state.record_injury(team_id, incident.player_id(), withdraw, replacement_id)?;
    events.push(state.emit(MatchEvent::InjuryIncidentRecorded(incident.clone()))?);
    if withdraw {
        events.push(state.emit(MatchEvent::PlayerAvailabilityChanged(
            PlayerAvailabilityChanged::new(
                incident.player_id(), team_id,
                AvailabilityStatus::Active, AvailabilityStatus::Injured, None,
            ),
        ))?);
        if let Some(player_in) = replacement_id {
            let clock = MatchClockInstant::with_total_elapsed_seconds(
                state.clock().period(),
                state.clock().seconds_in_period(),
                state.clock().total_elapsed_seconds(),
            );
            events.push(state.emit(MatchEvent::SubstitutionMade(SubstitutionMade::new(
                team_id, incident.player_id(), player_in, clock, SubstitutionReason::Injury,
            )))?);
        }
    }
    Ok(())
}

fn select_replacement(team: &TeamInput, state: &TeamState, injured_id: Uuid) -> Option<Uuid> {
    let position = team.lineup().assignments().iter()
        .find(|assignment| state.slot_player_id(assignment.player_id()) == injured_id)
        .map(|assignment| assignment.position())?;
    state.reserve_player_ids().iter().filter_map(|id| {
        let player = team.roster().iter().find(|player| player.id() == *id)?;
        let proficiency = player.positions().iter()
            .find(|candidate| candidate.position() == position)
            .map(|candidate| candidate.proficiency())
            .unwrap_or(0);
        Some((*id, proficiency))
    })
    .max_by_key(|(_, proficiency)| *proficiency)
    .map(|(id, _)| id)
}
