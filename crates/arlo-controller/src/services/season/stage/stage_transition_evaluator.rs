use crate::domain::season::{BracketSeed, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::stage::qualification_pool_evaluator::evaluate_pool;
use crate::services::season::standings::group_rank_annotator::annotate_group_ranks;
use arlo_domain::{CompetitionGroup, StageEntryRule};
use std::collections::HashMap;
use uuid::Uuid;

pub fn evaluate_stage_transition(
    standings: &[StandingsEntry],
    entry_rule: &StageEntryRule,
    groups: &[CompetitionGroup],
    external_winners: &HashMap<Uuid, Uuid>,
) -> ControllerResult<Vec<BracketSeed>> {
    if standings.is_empty() {
        return Err(ControllerError::Validation(
            "Standings must not be empty to evaluate stage transition".to_string(),
        ));
    }

    let annotated = annotate_group_ranks(standings, groups);
    let mut qualified_teams: Vec<Uuid> = Vec::new();

    for pool in entry_rule.pools() {
        let pool_teams = evaluate_pool(pool, &annotated, standings, external_winners)?;
        qualified_teams.extend(pool_teams);
    }

    let seeds = qualified_teams
        .iter()
        .enumerate()
        .map(|(idx, &team_id)| BracketSeed::new((idx + 1) as u32, team_id))
        .collect();

    Ok(seeds)
}
