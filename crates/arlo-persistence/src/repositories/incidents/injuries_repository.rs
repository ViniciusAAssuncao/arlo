use crate::error::PersistenceResult;
use crate::models::MatchInjuryRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "player_id",
    "team_id",
    "mechanism",
    "body_region",
    "severity_grade",
    "injury_definition_id",
    "trigger_probability",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchInjuryRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_injuries (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            team_id,
            mechanism,
            body_region,
            severity_grade,
            injury_definition_id,
            trigger_probability
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.player_id)
    .bind(&row.team_id)
    .bind(&row.mechanism)
    .bind(&row.body_region)
    .bind(&row.severity_grade)
    .bind(&row.injury_definition_id)
    .bind(row.trigger_probability)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchInjuryRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_injuries", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(&row.player_id);
        b.push_bind(&row.team_id);
        b.push_bind(&row.mechanism);
        b.push_bind(&row.body_region);
        b.push_bind(&row.severity_grade);
        b.push_bind(&row.injury_definition_id);
        b.push_bind(row.trigger_probability);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchInjuryRow>> {
    let rows = sqlx::query_as::<_, MatchInjuryRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            team_id,
            mechanism,
            body_region,
            severity_grade,
            injury_definition_id,
            trigger_probability
        FROM match_injuries
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}