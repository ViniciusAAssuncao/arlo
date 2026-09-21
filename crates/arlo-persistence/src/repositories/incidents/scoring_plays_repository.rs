use crate::error::PersistenceResult;
use crate::models::MatchScoringPlayRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "team_id",
    "scorer_id",
    "artrine_id",
    "assister_id",
    "play_type",
    "points",
    "scoring_post",
    "drives_completed",
    "territory_advance_mirim",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchScoringPlayRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_scoring_plays (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            scorer_id,
            artrine_id,
            assister_id,
            play_type,
            points,
            scoring_post,
            drives_completed,
            territory_advance_mirim
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(&row.scorer_id)
    .bind(&row.artrine_id)
    .bind(&row.assister_id)
    .bind(&row.play_type)
    .bind(row.points)
    .bind(&row.scoring_post)
    .bind(row.drives_completed)
    .bind(row.territory_advance_mirim)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchScoringPlayRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_scoring_plays", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(&row.team_id);
        b.push_bind(&row.scorer_id);
        b.push_bind(&row.artrine_id);
        b.push_bind(&row.assister_id);
        b.push_bind(&row.play_type);
        b.push_bind(row.points);
        b.push_bind(&row.scoring_post);
        b.push_bind(row.drives_completed);
        b.push_bind(row.territory_advance_mirim);
    })
    .await
}
