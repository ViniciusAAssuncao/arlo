use crate::domain::calendar::CalendarSystem;
use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::error::ControllerResult;
use crate::services::event_scheduling::stage_completion_date_calculator::calculate_stage_completion_date;
use crate::services::season::persistence::map_row_to_fixture;
use arlo_domain::LeagueCalendarConfig;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn rehydrate_stage_transition_triggers(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
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
            let current_stage_order_index = season.current_stage_order_index;

            let stages =
                arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
                    pool,
                    season_instance_id,
                )
                .await?;

            let current_stage = match stages
                .into_iter()
                .find(|s| s.stage_order_index == current_stage_order_index)
            {
                Some(s) => s,
                None => continue,
            };

            let stage_id = Uuid::parse_str(&current_stage.id)?;
            let fixture_rows =
                arlo_persistence::repositories::season::fixtures::list_by_stage_id(
                    pool, stage_id,
                )
                .await?;

            let mut fixtures = Vec::with_capacity(fixture_rows.len());
            for row in &fixture_rows {
                fixtures.push(map_row_to_fixture(row)?);
            }

            if let Some(completion_date) =
                calculate_stage_completion_date(calendar, &fixtures)
            {
                triggers.push(PendingTrigger::new(
                    completion_date,
                    competition_id,
                    TriggerKind::StageTransitionCheckDue,
                ));
            }
        }
    }

    Ok(triggers)
}