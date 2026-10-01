use crate::error::PersistenceResult;
use crate::models::condition::PlayerConditionRow;
use sqlx::{QueryBuilder, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn get_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
) -> PersistenceResult<Option<PlayerConditionRow>> {
    let row = sqlx::query_as::<_, PlayerConditionRow>(
        r#"SELECT
            player_id,
            energy_level,
            anaerobic_reserve,
            impulse_current_value,
            impulse_baseline,
            conditioning_score,
            last_updated_year,
            last_updated_day_of_year,
            last_match_year,
            last_match_day_of_year
        FROM player_condition
        WHERE player_id = ?"#,
    )
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_all(pool: &SqlitePool) -> PersistenceResult<Vec<PlayerConditionRow>> {
    let rows = sqlx::query_as::<_, PlayerConditionRow>(
        r#"SELECT
            player_id,
            energy_level,
            anaerobic_reserve,
            impulse_current_value,
            impulse_baseline,
            conditioning_score,
            last_updated_year,
            last_updated_day_of_year,
            last_match_year,
            last_match_day_of_year
        FROM player_condition"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn upsert(pool: &SqlitePool, row: &PlayerConditionRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO player_condition (
            player_id,
            energy_level,
            anaerobic_reserve,
            impulse_current_value,
            impulse_baseline,
            conditioning_score,
            last_updated_year,
            last_updated_day_of_year,
            last_match_year,
            last_match_day_of_year
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(player_id) DO UPDATE SET
            energy_level = excluded.energy_level,
            anaerobic_reserve = excluded.anaerobic_reserve,
            impulse_current_value = excluded.impulse_current_value,
            impulse_baseline = excluded.impulse_baseline,
            conditioning_score = excluded.conditioning_score,
            last_updated_year = excluded.last_updated_year,
            last_updated_day_of_year = excluded.last_updated_day_of_year,
            last_match_year = excluded.last_match_year,
            last_match_day_of_year = excluded.last_match_day_of_year"#,
    )
    .bind(&row.player_id)
    .bind(row.energy_level)
    .bind(row.anaerobic_reserve)
    .bind(row.impulse_current_value)
    .bind(row.impulse_baseline)
    .bind(row.conditioning_score)
    .bind(row.last_updated_year)
    .bind(row.last_updated_day_of_year)
    .bind(row.last_match_year)
    .bind(row.last_match_day_of_year)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn upsert_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    row: &PlayerConditionRow,
) -> PersistenceResult<()> {
    upsert_many_with_tx(tx, std::slice::from_ref(row)).await
}

pub async fn list_by_player_ids(
    pool: &SqlitePool,
    player_ids: &[Uuid],
) -> PersistenceResult<Vec<PlayerConditionRow>> {
    if player_ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut query = QueryBuilder::<Sqlite>::new(
        "SELECT player_id, energy_level, anaerobic_reserve, impulse_current_value, impulse_baseline, conditioning_score, last_updated_year, last_updated_day_of_year, last_match_year, last_match_day_of_year FROM player_condition WHERE player_id IN (",
    );
    let mut ids = query.separated(", ");
    for id in player_ids {
        ids.push_bind(id.to_string());
    }
    ids.push_unseparated(")");
    Ok(query.build_query_as::<PlayerConditionRow>().fetch_all(pool).await?)
}

pub async fn upsert_many_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[PlayerConditionRow],
) -> PersistenceResult<()> {
    for chunk in rows.chunks(300) {
        let mut query = QueryBuilder::<Sqlite>::new(
            "INSERT INTO player_condition (player_id, energy_level, anaerobic_reserve, impulse_current_value, impulse_baseline, conditioning_score, last_updated_year, last_updated_day_of_year, last_match_year, last_match_day_of_year) ",
        );
        query.push_values(chunk, |mut values, row| {
            values.push_bind(&row.player_id)
                .push_bind(row.energy_level)
                .push_bind(row.anaerobic_reserve)
                .push_bind(row.impulse_current_value)
                .push_bind(row.impulse_baseline)
                .push_bind(row.conditioning_score)
                .push_bind(row.last_updated_year)
                .push_bind(row.last_updated_day_of_year)
                .push_bind(row.last_match_year)
                .push_bind(row.last_match_day_of_year);
        });
        query.push(" ON CONFLICT(player_id) DO UPDATE SET energy_level = excluded.energy_level, anaerobic_reserve = excluded.anaerobic_reserve, impulse_current_value = excluded.impulse_current_value, impulse_baseline = excluded.impulse_baseline, conditioning_score = excluded.conditioning_score, last_updated_year = excluded.last_updated_year, last_updated_day_of_year = excluded.last_updated_day_of_year, last_match_year = excluded.last_match_year, last_match_day_of_year = excluded.last_match_day_of_year");
        query.build().execute(&mut **tx).await?;
    }
    Ok(())
}
