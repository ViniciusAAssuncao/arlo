use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerDuelSeasonTotalsRow {
    pub total_duels: i64,
    pub total_wins: i64,
    pub total_losses: i64,
    pub attacker_duels: i64,
    pub attacker_wins: i64,
    pub attacker_losses: i64,
    pub defender_duels: i64,
    pub defender_wins: i64,
    pub defender_losses: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, FromRow)]
pub struct PlayerDuelByKindSeasonRow {
    pub duel_kind: String,
    pub total: i64,
    pub wins: i64,
    pub losses: i64,
    pub as_attacker_wins: i64,
    pub as_attacker_losses: i64,
    pub as_defender_wins: i64,
    pub as_defender_losses: i64,
}

pub async fn get_player_duel_totals(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerDuelSeasonTotalsRow> {
    let row = sqlx::query_as::<_, PlayerDuelSeasonTotalsRow>(
        r#"SELECT
            COALESCE(SUM(d.total_duels), 0) as total_duels,
            COALESCE(SUM(d.total_wins), 0) as total_wins,
            COALESCE(SUM(d.total_losses), 0) as total_losses,
            COALESCE(SUM(d.attacker_duels), 0) as attacker_duels,
            COALESCE(SUM(d.attacker_wins), 0) as attacker_wins,
            COALESCE(SUM(d.attacker_losses), 0) as attacker_losses,
            COALESCE(SUM(d.defender_duels), 0) as defender_duels,
            COALESCE(SUM(d.defender_wins), 0) as defender_wins,
            COALESCE(SUM(d.defender_losses), 0) as defender_losses
        FROM match_player_duels d
        JOIN matches m ON d.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE d.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn list_player_duels_by_kind(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<Vec<PlayerDuelByKindSeasonRow>> {
    let rows = sqlx::query_as::<_, PlayerDuelByKindSeasonRow>(
        r#"SELECT
            k.duel_kind,
            COALESCE(SUM(k.total), 0) as total,
            COALESCE(SUM(k.wins), 0) as wins,
            COALESCE(SUM(k.losses), 0) as losses,
            COALESCE(SUM(k.as_attacker_wins), 0) as as_attacker_wins,
            COALESCE(SUM(k.as_attacker_losses), 0) as as_attacker_losses,
            COALESCE(SUM(k.as_defender_wins), 0) as as_defender_wins,
            COALESCE(SUM(k.as_defender_losses), 0) as as_defender_losses
        FROM match_player_duels_by_kind k
        JOIN matches m ON k.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE k.player_id = ? AND ss.season_instance_id = ?
        GROUP BY k.duel_kind
        ORDER BY k.duel_kind ASC"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
