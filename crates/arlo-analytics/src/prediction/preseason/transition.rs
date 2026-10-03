use super::plan::PreparedPlan;
use crate::error::{AnalyticsError, AnalyticsResult};
use arlo_domain::QualificationPoolRule;
use std::collections::HashSet;

pub(super) fn next_participants(
    plan: &PreparedPlan,
    next_stage: usize,
    ordered: &[usize],
) -> AnalyticsResult<Vec<usize>> {
    let rule = plan.plan.stages[next_stage].definition.entry_rule();
    let mut selected = Vec::new();
    for pool in rule.pools() {
        if matches!(pool, QualificationPoolRule::TopN { count } | QualificationPoolRule::BottomN { count } if *count as usize > ordered.len())
        {
            return Err(AnalyticsError::InvalidData(
                "not enough teams for projected qualification".into(),
            ));
        }
        let mut teams = match pool {
            QualificationPoolRule::AllTeams => ordered.to_vec(),
            QualificationPoolRule::TopN { count } => {
                ordered.iter().take(*count as usize).copied().collect()
            }
            QualificationPoolRule::BottomN { count } => ordered
                .iter()
                .rev()
                .take(*count as usize)
                .copied()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect(),
            QualificationPoolRule::GroupWinners => group_position(plan, ordered, 0),
            QualificationPoolRule::GroupRunnersUp => group_position(plan, ordered, 1),
            QualificationPoolRule::BestAtGroupPosition {
                position_index,
                count,
            } => group_position(plan, ordered, *position_index as usize)
                .into_iter()
                .take(*count as usize)
                .collect(),
            QualificationPoolRule::PositionRange {
                start_position,
                end_position,
            } => {
                if *start_position == 0
                    || end_position < start_position
                    || *end_position as usize > ordered.len()
                {
                    return Err(AnalyticsError::InvalidData(
                        "invalid projected position range".into(),
                    ));
                }
                ordered
                    .iter()
                    .skip((*start_position - 1) as usize)
                    .take((end_position - start_position + 1) as usize)
                    .copied()
                    .collect()
            }
            QualificationPoolRule::ExternalCompetitionWinner { competition_id } => {
                let id = plan
                    .plan
                    .external_winners
                    .get(competition_id)
                    .ok_or_else(|| {
                        AnalyticsError::InvalidData(
                            "external competition winner unavailable".into(),
                        )
                    })?;
                vec![*plan.index.get(id).ok_or_else(|| {
                    AnalyticsError::InvalidData("external winner has no projection rating".into())
                })?]
            }
        };
        selected.append(&mut teams);
    }
    let mut seen = HashSet::new();
    if selected.len() < 2 || selected.iter().any(|team| !seen.insert(*team)) {
        return Err(AnalyticsError::InvalidData(
            "invalid projected stage entrants".into(),
        ));
    }
    Ok(selected)
}

fn group_position(plan: &PreparedPlan, ordered: &[usize], position: usize) -> Vec<usize> {
    let mut eligible = Vec::new();
    for group in &plan.plan.groups {
        let mut count = 0;
        for &team in ordered {
            if group.team_ids().contains(&plan.ids[team]) {
                if count == position {
                    eligible.push(team);
                    break;
                }
                count += 1;
            }
        }
    }
    eligible.sort_by_key(|team| {
        ordered
            .iter()
            .position(|entry| entry == team)
            .unwrap_or(usize::MAX)
    });
    eligible
}
