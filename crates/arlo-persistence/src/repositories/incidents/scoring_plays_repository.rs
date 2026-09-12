use crate::error::PersistenceResult;
use crate::models::MatchScoringPlayRow;
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
