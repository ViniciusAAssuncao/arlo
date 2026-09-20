use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::collective_agreement::collective_agreement_catalog_cache::get_or_load_collective_agreement_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::calendar::resolve_collective_agreement_windows;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::conflict::postponement_resolver::resolve_conflicts_and_postpone;
use crate::services::season::persistence::{map_row_to_fixture, persist_conflict_scan_result};
pub use crate::services::season::conflict::ConflictScanReport;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_conflict_scan(
    pool: &SqlitePool,
    competition_id: Uuid,
    trigger_store: Arc<PendingTriggerStore>,
) -> ControllerResult<ConflictScanReport> {
    let config_arc = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let teams = arlo_db::repositories::team::list_by_league_id(pool, competition_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id()).collect();
    if team_ids.is_empty() {
        return Ok(ConflictScanReport::empty(competition_id));
    }

    let season_instances = arlo_persistence::repositories::season::season_instances::list_by_competition_id(
        pool,
        competition_id,
    )
    .await?;

    let active_season = match season_instances
        .iter()
        .find(|s| s.status == "Active" || s.status == "Pending")
        .or_else(|| season_instances.first())
    {
        Some(s) => s,
        None => return Ok(ConflictScanReport::empty(competition_id)),
    };

    let season_instance_id = Uuid::parse_str(&active_season.id)?;
    let reference_year = active_season.reference_year;

    let stages = arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
        pool,
        season_instance_id,
    )
    .await?;

    if stages.is_empty() {
        return Ok(ConflictScanReport::empty(competition_id));
    }

    let mut stage_ids = Vec::with_capacity(stages.len());
    let mut domain_fixtures = Vec::new();

    for stage in &stages {
        let stage_id = Uuid::parse_str(&stage.id)?;
        stage_ids.push(stage_id);

        let fixture_rows =
            arlo_persistence::repositories::season::fixtures::list_by_stage_id(pool, stage_id)
                .await?;

        for row in &fixture_rows {
            domain_fixtures.push(map_row_to_fixture(row)?);
        }
    }

    if domain_fixtures.is_empty() {
        return Ok(ConflictScanReport::empty(competition_id));
    }

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog
        .all()
        .next()
        .ok_or_else(|| ControllerError::NotFound("No calendar systems found".to_string()))?;

    let min_year = domain_fixtures
        .iter()
        .map(|f| f.scheduled_date().year())
        .min()
        .unwrap_or(reference_year);
    let max_year = domain_fixtures
        .iter()
        .map(|f| f.scheduled_date().year())
        .max()
        .unwrap_or(reference_year);
    let years = (min_year - 1)..=(max_year + 1);

    let ca_catalog = get_or_load_collective_agreement_catalog(pool).await?;
    let mut blackout_windows = Vec::new();
    for ca_id in config_arc.collective_agreement_ids() {
        if let Some(agreement) = ca_catalog.get(ca_id) {
            let windows = resolve_collective_agreement_windows(
                calendar,
                agreement,
                years.clone(),
            )?;
            blackout_windows.extend(windows);
        }
    }

    let max_search_days: u32 = 366;
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as u32
    } else {
        7
    };
    let max_search_weeks = (max_search_days + week_len - 1) / week_len;

    let report = resolve_conflicts_and_postpone(
        calendar,
        config_arc.timing(),
        reference_year,
        competition_id,
        &stage_ids,
        &config_arc.games_per_week(),
        &config_arc.rest_gap_policy(),
        &blackout_windows,
        &config_arc.postponement(),
        &mut domain_fixtures,
        &team_ids,
        max_search_weeks,
    )?;

    persist_conflict_scan_result(pool, &report, &domain_fixtures).await?;

    if report.postponements_applied() > 0 {
        if let Some(next_scan_date) = report
            .postponement_records()
            .iter()
            .map(|r| r.new_date())
            .min()
        {
            trigger_store
                .insert(PendingTrigger::new(
                    next_scan_date,
                    competition_id,
                    TriggerKind::ConflictScanDue,
                ))
                .await;
        }
    }

    Ok(report)
}