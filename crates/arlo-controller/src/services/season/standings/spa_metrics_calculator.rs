use crate::domain::season::{SpaMetrics, StandingsEntry};
use arlo_domain::SpaScoringPolicy;

pub fn calculate_spa_metrics(
    entry: &StandingsEntry,
    policy: &SpaScoringPolicy,
) -> SpaMetrics {
    if entry.played() == 0 {
        return SpaMetrics::default();
    }

    let j = entry.played() as f64;
    let sg = entry.goal_points_for() as f64 - entry.goal_points_against() as f64;
    let p = (entry.won() as f64 * policy.win_weight())
        + (entry.drawn() as f64 * policy.draw_weight())
        + (entry.lost() as f64 * policy.loss_weight());
    let pb = p / j;
    let tanh_val = (sg / (j * policy.feo_k_factor())).tanh();
    let feo = 1.0 + tanh_val;
    let ispa = pb * feo;

    SpaMetrics::new(pb, feo, ispa)
}

pub fn apply_spa_metrics_to_standings(
    entries: &mut [StandingsEntry],
    policy: &SpaScoringPolicy,
) {
    for entry in entries.iter_mut() {
        let metrics = calculate_spa_metrics(entry, policy);
        *entry = entry.with_spa_metrics(metrics);
    }
}
