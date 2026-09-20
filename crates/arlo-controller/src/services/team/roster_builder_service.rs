use crate::controllers::season::overview_calendar::resolve_overview_calendar;
use crate::domain::calendar::CalendarDate;
use crate::dto::team::RosterEntryDto;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_attribute_key_index;
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::player::player_ability_service::calculate_player_ability;
use crate::services::season::conflict::team_fixture_window_loader::days_between;
use arlo_recovery::availability::resolve_batch_player_statuses;
use arlo_recovery::InjuryStatusKind;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
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

    let player_ids: Vec<Uuid> = players.iter().map(|p| p.id()).collect();

    let condition_rows = arlo_persistence::repositories::condition::player_condition::list_all(pool).await?;
    let mut condition_map = HashMap::with_capacity(condition_rows.len());
    for row in condition_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            condition_map.insert(pid, row);
        }
    }

    let medical_statuses = resolve_batch_player_statuses(pool, &player_ids)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let injury_defs = arlo_db::repositories::injury_definition::list_all(pool).await.unwrap_or_default();
    let def_map: HashMap<Uuid, String> = injury_defs.into_iter().map(|d| (d.id(), d.description().to_string())).collect();

    let latest_physical_rows = arlo_persistence::repositories::player::physical_repository::list_latest_by_player_ids(
        pool,
        &player_ids,
    )
    .await?;
    let mut latest_physical_map = HashMap::with_capacity(latest_physical_rows.len());
    for row in latest_physical_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            latest_physical_map.insert(pid, row);
        }
    }

    let latest_impulse_rows = arlo_persistence::repositories::player::impulse_repository::list_latest_by_player_ids(
        pool,
        &player_ids,
    )
    .await?;
    let mut latest_impulse_map = HashMap::with_capacity(latest_impulse_rows.len());
    for row in latest_impulse_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            latest_impulse_map.insert(pid, row);
        }
    }

    let mut distinct_match_ids = HashSet::new();
    for pid in &player_ids {
        if let Some(phys) = latest_physical_map.get(pid) {
            distinct_match_ids.insert(phys.match_id.clone());
        } else if let Some(imp) = latest_impulse_map.get(pid) {
            distinct_match_ids.insert(imp.match_id.clone());
        }
    }

    let mut match_days_cache: HashMap<String, Option<u32>> = HashMap::with_capacity(distinct_match_ids.len());

    if !distinct_match_ids.is_empty() {
        let match_id_vec: Vec<String> = distinct_match_ids.into_iter().collect();
        let placeholders = std::iter::repeat("?").take(match_id_vec.len()).collect::<Vec<_>>().join(", ");
        let sql = format!(
            r#"SELECT m.id, f.scheduled_year, f.scheduled_day_of_year
               FROM matches m
               JOIN fixtures f ON m.fixture_id = f.id
               WHERE m.id IN ({})"#,
            placeholders
        );

        let mut query = sqlx::query_as::<_, (String, i64, i32)>(&sql);
        for m_id in &match_id_vec {
            query = query.bind(m_id);
        }

        let fixture_infos: Vec<(String, i64, i32)> = query.fetch_all(pool).await.unwrap_or_default();

        for (m_id, year, day) in fixture_infos {
            let computed_days = match calendar_opt {
                Some(cal) => {
                    let f_date = CalendarDate::new(year, day as u32);
                    let diff = days_between(cal, &f_date, &current_date);
                    Some(diff.max(0) as u32)
                }
                None => {
                    let f_unix = (year - 1970) * 31_557_600 + (day as i64) * 86_400;
                    let diff = ((current_date_unix_seconds - f_unix) / 86_400).max(0);
                    Some(diff as u32)
                }
            };
            match_days_cache.insert(m_id, computed_days);
        }
    }

    let mut roster_entries = Vec::with_capacity(players.len());

    for player in players {
        let pid = player.id();

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

        let (condition, morale, conditioning_score) = if let Some(cond) = condition_map.get(&pid) {
            (cond.energy_level, cond.impulse_current_value as f64, cond.conditioning_score)
        } else {
            let c = latest_physical_map.get(&pid).map(|p| p.end_energy_level).unwrap_or(1.0);
            let m = latest_impulse_map.get(&pid).map(|i| i.current_value as f64).unwrap_or(50.0);
            (c, m, 0.5)
        };

        let med_status = medical_statuses.get(&pid);
        let is_injured = med_status.map(|s| s.is_injured()).unwrap_or(false);
        let injury_status = med_status
            .map(|s| s.display_status().to_string())
            .unwrap_or_else(|| "Healthy".to_string());
        let injury_name = med_status
            .filter(|s| s.status != InjuryStatusKind::Healthy)
            .and_then(|s| s.injury_record.as_ref())
            .and_then(|r| def_map.get(&r.injury_definition_id()).cloned());
        let injury_days_remaining = med_status.and_then(|s| s.active_days_remaining());

        let last_match_id = latest_physical_map
            .get(&pid)
            .map(|p| p.match_id.clone())
            .or_else(|| latest_impulse_map.get(&pid).map(|i| i.match_id.clone()));

        let days_since_last_match = last_match_id.and_then(|m_id| match_days_cache.get(&m_id).copied().flatten());

        roster_entries.push(RosterEntryDto {
            player_id: player.id().to_string(),
            name: player.name().to_string(),
            squad_number: player.squad_number(),
            position,
            age,
            current_ability: ca,
            condition,
            morale,
            conditioning_score,
            height_m: player.height_m(),
            days_since_last_match,
            is_injured,
            injury_status,
            injury_name,
            injury_days_remaining,
        });
    }

    roster_entries.sort_by_key(|p| (p.squad_number.is_none(), p.squad_number, p.name.clone()));

    Ok(roster_entries)
}