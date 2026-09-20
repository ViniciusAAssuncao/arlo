pub fn calculate_strength_z_gap(
    attacker_power: f64,
    defender_power: f64,
    attacker_mean: f64,
    attacker_stddev: f64,
    defender_mean: f64,
    defender_stddev: f64,
) -> f64 {
    let z_att = if attacker_stddev > 0.0 { (attacker_power - attacker_mean) / attacker_stddev } else { 0.0 };
    let z_def = if defender_stddev > 0.0 { (defender_power - defender_mean) / defender_stddev } else { 0.0 };
    z_att - z_def
}