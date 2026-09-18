pub fn calculate_desperation_multiplier(score_deficit: i32) -> f64 {
    if score_deficit <= 0 {
        1.0
    } else {
        1.0 + ((score_deficit as f64) * 0.08).clamp(0.0, 0.80)
    }
}

pub fn apply_desperation_buff(
    determination: f64,
    bravery: f64,
    score_deficit: i32,
) -> (f64, f64) {
    let mult = calculate_desperation_multiplier(score_deficit);
    let buffed_det = (determination * mult).clamp(0.0, 25.0);
    let buffed_brav = (bravery * mult).clamp(0.0, 25.0);
    (buffed_det, buffed_brav)
}

pub fn desperation_dampener(score_deficit: i32, determination: f64, bravery: f64) -> f64 {
    if score_deficit <= 0 {
        1.0
    } else {
        let norm_grit = ((determination + bravery) / 20.0).clamp(0.5, 2.0);
        let deficit_factor = ((score_deficit as f64) * 0.06).clamp(0.0, 0.60);
        1.0 / (1.0 + deficit_factor * norm_grit)
    }
}

pub fn desperation_floor_offset(score_deficit: i32, baseline: f64) -> f64 {
    if score_deficit <= 0 {
        0.0
    } else {
        let max_offset = baseline * 0.25;
        ((score_deficit as f64) * 0.85).clamp(0.0, max_offset)
    }
}