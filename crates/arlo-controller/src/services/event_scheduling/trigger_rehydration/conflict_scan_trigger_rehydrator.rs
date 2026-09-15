use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::domain::season::FixtureStatus;
use crate::error::ControllerResult;
use crate::services::season::persistence::map_row_to_fixture;
use arlo_domain::LeagueCalendarConfig;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn rehydrate_conflict_scan_triggers(
    pool: &SqlitePool,
    configs: &[Arc<LeagueCalendarConfig>],
) -> ControllerResult<Vec<PendingTrigger>> {
    let mut triggers = Vec::new();

    for config in configs {
        let competition_id = config.league_id();
        let season_instances =
            arlo_persistence::repositories::season::season_instances::list_by_competition_id(
                pool,
                competition_id,
            )
            .await?;

        for season in season_instances {
            if season.status != "Active" {
                continue;
            }

            let season_instance_id = Uuid::parse_str(&season.id)?;
            let stages =
                arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
                    pool,
                    season_instance_id,
                )
                .await?;

            let mut unplayed_dates = Vec::new();

            for stage in stages {
                let stage_id = Uuid::parse_str(&stage.id)?;
                let fixture_rows =
                    arlo_persistence::repositories::season::fixtures::list_by_stage_id(
                        pool, stage_id,
                    )
                    .await?;

                for row in &fixture_rows {
                    let fixture = map_row_to_fixture(row)?;
                    if matches!(
                        fixture.status(),
                        FixtureStatus::Scheduled | FixtureStatus::Postponed
                    ) {
                        unplayed_dates.push(fixture.scheduled_date());
                    }
                }
            }

            if let Some(next_scan_date) = unplayed_dates.into_iter().min() {
                triggers.push(PendingTrigger::new(
                    next_scan_date,
                    competition_id,
                    TriggerKind::ConflictScanDue,
                ));
            }
        }
    }

    Ok(triggers)
}