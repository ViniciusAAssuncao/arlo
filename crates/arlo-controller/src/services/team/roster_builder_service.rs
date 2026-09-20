use crate::controllers::season::overview_calendar::resolve_overview_calendar;
use crate::domain::calendar::CalendarDate;
use crate::dto::team::RosterEntryDto;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_attribute_key_index;
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::player::player_ability_service::calculate_player_ability;
use crate::services::season::conflict::team_fixture_window_loader::days_between;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn build_team_roster(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<Vec<RosterEntryDto>> {
    let _team = arlo_db::repositories::team::get_by_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Team {} not found", team_id)))?;

    let players = arlo_db::repositories::player::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if players.is_empty() {
        return Ok(Vec::new());
    }

    let key_index = get_or_load_attribute_key_index(pool).await?;

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

    let (current_year, current_day_of_year, current_date_unix_seconds) = if let Some(row) =
        &save_calendar_row
    {
        let year = row.current_year;
        let day = row.current_day_of_year as u32;
        let unix_sec = (year - 1970) * 31_557_600 + (day as i64) * 86_400;
        (year, day, unix_sec)
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let year = 1970 + now / 31_557_600;
        (year, 1, now)
    };

    let current_date = CalendarDate::new(current_year, current_day_of_year);

    let catalog_res = get_or_load_calendar_catalog(pool).await;
    let calendar_opt = if let Ok(catalog) = &catalog_res {
        resolve_overview_calendar(pool, catalog).await.ok()
    } else {
        None
    };

    let mut match_days_cache: HashMap<String, Option<u32>> = HashMap::new();
    let mut roster_entries = Vec::with_capacity(players.len());

    for player in players {
        let position = player
            .positions()
            .iter()
            .max_by_key(|p| p.proficiency())
            .map(|p| arlo_db::models::position_code::position_to_code(p.position()).to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let age = if current_date_unix_seconds > player.birthdate_unix_seconds() {
            ((current_date_unix_seconds - player.birthdate_unix_seconds()) / 31_557_600) as u32
        } else {
            0
        };

        let ca = calculate_player_ability(&player, &key_index);

        let latest_physical = arlo_persistence::repositories::player::physical_repository::get_latest_by_player_id(
            pool,
            player.id(),
        )
        .await?;

        let latest_impulse = arlo_persistence::repositories::player::impulse_repository::get_latest_by_player_id(
            pool,
            player.id(),
        )
        .await?;

        let condition = latest_physical
            .as_ref()
            .map(|p| p.end_energy_level)
            .unwrap_or(1.0);

        let morale = latest_impulse
            .as_ref()
            .map(|i| i.current_value as f64)
            .unwrap_or(50.0);

        let last_match_id = latest_physical
            .as_ref()
            .map(|p| p.match_id.clone())
            .or_else(|| latest_impulse.as_ref().map(|i| i.match_id.clone()));

        let days_since_last_match = if let Some(m_id) = last_match_id {
            if let Some(&cached) = match_days_cache.get(&m_id) {
                cached
            } else {
                let fixture_info: Option<(i64, i32)> = sqlx::query_as::<_, (i64, i32)>(
                    r#"SELECT f.scheduled_year, f.scheduled_day_of_year
                       FROM matches m
                       JOIN fixtures f ON m.fixture_id = f.id
                       WHERE m.id = ?"#,
                )
                .bind(&m_id)
                .fetch_optional(pool)
                .await
                .unwrap_or(None);

                let computed_days = match (fixture_info, calendar_opt) {
                    (Some((year, day)), Some(cal)) => {
                        let f_date = CalendarDate::new(year, day as u32);
                        let diff = days_between(cal, &f_date, &current_date);
                        Some(diff.max(0) as u32)
                    }
                    (Some((year, day)), None) => {
                        let f_unix = (year - 1970) * 31_557_600 + (day as i64) * 86_400;
                        let diff = ((current_date_unix_seconds - f_unix) / 86_400).max(0);
                        Some(diff as u32)
                    }
                    _ => {
                        let completed_at: Option<Option<i64>> = sqlx::query_scalar(
                            "SELECT completed_at_unix_seconds FROM matches WHERE id = ?",
                        )
                        .bind(&m_id)
                        .fetch_optional(pool)
                        .await
                        .unwrap_or(None);

                        if let Some(Some(comp_sec)) = completed_at {
                            if comp_sec > 0 {
                                let diff = ((current_date_unix_seconds - comp_sec) / 86_400).max(0);
                                Some(diff as u32)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                };

                match_days_cache.insert(m_id, computed_days);
                computed_days
            }
        } else {
            None
        };

        roster_entries.push(RosterEntryDto {
            player_id: player.id().to_string(),
            name: player.name().to_string(),
            squad_number: player.squad_number(),
            position,
            age,
            current_ability: ca,
            condition,
            morale,
            height_m: player.height_m(),
            days_since_last_match,
        });
    }

    roster_entries.sort_by_key(|p| (p.squad_number.is_none(), p.squad_number, p.name.clone()));

    Ok(roster_entries)
}