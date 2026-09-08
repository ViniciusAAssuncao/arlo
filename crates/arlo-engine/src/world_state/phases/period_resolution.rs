use crate::world_state::match_state::MatchState;

pub fn resolve_period_end(state: &mut MatchState) {
    let current_period = state.clock().period();
    let reg_periods = state.format_rules().regulation_periods();
    let allows_ot = state.format_rules().allows_overtime();
    let ot_periods = state.format_rules().overtime_periods();

    if current_period < reg_periods {
        state.clock_mut().next_period();
    } else if current_period == reg_periods {
        let home_pts = state.home_score().total_points;
        let away_pts = state.away_score().total_points;
        if home_pts == away_pts && allows_ot {
            state.clock_mut().next_period();
        } else {
            state.clock_mut().finish_match();
        }
    } else if current_period < reg_periods + ot_periods {
        state.clock_mut().next_period();
    } else {
        state.clock_mut().finish_match();
    }
}
