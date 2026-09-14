use crate::domain::season::{BracketSeed, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::stage::qualification_pool_evaluator::evaluate_pool;
use crate::services::season::standings::group_rank_annotator::annotate_group_ranks;
use arlo_domain::{CompetitionGroup, StageEntryRule};

pub fn evaluate_stage_transition(
    standings: &[StandingsEntry],
    entry_rule: &StageEntryRule,
    groups: &[CompetitionGroup],
) -> ControllerResult<Vec<BracketSeed>> {
    if standings.is_empty() {
        return Err(ControllerError::Validation(
            "Standings must not be empty to evaluate stage transition".to_string(),
        ));
    }

    let annotated = annotate_group_ranks(standings, groups);
    let mut qualified_teams: Vec<StandingsEntry> = Vec::new();

    for pool in entry_rule.pools() {
        let pool_teams = evaluate_pool(pool, &annotated, standings)?;
        qualified_teams.extend(pool_teams);
    }

    let seeds = qualified_teams
        .iter()
        .enumerate()
        .map(|(idx, entry)| BracketSeed::new((idx + 1) as u32, entry.team_id()))
        .collect();

    Ok(seeds)
}