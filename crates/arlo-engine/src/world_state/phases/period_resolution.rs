use crate::world_state::match_state::MatchState;

fn advance_to_next_period(state: &mut MatchState) {
    state.clock_mut().next_period();
    state.reset_added_time_tracker_for_new_period();
}

pub fn resolve_period_end(state: &mut MatchState) {
    let current_period = state.clock().period();
    let reg_periods = state.format_rules().regulation_periods();
    let allows_ot = state.format_rules().allows_overtime();
    let ot_periods = state.format_rules().overtime_periods();

    if current_period < reg_periods {
        advance_to_next_period(state);
    } else if current_period == reg_periods {
        let home_pts = state.home_score().total_points;
        let away_pts = state.away_score().total_points;
        if home_pts == away_pts && allows_ot {
            advance_to_next_period(state);
        } else {
            state.clock_mut().finish_match();
        }
    } else if current_period < reg_periods + ot_periods {
        advance_to_next_period(state);
    } else {
        state.clock_mut().finish_match();
    }
}