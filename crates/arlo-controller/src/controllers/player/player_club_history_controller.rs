use crate::dto::player::PlayerClubHistoryDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_persistence::models::player::player_club_history_row::PlayerClubHistoryRow;
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn list_club_history(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<Vec<PlayerClubHistoryDto>> {
    let mut rows = arlo_persistence::repositories::player::player_club_history::list_by_player_id(
        pool,
        player_id,
    )
    .await?;

    if rows.is_empty() {
        let player = arlo_db::repositories::player::get_by_id(pool, player_id)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?
            .ok_or_else(|| ControllerError::NotFound(format!("Player {} not found", player_id)))?;

        if let Some(team_id) = player.team_id() {
            let metadata = arlo_db::repositories::save_metadata::get(pool)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

            let save_calendar_row = if let Some(meta) = &metadata {
                arlo_persistence::repositories::calendar::save_calendar_state::get_by_save_uuid(
                    pool,
                    meta.save_uuid(),
                )
                .await?
            } else {
                None
            };

            let current_year = if let Some(state) = &save_calendar_row {
                state.current_year
            } else {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0);
                1970 + (now / 31_557_600)
            };

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            let new_id = Uuid::new_v4();
            let new_row = PlayerClubHistoryRow::new(
                new_id,
                player_id,
                team_id,
                current_year,
                None,
                now,
            );

            let inserted = sqlx::query(
                "INSERT INTO player_club_history (id, player_id, team_id, joined_year, left_year, created_at_unix_seconds)
                 SELECT ?, ?, ?, ?, NULL, ?
                 WHERE NOT EXISTS (
                     SELECT 1 FROM player_club_history WHERE player_id = ? AND team_id = ? AND left_year IS NULL
                 )",
            )
            .bind(new_id.to_string())
            .bind(player_id.to_string())
            .bind(team_id.to_string())
            .bind(current_year)
            .bind(now)
            .bind(player_id.to_string())
            .bind(team_id.to_string())
            .execute(pool)
            .await?;

            if inserted.rows_affected() > 0 {
                rows.push(new_row);
            } else {
                rows = arlo_persistence::repositories::player::player_club_history::list_by_player_id(
                    pool,
                    player_id,
                )
                .await?;
            }
        }
    }

    let mut duplicate_ids: Vec<String> = Vec::new();
    let mut seen_entries: HashSet<(String, i64, Option<i64>)> = HashSet::new();
    let mut seen_active_teams: HashSet<String> = HashSet::new();
    let mut sanitized_rows = Vec::with_capacity(rows.len());

    for row in rows {
        let is_active = row.left_year.is_none();
        let is_duplicate = if is_active {
            if !seen_active_teams.insert(row.team_id.clone()) {
                true
            } else {
                !seen_entries.insert((row.team_id.clone(), row.joined_year, row.left_year))
            }
        } else {
            !seen_entries.insert((row.team_id.clone(), row.joined_year, row.left_year))
        };

        if is_duplicate {
            duplicate_ids.push(row.id.clone());
        } else {
            sanitized_rows.push(row);
        }
    }

    if !duplicate_ids.is_empty() {
        for dup_id in &duplicate_ids {
            let _ = sqlx::query("DELETE FROM player_club_history WHERE id = ?")
                .bind(dup_id)
                .execute(pool)
                .await;
        }
    }

    let mut dtos = Vec::with_capacity(sanitized_rows.len());

    for row in sanitized_rows {
        let team_uuid = Uuid::parse_str(&row.team_id)?;
        let team = arlo_db::repositories::team::get_by_id(pool, team_uuid)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

        let team_name = team
            .map(|t| t.name().to_string())
            .unwrap_or_else(|| "Clube Desconhecido".to_string());

        let period_display = match row.left_year {
            Some(left) => format!("{}-{}", row.joined_year, left),
            None => format!("{}-", row.joined_year),
        };

        dtos.push(PlayerClubHistoryDto {
            id: row.id,
            player_id: row.player_id,
            team_id: row.team_id,
            team_name,
            joined_year: row.joined_year,
            left_year: row.left_year,
            period_display,
        });
    }

    Ok(dtos)
}