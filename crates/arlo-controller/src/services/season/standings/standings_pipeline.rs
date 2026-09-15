use crate::domain::season::{Fixture, StandingsEntry};
use crate::services::season::standings::head_to_head_resolver::resolve_head_to_head;
use crate::services::season::standings::qta_calculator::apply_qta_to_standings;
use crate::services::season::standings::random_tiebreak_resolver::resolve_random;
use crate::services::season::standings::spa_metrics_calculator::apply_spa_metrics_to_standings;
use crate::services::season::standings::standings_calculator::calculate_standings;
use crate::services::season::standings::tie_group_partitioner::partition_tied_groups;
use arlo_domain::{QtaWeightingPolicy, SpaScoringPolicy, TieBreakCriterion};
use uuid::Uuid;

pub fn calculate_and_rank_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    spa_policy: &SpaScoringPolicy,
    qta_policy: &QtaWeightingPolicy,
    criteria: &[TieBreakCriterion],
    seed: u64,
) -> Vec<StandingsEntry> {
    let mut entries = calculate_standings(team_ids, fixtures);
    apply_spa_metrics_to_standings(&mut entries, spa_policy);
    apply_qta_to_standings(&mut entries, qta_policy);

    let initial_groups = partition_tied_groups(&entries, criteria);
    let has_h2h = criteria.contains(&TieBreakCriterion::HeadToHead);
    let has_random = criteria.contains(&TieBreakCriterion::Random);

    let mut ranked = Vec::with_capacity(entries.len());

    for group in initial_groups {
        if group.len() <= 1 {
            ranked.extend(group);
            continue;
        }

        if has_h2h {
            let after_h2h = resolve_head_to_head(&group, fixtures, spa_policy, qta_policy, criteria);
            let h2h_groups = partition_tied_groups(&after_h2h, criteria);
            for sub_group in h2h_groups {
                if sub_group.len() > 1 && has_random {
                    let after_random = resolve_random(&sub_group, seed);
                    ranked.extend(after_random);
                } else {
                    ranked.extend(sub_group);
                }
            }
        } else if has_random {
            let after_random = resolve_random(&group, seed);
            ranked.extend(after_random);
        } else {
            ranked.extend(group);
        }
    }

    ranked
}