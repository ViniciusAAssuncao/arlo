use crate::error::PersistenceResult;
use crate::models::MatchPlayerReceivingRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{ Sqlite, Transaction };

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "targets",
    "receptions",
    "drops",
    "catch_rate",
    "drop_rate",
    "receiving_mirins",
    "run_after_catch_mirins",
    "longest_reception_mirim",
    "average_mirins_per_reception",
    "average_rac_per_reception",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerReceivingRow
) -> PersistenceResult<()> {
    sqlx
        ::query(
            r#"INSERT INTO match_player_receiving (
            id,
            match_id,
            player_id,
            targets,
            receptions,
            drops,
            catch_rate,
            drop_rate,
            receiving_mirins,
            run_after_catch_mirins,
            longest_reception_mirim,
            average_mirins_per_reception,
            average_rac_per_reception
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&row.id)
        .bind(&row.match_id)
        .bind(&row.player_id)
        .bind(row.targets)
        .bind(row.receptions)
        .bind(row.drops)
        .bind(row.catch_rate)
        .bind(row.drop_rate)
        .bind(row.receiving_mirins)
        .bind(row.run_after_catch_mirins)
        .bind(row.longest_reception_mirim)
        .bind(row.average_mirins_per_reception)
        .bind(row.average_rac_per_reception)
        .execute(&mut **tx).await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerReceivingRow]
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_player_receiving", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.targets);
        b.push_bind(row.receptions);
        b.push_bind(row.drops);
        b.push_bind(row.catch_rate);
        b.push_bind(row.drop_rate);
        b.push_bind(row.receiving_mirins);
        b.push_bind(row.run_after_catch_mirins);
        b.push_bind(row.longest_reception_mirim);
        b.push_bind(row.average_mirins_per_reception);
        b.push_bind(row.average_rac_per_reception);
    }).await
}
