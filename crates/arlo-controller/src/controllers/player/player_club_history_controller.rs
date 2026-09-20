use crate::dto::player::PlayerClubHistoryDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_persistence::models::player::player_club_history_row::PlayerClubHistoryRow;
use sqlx::SqlitePool;
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

            let new_row = PlayerClubHistoryRow::new(
                Uuid::new_v4(),
                player_id,
                team_id,
                current_year,
                None,
                now,
            );

            arlo_persistence::repositories::player::player_club_history::insert(pool, &new_row)
                .await?;

            rows.push(new_row);
        }
    }

    let mut dtos = Vec::with_capacity(rows.len());

    for row in rows {
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
