use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::availability::PlayerAvailabilityTracker;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::Position;
use uuid::Uuid;

pub fn detect_forced_departures(
    lineup: &Lineup,
    availability: &PlayerAvailabilityTracker,
) -> Vec<(Uuid, Position)> {
    lineup
        .assignments()
        .iter()
        .filter(|a| {
            let pid = a.player().id();
            availability.availability_for(&pid).is_injured()
        })
        .map(|a| (a.player().id(), a.slot().position()))
        .collect()
}

pub fn forced_departures_for_team(state: &MatchState, team_id: Uuid) -> Vec<(Uuid, Position)> {
    let is_home = team_id == state.home_team_id();
    let lineup = if is_home {
        state.home_lineup()
    } else {
        state.away_lineup()
    };
    detect_forced_departures(lineup, state.availability())
}