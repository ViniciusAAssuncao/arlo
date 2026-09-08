use crate::error::{TacticsError, TacticsResult};
use crate::persistence::models::rows::{SeriesScriptEntryRow, SeriesScriptRow};
use crate::series::SeriesScript;
use arlo_db::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> TacticsResult<Option<SeriesScript>> {
    let row = fetch_optional_by_param::<SeriesScriptRow>(
        pool,
        "SELECT id, team_id, name, created_at_unix_seconds FROM series_scripts WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let entry_rows = sqlx::query_as::<_, SeriesScriptEntryRow>(
        "SELECT id, series_script_id, sequence_index, play_call_id FROM series_script_entries WHERE series_script_id = ? ORDER BY sequence_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(Some(row.to_domain(&entry_rows)?))
}

pub async fn list_by_team_id(pool: &SqlitePool, team_id: Uuid) -> TacticsResult<Vec<SeriesScript>> {
    let rows = fetch_all_by_param::<SeriesScriptRow>(
        pool,
        "SELECT id, team_id, name, created_at_unix_seconds FROM series_scripts WHERE team_id = ? ORDER BY created_at_unix_seconds ASC",
        &team_id.to_string(),
    )
    .await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let entry_rows = sqlx::query_as::<_, SeriesScriptEntryRow>(
            "SELECT id, series_script_id, sequence_index, play_call_id FROM series_script_entries WHERE series_script_id = ? ORDER BY sequence_index ASC",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        results.push(row.to_domain(&entry_rows)?);
    }

    Ok(results)
}

pub async fn insert(pool: &SqlitePool, script: &SeriesScript) -> TacticsResult<()> {
    let mut tx = pool.begin().await?;

    let team_id_str = script.team_id().to_string();

    for play_call_id in script.entries() {
        let play_row: Option<(String,)> =
            sqlx::query_as("SELECT team_id FROM play_calls WHERE id = ?")
                .bind(play_call_id.to_string())
                .fetch_optional(&mut *tx)
                .await?;

        match play_row {
            Some((play_team_id,)) => {
                if play_team_id != team_id_str {
                    return Err(TacticsError::InvalidPlayCall(format!(
                        "Play call {} belongs to a different team",
                        play_call_id
                    )));
                }
            }
            None => {
                return Err(TacticsError::InvalidPlayCall(format!(
                    "Play call {} not found",
                    play_call_id
                )));
            }
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    sqlx::query(
        "INSERT INTO series_scripts (id, team_id, name, created_at_unix_seconds) VALUES (?, ?, ?, ?)",
    )
    .bind(script.id().to_string())
    .bind(&team_id_str)
    .bind(script.name())
    .bind(timestamp)
    .execute(&mut *tx)
    .await?;

    for (idx, play_call_id) in script.entries().iter().enumerate() {
        let entry_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO series_script_entries (id, series_script_id, sequence_index, play_call_id) VALUES (?, ?, ?, ?)",
        )
        .bind(entry_id)
        .bind(script.id().to_string())
        .bind(idx as i32)
        .bind(play_call_id.to_string())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Uuid) -> TacticsResult<()> {
    sqlx::query("DELETE FROM series_scripts WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
