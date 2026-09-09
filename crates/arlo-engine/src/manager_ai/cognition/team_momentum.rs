use crate::world_state::MatchState;
use uuid::Uuid;

pub fn aggregate_team_momentum(state: &MatchState, team_id: Uuid) -> f64 {
    let lineup = if team_id == state.home_team_id() {
        state.home_lineup()
    } else {
        state.away_lineup()
    };
    let players = lineup.players();
    if players.is_empty() {
        return 0.0;
    }
    let current_time = state.clock().to_instant().total_elapsed_seconds();
    let total: f64 = players
        .iter()
        .map(|p| state.impulse_for(&p.id()).momentum_index(current_time))
        .sum();
    total / (players.len() as f64)
}