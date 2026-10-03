use arlo_analytics::prediction::preseason::GoalPointModel;
use arlo_persistence::repositories::season::forecast_calibrations::ForecastHistoryRow;

pub(super) fn fit_goal_point_model(rows: &[ForecastHistoryRow]) -> Option<GoalPointModel> {
    let eligible: Vec<_> = rows
        .iter()
        .filter_map(|row| {
            let home = u32::try_from(row.home_goal_points?).ok()?;
            let away = u32::try_from(row.away_goal_points?).ok()?;
            Some((row, home, away))
        })
        .collect();
    if eligible.len() < 100 {
        return None;
    }
    let mean = eligible
        .iter()
        .map(|(_, home, away)| (*home + *away) as f64)
        .sum::<f64>()
        / (2 * eligible.len()) as f64;
    if mean <= 0.0 {
        return None;
    }
    let mut covariance = 0.0;
    let mut variance = 0.0;
    for (row, home, away) in &eligible {
        let rating_difference = (row.home_rating - row.away_rating) / 400.0;
        covariance += rating_difference * (*home as f64 - *away as f64);
        variance += rating_difference * rating_difference;
    }
    let slope = if variance > 0.0 {
        (covariance / (2.0 * mean * variance)).clamp(-1.0, 1.0)
    } else {
        0.0
    };
    Some(GoalPointModel {
        mean_per_team: mean,
        rating_slope: slope,
    })
}
