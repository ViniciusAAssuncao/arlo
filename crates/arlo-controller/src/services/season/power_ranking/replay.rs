use super::result_loader::load_results;
use crate::domain::calendar::CalendarDate;
use crate::error::ControllerResult;
use arlo_analytics::{
    replay_power_ranking, PowerRankingConfig, PowerRankingSnapshot, TeamPowerSeed,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn replay_season_power_ranking(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    through: CalendarDate,
    seeds: &[TeamPowerSeed],
    config: &PowerRankingConfig,
) -> ControllerResult<PowerRankingSnapshot> {
    let results = load_results(pool, season_instance_id, through).await?;
    Ok(replay_power_ranking(seeds, &results, config)?)
}
