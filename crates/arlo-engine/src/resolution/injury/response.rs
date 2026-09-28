use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::state::MatchState;
use arlo_events::{
    AvailabilityStatus, InjuryIncidentRecorded, MatchClockInstant, MatchEvent,
    MatchEventEnvelope, PlayerAvailabilityChanged, SubstitutionMade, SubstitutionReason,
};

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
    let human_controlled = team.manager().is_human_controlled();
    let reserve_count = team_state.reserve_player_ids().len();
    let pending_replacements = state.pending_forced_substitutions().iter()
        .filter(|(pending_team, _)| *pending_team == team_id).count();
    let replacement_id = if withdraw && !human_controlled {
        super::manager_ai::best_reserve(input, team, team_state, incident.player_id())
            .map(|(id, _)| id)
    } else {
        None
    };
    state.record_injury(team_id, incident.player_id(), withdraw, replacement_id)?;
    events.push(state.emit(MatchEvent::InjuryIncidentRecorded(incident.clone()))?);
    if !withdraw && state.phase() != crate::state::MatchPhase::Finished {
        state.queue_injury_decision(team_id, incident.player_id(), incident.injury_definition_id(), incident.severity_grade());
    }
    if withdraw && human_controlled && pending_replacements < reserve_count
        && state.phase() != crate::state::MatchPhase::Finished {
        state.queue_forced_substitution(team_id, incident.player_id());
    }
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
