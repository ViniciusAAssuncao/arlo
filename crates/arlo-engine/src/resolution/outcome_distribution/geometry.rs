pub fn finish_distance_multiplier(normalized_proximity: f64) -> f64 {
    let p = normalized_proximity.clamp(0.0, 1.0);
    (0.10 + 0.95 * p.powf(2.4)).clamp(0.10, 1.05)
}

pub fn scoring_distance_adjustment(territory_advance_mirim: f64, is_valid: bool) -> f64 {
    if !is_valid {
        return -8.0;
    }
    let norm = (territory_advance_mirim / 15.0).clamp(0.0, 1.2);
    (norm - 0.75) * 3.0
}

pub fn zone_defensive_congestion_bonus(normalized_proximity: f64) -> f64 {
    let p = normalized_proximity.clamp(0.0, 1.0);
    if p >= 0.88 {
        1.8
    } else if p >= 0.72 {
        0.6
    } else {
        0.0
    }
}