use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerKickFoulSeasonTotalsRow {
    pub kick_foul_takes: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, FromRow)]
pub struct PlayerKickFoulByDecisionSeasonRow {
    pub decision_kind: String,
    pub takes_count: i64,
}

pub async fn get_player_kick_foul_totals(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerKickFoulSeasonTotalsRow> {
    let row = sqlx::query_as::<_, PlayerKickFoulSeasonTotalsRow>(
        r#"SELECT
            COALESCE(SUM(kf.kick_foul_takes), 0) as kick_foul_takes
        FROM match_player_kick_fouls kf
        JOIN matches m ON kf.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE kf.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn list_player_kick_fouls_by_decision(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<Vec<PlayerKickFoulByDecisionSeasonRow>> {
    let rows = sqlx::query_as::<_, PlayerKickFoulByDecisionSeasonRow>(
        r#"SELECT
            kfd.decision_kind,
            COALESCE(SUM(kfd.takes_count), 0) as takes_count
        FROM match_player_kick_fouls_by_decision kfd
        JOIN matches m ON kfd.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE kfd.player_id = ? AND ss.season_instance_id = ?
        GROUP BY kfd.decision_kind
        ORDER BY kfd.decision_kind ASC"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
