use crate::dto::r#match::{
    ManagerPlayCallCategoryDto, ManagerSubstitutionReasonDto, MatchManagerSummaryDto,
    MatchPlayCallOutcomeSummaryDto, TeamManagerSummaryDto,
};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_manager_attribute_definitions;
use arlo_persistence::models::MatchRow;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_match_manager_report(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchManagerSummaryDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    build_manager_report(pool, &match_row).await
}

pub async fn build_manager_report(
    pool: &SqlitePool,
    match_row: &MatchRow,
) -> ControllerResult<MatchManagerSummaryDto> {
    let match_uuid = Uuid::parse_str(&match_row.id)?;
    let home_team_uuid = Uuid::parse_str(&match_row.home_team_id)?;
    let away_team_uuid = Uuid::parse_str(&match_row.away_team_id)?;

    let usage_rows =
        arlo_persistence::repositories::match_lineup_usage::list_by_match_id(pool, match_uuid)
            .await?;
    let home_usage = usage_rows
        .iter()
        .find(|u| u.team_id == match_row.home_team_id);
    let away_usage = usage_rows
        .iter()
        .find(|u| u.team_id == match_row.away_team_id);

    let manager_defs = get_or_load_manager_attribute_definitions(pool).await?;

    let home_managers =
        arlo_db::repositories::manager::list_by_team_id(pool, home_team_uuid, &manager_defs)
            .await
            .unwrap_or_default();
    let home_manager_name = home_managers
        .into_iter()
        .find(|m| Some(m.id().to_string()) == home_usage.map(|u| u.manager_id.clone()))
        .map(|m| m.person().name().to_string());

    let away_managers =
        arlo_db::repositories::manager::list_by_team_id(pool, away_team_uuid, &manager_defs)
            .await
            .unwrap_or_default();
    let away_manager_name = away_managers
        .into_iter()
        .find(|m| Some(m.id().to_string()) == away_usage.map(|u| u.manager_id.clone()))
        .map(|m| m.person().name().to_string());

    let decisions =
        arlo_persistence::repositories::match_manager_decisions::list_by_match_id(
            pool, match_uuid,
        )
        .await?;

    let subs_by_reason =
        arlo_persistence::repositories::match_manager_decisions::list_substitutions_by_reason_by_match_id(
            pool, match_uuid,
        )
        .await?;

    let play_calls_by_category =
        arlo_persistence::repositories::match_manager_decisions::list_play_calls_by_category_by_match_id(
            pool, match_uuid,
        )
        .await?;

    let outcomes =
        arlo_persistence::repositories::match_play_call_outcomes::list_by_match_id(
            pool, match_uuid,
        )
        .await?;

    let home_calls =
        arlo_tactics::play_call::list_by_team_id(pool, home_team_uuid).await.unwrap_or_default();
    let away_calls =
        arlo_tactics::play_call::list_by_team_id(pool, away_team_uuid).await.unwrap_or_default();

    let home_call_map: HashMap<Uuid, String> =
        home_calls.into_iter().map(|c| (c.id(), c.name().to_string())).collect();
    let away_call_map: HashMap<Uuid, String> =
        away_calls.into_iter().map(|c| (c.id(), c.name().to_string())).collect();

    let home_manager = build_team_manager_summary(
        &match_row.home_team_id,
        home_usage.map(|u| u.manager_id.clone()),
        home_manager_name,
        &decisions,
        &subs_by_reason,
        &play_calls_by_category,
        &outcomes,
        &home_call_map,
    );

    let away_manager = build_team_manager_summary(
        &match_row.away_team_id,
        away_usage.map(|u| u.manager_id.clone()),
        away_manager_name,
        &decisions,
        &subs_by_reason,
        &play_calls_by_category,
        &outcomes,
        &away_call_map,
    );

    Ok(MatchManagerSummaryDto {
        home_manager,
        away_manager,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_team_manager_summary(
    team_id: &str,
    manager_id: Option<String>,
    manager_name: Option<String>,
    decisions: &[arlo_persistence::models::MatchManagerDecisionRow],
    subs_by_reason: &[arlo_persistence::models::MatchManagerSubstitutionByReasonRow],
    play_calls_by_category: &[arlo_persistence::models::MatchManagerPlayCallByCategoryRow],
    outcomes: &[arlo_persistence::models::MatchPlayCallOutcomeRow],
    call_map: &HashMap<Uuid, String>,
) -> Option<TeamManagerSummaryDto> {
    let dec = decisions.iter().find(|d| d.team_id == team_id);

    let substitutions_by_reason: Vec<ManagerSubstitutionReasonDto> = subs_by_reason
        .iter()
        .filter(|s| s.team_id == team_id)
        .map(|s| ManagerSubstitutionReasonDto {
            reason: s.reason.clone(),
            count: s.substitutions_count as u32,
        })
        .collect();

    let play_calls_by_category: Vec<ManagerPlayCallCategoryDto> = play_calls_by_category
        .iter()
        .filter(|c| c.team_id == team_id)
        .map(|c| ManagerPlayCallCategoryDto {
            category: c.category.clone(),
            count: c.play_calls_count as u32,
        })
        .collect();

    let play_call_outcomes: Vec<MatchPlayCallOutcomeSummaryDto> = outcomes
        .iter()
        .filter_map(|o| {
            let pid = Uuid::parse_str(&o.play_call_id).ok()?;
            let name = call_map.get(&pid)?;
            let attempts = o.attempts as u32;
            let successes = o.successes as u32;
            let success_rate = if attempts > 0 {
                successes as f64 / attempts as f64
            } else {
                0.0
            };
            Some(MatchPlayCallOutcomeSummaryDto {
                play_call_id: o.play_call_id.clone(),
                play_call_name: name.clone(),
                attempts,
                successes,
                success_rate,
            })
        })
        .collect();

    match dec {
        Some(d) => Some(TeamManagerSummaryDto {
            team_id: team_id.to_string(),
            manager_id,
            manager_name,
            substitutions_made: d.substitutions_made as u32,
            time_calls_used: d.time_calls_used as u32,
            challenges_won: d.challenges_won as u32,
            challenges_lost: d.challenges_lost as u32,
            tactical_profile_switches: d.tactical_profile_switches as u32,
            substitutions_by_reason,
            play_calls_by_category,
            play_call_outcomes,
        }),
        None => {
            if !substitutions_by_reason.is_empty()
                || !play_calls_by_category.is_empty()
                || !play_call_outcomes.is_empty()
            {
                Some(TeamManagerSummaryDto {
                    team_id: team_id.to_string(),
                    manager_id,
                    manager_name,
                    substitutions_made: 0,
                    time_calls_used: 0,
                    challenges_won: 0,
                    challenges_lost: 0,
                    tactical_profile_switches: 0,
                    substitutions_by_reason,
                    play_calls_by_category,
                    play_call_outcomes,
                })
            } else {
                None
            }
        }
    }
}