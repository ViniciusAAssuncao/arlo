use crate::dto::r#match::{
    FoulOriginSummaryDto, MatchOfficiatingDto, RefereePerformanceSummaryDto,
};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_referee_attribute_definitions;
use arlo_persistence::models::incidents::MatchFoulRow;
use arlo_persistence::models::MatchRow;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_match_officiating(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchOfficiatingDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let fouls =
        arlo_persistence::repositories::match_fouls::list_by_match_id(pool, match_id).await?;

    build_officiating_summary(pool, &match_row, &fouls).await
}

pub async fn build_officiating_summary(
    pool: &SqlitePool,
    match_row: &MatchRow,
    fouls: &[MatchFoulRow],
) -> ControllerResult<MatchOfficiatingDto> {
    let match_uuid = Uuid::parse_str(&match_row.id)?;
    let perf_rows = arlo_persistence::repositories::match_referee_performance::list_by_match_id(
        pool, match_uuid,
    )
    .await?;

    let referee_defs = get_or_load_referee_attribute_definitions(pool).await?;

    let head_row = perf_rows.iter().find(|r| r.role == "Head");
    let peace_row = perf_rows.iter().find(|r| r.role == "Peace");

    let head_referee = if let Some(row) = head_row {
        let ref_uuid = Uuid::parse_str(&row.referee_id).unwrap_or_default();
        let referee = arlo_db::repositories::referee::get_by_id(pool, ref_uuid, &referee_defs)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        let name = referee
            .map(|r| r.person().name().to_string())
            .unwrap_or_else(|| "Árbitro Principal".to_string());
        let accuracy_rate = if row.calls_made > 0 {
            row.calls_correct as f64 / row.calls_made as f64
        } else {
            1.0
        };
        Some(RefereePerformanceSummaryDto {
            referee_id: row.referee_id.clone(),
            referee_name: name,
            role: row.role.clone(),
            calls_made: row.calls_made,
            calls_correct: row.calls_correct,
            calls_incorrect: row.calls_incorrect,
            accuracy_rate,
            peace_referee_interventions: row.peace_referee_interventions,
        })
    } else {
        None
    };

    let peace_referee = if let Some(row) = peace_row {
        let ref_uuid = Uuid::parse_str(&row.referee_id).unwrap_or_default();
        let referee = arlo_db::repositories::referee::get_by_id(pool, ref_uuid, &referee_defs)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        let name = referee
            .map(|r| r.person().name().to_string())
            .unwrap_or_else(|| "Árbitro de Paz".to_string());
        Some(RefereePerformanceSummaryDto {
            referee_id: row.referee_id.clone(),
            referee_name: name,
            role: row.role.clone(),
            calls_made: row.calls_made,
            calls_correct: row.calls_correct,
            calls_incorrect: row.calls_incorrect,
            accuracy_rate: 1.0,
            peace_referee_interventions: row.peace_referee_interventions,
        })
    } else {
        None
    };

    let mut origin_counts: HashMap<String, (u32, u32, u32)> = HashMap::new();
    for f in fouls {
        let entry = origin_counts.entry(f.origin.clone()).or_insert((0, 0, 0));
        entry.0 += 1;
        if f.original_call_correct {
            entry.1 += 1;
        } else {
            entry.2 += 1;
        }
    }

    let mut fouls_by_origin: Vec<FoulOriginSummaryDto> = origin_counts
        .into_iter()
        .map(
            |(origin, (count, correct_count, incorrect_count))| FoulOriginSummaryDto {
                origin,
                count,
                correct_count,
                incorrect_count,
            },
        )
        .collect();

    fouls_by_origin.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(MatchOfficiatingDto {
        head_referee,
        peace_referee,
        fouls_by_origin,
    })
}
