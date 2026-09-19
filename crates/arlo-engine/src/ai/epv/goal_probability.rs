use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;

pub fn calculate_goal_probability(
    offensive_gravity: f64,
    normalized_x: f64,
    drives_in_series: u32,
    down: u8,
    remaining_advance_mirim: f64,
) -> f64 {
    if drives_in_series < GOAL_POINT_REQUIRED_DRIVES {
        let drives_needed = GOAL_POINT_REQUIRED_DRIVES - drives_in_series;
        let x = normalized_x.clamp(0.0, 1.0);
        let remaining_downs = 4u8.saturating_sub(down.clamp(1, 4));
        if (remaining_downs as u32) < drives_needed {
            return 0.0;
        }
        let chain_discount = match drives_needed {
            1 => 0.35,
            2 => 0.10,
            _ => 0.02,
        };
        let base = 0.32 * x.powf(2.8) * chain_discount;
        let dist_penalty = (remaining_advance_mirim.max(0.0) / 20.0).clamp(0.0, 0.20);
        let grav_bonus = (offensive_gravity - 1.0) * 0.03;
        return (base - dist_penalty + grav_bonus).clamp(0.0, 0.20);
    }

    let x = normalized_x.clamp(0.0, 1.0);
    let down_penalty = ((down.clamp(1, 4) - 1) as f64) * 0.09;
    let dist_penalty = (remaining_advance_mirim.max(0.0) / 20.0).clamp(0.0, 0.25);
    let grav_bonus = (offensive_gravity - 1.0) * 0.05;
    let base = 0.36 * x.powf(2.2);
    (base - down_penalty - dist_penalty + grav_bonus).clamp(0.0, 0.42)
}