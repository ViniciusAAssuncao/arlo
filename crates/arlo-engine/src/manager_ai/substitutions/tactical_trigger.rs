use crate::manager_ai::context::ManagerDecisionContext;
use arlo_domain::sport_constants::substitution::SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT;

pub fn tactical_urgency(context: &ManagerDecisionContext) -> f64 {
    let deficit = context.game_state_pressure.score_deficit();
    let deficit_factor = if deficit > 0 {
        ((deficit as f64) / 10.0).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let pressure_urgency = (context.game_state_pressure.urgency_index() / 5.0).clamp(0.0, 1.0);
    let normalized_iga = (context.manager_snapshot.in_game_adjustments.clamp(0.0, 20.0)) / 20.0;

    let raw_urgency = (deficit_factor * SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT)
        + (pressure_urgency * (1.0 - SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT));

    (raw_urgency * (0.5 + 0.5 * normalized_iga)).clamp(0.0, 1.0)
}