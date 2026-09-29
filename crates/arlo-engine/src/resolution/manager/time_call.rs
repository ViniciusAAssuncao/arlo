use super::valuation::{manager_value, team_pair};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use arlo_domain::AttributeKey;

pub fn should_use_time_call(input: &MatchInput, state: &MatchState) -> bool {
    if state.phase() != MatchPhase::Live { return false; }
    let team_id = state.possessor_team_id();
    let Some((team, team_state)) = team_pair(input, state, team_id) else { return false };
    if team.manager().is_human_controlled() || team_state.time_calls_used_in_period() >= input.format().time_calls_per_period()
        || state.series().team_id() != team_id || state.series().down() >= 4 { return false; }
    let remaining = state.clock().period_limit_seconds() - state.clock().seconds_in_period();
    let opponent_score = if team_id == input.home().team_id() { state.away().score().total_points() }
        else { state.home().score().total_points() };
    let deficit = i64::from(opponent_score) - i64::from(team_state.score().total_points());
    let judgment = manager_value(input, team.manager(), AttributeKey::TimeCallManagement);
    if deficit <= 0 || remaining <= 15.0 { return false; }
    let deficit = deficit.min(30) as f64;
    let down = f64::from(state.series().down());
    let final_period = state.clock().period() == input.format().regulation_periods();
    if deficit < if final_period { 5.0 } else { 12.0 }
        || team_state.time_calls_used_in_period() >= if final_period { 2 } else { 1 } {
        return false;
    }
    let final_pressure = if final_period { 40.0 + deficit * 2.0 } else { 0.0 };
    let scarcity = f64::from(team_state.time_calls_used_in_period()) * 130.0;
    let window = (75.0 + judgment * 5.0 + deficit * 2.0 + final_pressure
        - down * 25.0 - scarcity).clamp(50.0, 330.0);
    remaining < window
}
