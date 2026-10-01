use crate::domain::season::Fixture;
use crate::error::ControllerResult;
use crate::services::season::conflict::ConflictScanReport;
use crate::services::season::persistence::conflict_scan_row_mapper::map_conflict_scan_to_rows;
use crate::services::season::persistence::generated_season_row_mapper::map_generated_season_to_rows;
use crate::services::season::persistence::generated_stage_schedule_row_mapper::map_generated_stage_schedule_to_rows;
use crate::services::season::season_generator::GeneratedSeason;
use crate::services::season::stage::stage_schedule_generator::GeneratedStageSchedule;
use arlo_persistence::persister::SeasonPersister;
use sqlx::SqlitePool;

pub async fn persist_generated_season(
    pool: &SqlitePool,
    generated_season: &GeneratedSeason,
) -> ControllerResult<()> {
    let (season_row, stage_row, fixture_rows) = map_generated_season_to_rows(generated_season);

    SeasonPersister::persist_season_schedule(pool, &season_row, &[stage_row], &fixture_rows)
        .await?;

    Ok(())
}

pub async fn persist_generated_stage_schedule(
    pool: &SqlitePool,
    generated_stage: &GeneratedStageSchedule,
) -> ControllerResult<()> {
    let (stage_row, fixture_rows, tie_rows) = map_generated_stage_schedule_to_rows(generated_stage);

    SeasonPersister::persist_stage_schedule(pool, &stage_row, &fixture_rows, &tie_rows).await?;

    Ok(())
}

pub async fn persist_conflict_scan_result(
    pool: &SqlitePool,
    report: &ConflictScanReport,
    fixtures: &[Fixture],
) -> ControllerResult<()> {
    if report.postponements_applied() == 0 {
        return Ok(());
    }

    let (fixture_rows, postponement_rows) = map_conflict_scan_to_rows(report, fixtures);

    SeasonPersister::persist_postponements(pool, &fixture_rows, &postponement_rows).await?;

    Ok(())
}
