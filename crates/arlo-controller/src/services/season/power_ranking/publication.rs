use super::{load_preseason_seeds, replay_season_power_ranking};
use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::date_advancer;
use arlo_analytics::{PowerRankingConfig, POWER_RANKING_MODEL_VERSION};
use arlo_persistence::models::season::PowerRankingSnapshotRow;
use arlo_persistence::repositories::season::power_rankings;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn maybe_publish_power_rankings(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    current_date: CalendarDate,
) -> ControllerResult<()> {
    let config = PowerRankingConfig::default();
    let candidates = power_rankings::list_publication_candidates(
        pool,
        current_date.year(),
        current_date.day_of_year(),
        POWER_RANKING_MODEL_VERSION,
    )
    .await?;
    for (season_id, status) in candidates {
        let season_id = Uuid::parse_str(&season_id)?;
        let latest =
            power_rankings::get_latest(pool, season_id, POWER_RANKING_MODEL_VERSION).await?;
        if !publication_due(calendar, current_date, &status, latest.as_ref())? {
            continue;
        }
        let seeds = load_preseason_seeds(pool, season_id, current_date, &config).await?;
        let snapshot =
            replay_season_power_ranking(pool, season_id, current_date, &seeds, &config).await?;
        power_rankings::publish(
            pool,
            season_id,
            current_date.year(),
            current_date.day_of_year(),
            &snapshot,
            &seeds,
        )
        .await?;
    }
    Ok(())
}

fn publication_due(
    calendar: &CalendarSystem,
    current_date: CalendarDate,
    status: &str,
    latest: Option<&PowerRankingSnapshotRow>,
) -> ControllerResult<bool> {
    let Some(latest) = latest else {
        return Ok(true);
    };
    let day = u32::try_from(latest.day_of_year).map_err(|_| {
        ControllerError::InvalidData(format!("snapshot {} has an invalid day", latest.id))
    })?;
    let last_date = CalendarDate::new(latest.year, day);
    if last_date >= current_date {
        return Ok(false);
    }
    if status == "Completed" {
        return Ok(true);
    }
    let interval = calendar.week_days().len() as i64;
    Ok(date_advancer::advance(calendar, &last_date, interval) <= current_date)
}
