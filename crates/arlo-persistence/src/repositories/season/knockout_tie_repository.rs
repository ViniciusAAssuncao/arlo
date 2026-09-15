use crate::error::PersistenceResult;
use crate::models::season::KnockoutTieRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &KnockoutTieRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO knockout_ties (
            id,
            season_stage_id,
            round_index,
            tie_index,
            high_seed_team_id,
            high_seed_number,
            low_seed_team_id,
            low_seed_number,
            leg_one_fixture_id,
            leg_two_fixture_id,
            aggregate_winner_team_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.season_stage_id)
    .bind(row.round_index)
    .bind(row.tie_index)
    .bind(&row.high_seed_team_id)
    .bind(row.high_seed_number)
    .bind(&row.low_seed_team_id)
    .bind(row.low_seed_number)
    .bind(&row.leg_one_fixture_id)
    .bind(&row.leg_two_fixture_id)
    .bind(&row.aggregate_winner_team_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[KnockoutTieRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn update_winner(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
    aggregate_winner_team_id: Option<Uuid>,
    leg_two_fixture_id: Option<Uuid>,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"UPDATE knockout_ties SET
            aggregate_winner_team_id = ?,
            leg_two_fixture_id = ?
        WHERE id = ?"#,
    )
    .bind(aggregate_winner_team_id.map(|id| id.to_string()))
    .bind(leg_two_fixture_id.map(|id| id.to_string()))
    .bind(id.to_string())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_by_id(
    pool: &SqlitePool,
    id: Uuid,
) -> PersistenceResult<Option<KnockoutTieRow>> {
    let row = sqlx::query_as::<_, KnockoutTieRow>(
        "SELECT id, season_stage_id, round_index, tie_index, high_seed_team_id, high_seed_number, low_seed_team_id, low_seed_number, leg_one_fixture_id, leg_two_fixture_id, aggregate_winner_team_id FROM knockout_ties WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_season_stage_id(
    pool: &SqlitePool,
    stage_id: Uuid,
) -> PersistenceResult<Vec<KnockoutTieRow>> {
    let rows = sqlx::query_as::<_, KnockoutTieRow>(
        "SELECT id, season_stage_id, round_index, tie_index, high_seed_team_id, high_seed_number, low_seed_team_id, low_seed_number, leg_one_fixture_id, leg_two_fixture_id, aggregate_winner_team_id FROM knockout_ties WHERE season_stage_id = ? ORDER BY round_index, tie_index ASC",
    )
    .bind(stage_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}