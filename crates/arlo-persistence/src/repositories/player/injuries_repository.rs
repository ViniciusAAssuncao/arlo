use crate::error::PersistenceResult;
use crate::models::{MatchPlayerInjuryByBodyRegionRow, MatchPlayerInjuryRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const INJURY_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "total_injuries",
    "contact_injuries",
    "non_contact_injuries",
    "grade_1_injuries",
    "grade_2_injuries",
    "grade_3_injuries",
];

const INJURY_BY_BODY_REGION_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "body_region",
    "injuries_count",
];

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
    execute_batch_insert(
        tx,
        "match_player_injuries",
        INJURY_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(row.total_injuries);
            b.push_bind(row.contact_injuries);
            b.push_bind(row.non_contact_injuries);
            b.push_bind(row.grade_1_injuries);
            b.push_bind(row.grade_2_injuries);
            b.push_bind(row.grade_3_injuries);
        },
    )
    .await
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
    execute_batch_insert(
        tx,
        "match_player_injuries_by_body_region",
        INJURY_BY_BODY_REGION_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(&row.body_region);
            b.push_bind(row.injuries_count);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerInjuryRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerInjuryRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_injuries,
            contact_injuries,
            non_contact_injuries,
            grade_1_injuries,
            grade_2_injuries,
            grade_3_injuries
        FROM match_player_injuries
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerInjuryRow>> {
    let row = sqlx::query_as::<_, MatchPlayerInjuryRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_injuries,
            contact_injuries,
            non_contact_injuries,
            grade_1_injuries,
            grade_2_injuries,
            grade_3_injuries
        FROM match_player_injuries
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_body_region_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerInjuryByBodyRegionRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerInjuryByBodyRegionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            body_region,
            injuries_count
        FROM match_player_injuries_by_body_region
        WHERE match_id = ?
        ORDER BY player_id ASC, body_region ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_body_region_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerInjuryByBodyRegionRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerInjuryByBodyRegionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            body_region,
            injuries_count
        FROM match_player_injuries_by_body_region
        WHERE match_id = ? AND player_id = ?
        ORDER BY body_region ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
