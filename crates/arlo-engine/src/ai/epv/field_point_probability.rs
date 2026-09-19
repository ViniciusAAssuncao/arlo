use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
};

pub fn calculate_field_point_probability(
    offensive_gravity: f64,
    normalized_x: f64,
    drives_in_series: u32,
    down: u8,
    remaining_advance_mirim: f64,
) -> f64 {
    if drives_in_series < FIELD_POINT_REQUIRED_DRIVES {
        return 0.0;
    }
    let advance_in_series = (10.0 - remaining_advance_mirim).max(0.0);
    let is_in_range = advance_in_series >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
        || normalized_x >= 0.65;
    if !is_in_range {
        return 0.0;
    }
    let x = normalized_x.clamp(0.0, 1.0);
    let down_penalty = ((down.clamp(1, 4) - 1) as f64) * 0.04;
    let dist_penalty = (remaining_advance_mirim.max(0.0) / 25.0).clamp(0.0, 0.15);
    let grav_bonus = (offensive_gravity - 1.0) * 0.08;
    (0.28 + x * 0.62 - down_penalty - dist_penalty + grav_bonus).clamp(0.0, 0.94)
}