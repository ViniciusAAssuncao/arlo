use crate::domain::season::{HomeAwayRecord, StandingsEntry};
use arlo_domain::QtaWeightingPolicy;

pub fn calculate_qta(home_away: &HomeAwayRecord, played: u32, policy: &QtaWeightingPolicy) -> f64 {
    if played == 0 {
        return 0.0;
    }

    let score = (home_away.home_won() as f64 * policy.home_win_weight())
        + (home_away.away_won() as f64 * policy.away_win_weight())
        + (home_away.home_drawn() as f64 * policy.home_draw_weight())
        + (home_away.away_drawn() as f64 * policy.away_draw_weight())
        + (home_away.home_lost() as f64 * policy.home_loss_weight())
        + (home_away.away_lost() as f64 * policy.away_loss_weight());

    score / played as f64
}

pub fn apply_qta_to_standings(entries: &mut [StandingsEntry], policy: &QtaWeightingPolicy) {
    for entry in entries.iter_mut() {
        let qta = calculate_qta(&entry.home_away(), entry.played(), policy);
        *entry = entry.with_qta(qta);
    }
}
