pub fn calculate_turnover_probability(
    normalized_x: f64,
    down: u8,
    remaining_advance_mirim: f64,
) -> f64 {
    let x = normalized_x.clamp(0.0, 1.0);
    let dist_factor = (remaining_advance_mirim.max(0.0) / 10.0).clamp(0.0, 1.0);
    if down >= 4 {
        (0.65 - x * 0.30 + dist_factor * 0.10).clamp(0.20, 0.90)
    } else {
        (0.11 - x * 0.05 + (down as f64) * 0.04 + dist_factor * 0.03).clamp(0.02, 0.45)
    }
}

pub fn calculate_opponent_epa(normalized_x: f64) -> f64 {
    let opp_x = (1.0 - normalized_x).clamp(0.0, 1.0);
    opp_x * 3.5
}