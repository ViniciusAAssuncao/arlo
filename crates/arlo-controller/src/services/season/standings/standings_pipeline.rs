use crate::domain::season::{Fixture, StandingsEntry};
use crate::services::season::standings::qta_calculator::apply_qta_to_standings;
use crate::services::season::standings::spa_metrics_calculator::apply_spa_metrics_to_standings;
use crate::services::season::standings::standings_calculator::calculate_standings;
use crate::services::season::standings::tie_break_resolver::sort_standings;
use arlo_domain::{QtaWeightingPolicy, SpaScoringPolicy, TieBreakCriterion};
use uuid::Uuid;

pub fn calculate_and_rank_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    spa_policy: &SpaScoringPolicy,
    qta_policy: &QtaWeightingPolicy,
    criteria: &[TieBreakCriterion],
) -> Vec<StandingsEntry> {
    let mut entries = calculate_standings(team_ids, fixtures);
    apply_spa_metrics_to_standings(&mut entries, spa_policy);
    apply_qta_to_standings(&mut entries, qta_policy);
    sort_standings(entries, criteria)
}
