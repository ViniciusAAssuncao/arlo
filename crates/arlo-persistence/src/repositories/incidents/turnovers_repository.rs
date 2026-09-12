use crate::error::PersistenceResult;
use crate::models::MatchTurnoverRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTurnoverRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_turnovers (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            previous_offense_team_id,
            new_offense_team_id,
            recovering_player_id,
            lost_by_player_id,
            in_live_play,
            point_x,
            point_y
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.previous_offense_team_id)
    .bind(&row.new_offense_team_id)
    .bind(&row.recovering_player_id)
    .bind(&row.lost_by_player_id)
    .bind(row.in_live_play)
    .bind(row.point_x)
    .bind(row.point_y)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTurnoverRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
