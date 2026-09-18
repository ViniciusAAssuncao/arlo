use crate::psychology::math::desperation::apply_desperation_buff;

pub fn calculate_loss_aversion_lambda_with_deficit(
    composure: f64,
    determination: f64,
    bravery: f64,
    exhaustion: f64,
    score_deficit: i32,
) -> f64 {
    let (buffed_det, buffed_brav) = apply_desperation_buff(determination, bravery, score_deficit);
    let norm_comp = composure.clamp(0.0, 20.0) / 10.0;
    let norm_det = buffed_det / 10.0;
    let norm_brav = buffed_brav / 10.0;

    let base_lambda = 2.10
        - 0.35 * (norm_comp - 1.0)
        - 0.40 * (norm_det - 1.0)
        - 0.30 * (norm_brav - 1.0)
        + 0.35 * exhaustion.clamp(0.0, 1.0);

    let max_lambda = if score_deficit > 0 {
        (2.8 - ((score_deficit as f64) * 0.10)).clamp(1.5, 2.8)
    } else {
        3.2
    };

    base_lambda.clamp(1.1, max_lambda)
}

pub fn calculate_loss_aversion_lambda(
    composure: f64,
    determination: f64,
    bravery: f64,
    exhaustion: f64,
) -> f64 {
    calculate_loss_aversion_lambda_with_deficit(composure, determination, bravery, exhaustion, 0)
}