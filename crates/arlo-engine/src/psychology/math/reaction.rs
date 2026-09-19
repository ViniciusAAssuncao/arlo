pub fn calculate_reaction_scale(
    determination: f64,
    bravery: f64,
    composure: f64,
    consistency: f64,
    involved: bool,
) -> f64 {
    let norm_det = determination.clamp(0.0, 20.0) / 10.0;
    let norm_brav = bravery.clamp(0.0, 20.0) / 10.0;
    let norm_comp = composure.clamp(0.0, 20.0) / 10.0;
    let norm_cons = consistency.clamp(0.0, 20.0) / 10.0;

    let involvement_factor = if involved { 1.0 } else { 0.45 };
    let mental_drive = (0.4 * norm_det + 0.35 * norm_brav + 0.25 * norm_comp).clamp(0.5, 1.8);
    let stability_dampener = (1.0 / (0.7 + 0.20 * norm_comp + 0.15 * norm_cons)).clamp(0.5, 1.4);

    8.5 * involvement_factor * mental_drive * stability_dampener
}