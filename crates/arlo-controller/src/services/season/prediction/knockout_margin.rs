use arlo_persistence::repositories::season::forecast_calibrations::ForecastHistoryRow;

pub(super) fn fit_knockout_margin(
    rows: &[ForecastHistoryRow],
    prior_mean: f64,
    prior_stddev: f64,
) -> (f64, f64) {
    let mut count = 0_usize;
    let mut mean = 0.0;
    let mut squared_deviations = 0.0;
    for row in rows {
        if row.home_score < 0 || row.away_score < 0 {
            continue;
        }
        let margin = row.home_score.abs_diff(row.away_score) as f64;
        if margin == 0.0 {
            continue;
        }
        count += 1;
        let difference = margin - mean;
        mean += difference / count as f64;
        squared_deviations += difference * (margin - mean);
    }
    if count < 100 {
        return (prior_mean, prior_stddev);
    }
    let weight = count as f64 / (count as f64 + 40.0);
    let observed_variance = squared_deviations / (count - 1) as f64;
    let fitted_mean = prior_mean + weight * (mean - prior_mean);
    let fitted_variance =
        prior_stddev.powi(2) + weight * (observed_variance - prior_stddev.powi(2));
    (fitted_mean, fitted_variance.sqrt())
}
