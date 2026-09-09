use arlo_domain::sport_constants::{
    LEVERAGE_DEFICIT_DECAY, LEVERAGE_DRIVE_SCARCITY_WEIGHT, LEVERAGE_MAX,
};

pub fn compute_leverage(
    my_score: u32,
    opponent_score: u32,
    urgency_index: f64,
    drive_scarcity_component: f64,
) -> f64 {
    let deficit = ((opponent_score as f64) - (my_score as f64)).abs();
    let closeness = 1.0 / (1.0 + deficit * LEVERAGE_DEFICIT_DECAY);
    let leverage = closeness
        * urgency_index
        * (1.0 + drive_scarcity_component * LEVERAGE_DRIVE_SCARCITY_WEIGHT);
    leverage.clamp(0.0, LEVERAGE_MAX)
}