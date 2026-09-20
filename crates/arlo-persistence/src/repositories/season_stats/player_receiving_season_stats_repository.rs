use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, FromRow)]
pub struct PlayerReceivingSeasonStatsRow {
    pub targets: i64,
    pub receptions: i64,
    pub drops: i64,
    pub receiving_mirins: f64,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
}

pub async fn get_player_receiving_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerReceivingSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerReceivingSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(r.targets), 0) as targets,
            COALESCE(SUM(r.receptions), 0) as receptions,
            COALESCE(SUM(r.drops), 0) as drops,
            COALESCE(SUM(r.receiving_mirins), 0.0) as receiving_mirins,
            COALESCE(SUM(r.run_after_catch_mirins), 0.0) as run_after_catch_mirins,
            COALESCE(MAX(r.longest_reception_mirim), 0.0) as longest_reception_mirim
        FROM match_player_receiving r
        JOIN matches m ON r.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE r.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
