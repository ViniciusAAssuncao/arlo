use crate::error::PersistenceResult;
use crate::models::{MatchPlayerInjuryByBodyRegionRow, MatchPlayerInjuryRow};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerInjuryRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_injuries (
            id,
            match_id,
            player_id,
            total_injuries,
            contact_injuries,
            non_contact_injuries,
            grade_1_injuries,
            grade_2_injuries,
            grade_3_injuries
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_injuries)
    .bind(row.contact_injuries)
    .bind(row.non_contact_injuries)
    .bind(row.grade_1_injuries)
    .bind(row.grade_2_injuries)
    .bind(row.grade_3_injuries)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerInjuryRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_by_body_region(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerInjuryByBodyRegionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_injuries_by_body_region (
            id,
            match_id,
            player_id,
            body_region,
            injuries_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.body_region)
    .bind(row.injuries_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_body_region_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerInjuryByBodyRegionRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_by_body_region(tx, row).await?;
    }
    Ok(())
}
