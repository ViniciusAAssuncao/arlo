use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerFoulSeasonStatsRow {
    pub fouls_committed: i64,
    pub fouls_drawn: i64,
    pub correct_calls_committed: i64,
    pub incorrect_calls_committed: i64,
    pub expulsions: i64,
    pub time_penalties: i64,
}

pub async fn get_player_foul_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerFoulSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerFoulSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(foul.fouls_committed), 0) as fouls_committed,
            COALESCE(SUM(foul.fouls_drawn), 0) as fouls_drawn,
            COALESCE(SUM(foul.correct_calls_committed), 0) as correct_calls_committed,
            COALESCE(SUM(foul.incorrect_calls_committed), 0) as incorrect_calls_committed,
            (
                SELECT COALESCE(SUM(p.expulsion_count), 0)
                FROM match_player_punishments p
                JOIN matches m2 ON p.match_id = m2.id
                JOIN fixtures f2 ON m2.fixture_id = f2.id
                JOIN season_stages ss2 ON f2.season_stage_id = ss2.id
                WHERE p.player_id = ? AND ss2.season_instance_id = ?
            ) as expulsions,
            (
                SELECT COALESCE(SUM(p.time_penalty_count), 0)
                FROM match_player_punishments p
                JOIN matches m3 ON p.match_id = m3.id
                JOIN fixtures f3 ON m3.fixture_id = f3.id
                JOIN season_stages ss3 ON f3.season_stage_id = ss3.id
                WHERE p.player_id = ? AND ss3.season_instance_id = ?
            ) as time_penalties
        FROM match_player_fouls foul
        JOIN matches m ON foul.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE foul.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
