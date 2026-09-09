use crate::manager_ai::context::ManagerDecisionContext;
use arlo_domain::sport_constants::time_management::{
    TIME_CALL_ENDGAME_RESERVE_BIAS, TIME_CALL_ENDGAME_THRESHOLD_SECONDS,
    TIME_CALL_FATIGUE_URGENCY_WEIGHT, TIME_CALL_JUST_CONCEDED_BONUS, TIME_CALL_LEVERAGE_WEIGHT,
    TIME_CALL_MOMENTUM_URGENCY_WEIGHT, TIME_CALL_PRESSURE_WEIGHT,
};

pub fn compute_urgency(context: &ManagerDecisionContext, just_conceded: bool) -> f64 {
    let fatigue_urgency = (1.0 - context.squad_fatigue_summary.mean_w_prime_balance)
        .clamp(0.0, 1.0)
        * TIME_CALL_FATIGUE_URGENCY_WEIGHT;

    let momentum_urgency = if just_conceded {
        TIME_CALL_MOMENTUM_URGENCY_WEIGHT + TIME_CALL_JUST_CONCEDED_BONUS
    } else {
        0.0
    };

    let pressure_urgency = context.game_state_pressure.urgency_index() * TIME_CALL_PRESSURE_WEIGHT;
    let leverage_urgency = context.situational_awareness.leverage() * TIME_CALL_LEVERAGE_WEIGHT;
    let deficit_delta_adjustment = context.situational_awareness.projected_deficit_delta() * 0.10;

    let mut urgency = fatigue_urgency
        + momentum_urgency
        + pressure_urgency
        + leverage_urgency
        + deficit_delta_adjustment;

    if context.remaining_time_calls == 1 {
        let total_rem_seconds = context.game_state_pressure.total_remaining_seconds();
        if total_rem_seconds > TIME_CALL_ENDGAME_THRESHOLD_SECONDS {
            let normalized_tcm = (context
                .manager_snapshot
                .time_call_management
                .clamp(0.0, 20.0))
                / 20.0;
            let time_ratio = (total_rem_seconds / 3600.0).clamp(0.0, 1.0);
            let reserve_penalty = TIME_CALL_ENDGAME_RESERVE_BIAS * time_ratio * normalized_tcm;
            urgency = (urgency - reserve_penalty).max(0.0);
        }
    }

    urgency.max(0.0)
}
