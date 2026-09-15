use crate::error::PersistenceResult;
use crate::models::season::PromotionRelegationResultRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &PromotionRelegationResultRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO season_promotion_relegation_results (
            id,
            season_instance_id,
            team_id,
            movement_kind,
            source_league_id,
            destination_league_id,
            final_standing_position,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.season_instance_id)
    .bind(&row.team_id)
    .bind(&row.movement_kind)
    .bind(&row.source_league_id)
    .bind(&row.destination_league_id)
    .bind(row.final_standing_position)
    .bind(row.created_at_unix_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[PromotionRelegationResultRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn list_by_season_instance_id(
    pool: &SqlitePool,
    season_instance_id: Uuid,
) -> PersistenceResult<Vec<PromotionRelegationResultRow>> {
    let rows = sqlx::query_as::<_, PromotionRelegationResultRow>(
        "SELECT id, season_instance_id, team_id, movement_kind, source_league_id, destination_league_id, final_standing_position, created_at_unix_seconds FROM season_promotion_relegation_results WHERE season_instance_id = ? ORDER BY created_at_unix_seconds ASC",
    )
    .bind(season_instance_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
