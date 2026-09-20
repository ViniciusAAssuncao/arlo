use crate::dto::player::{PlayerAttributeDto, PlayerPositionDto, PlayerProfileDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::{
    get_or_load_attribute_definitions, get_or_load_attribute_key_index,
};
use crate::services::player::player_ability_service::calculate_player_ability;
use arlo_db::models::attribute_key_code::attribute_key_to_code;
use arlo_db::models::position_code::position_to_code;
use arlo_domain::CaptaincyRole;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_player_profile(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<PlayerProfileDto> {
    let player = arlo_db::repositories::player::get_by_id(pool, player_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Player {} not found", player_id)))?;

    let nationality_name = match arlo_db::repositories::country::get_by_id(pool, player.nationality_id())
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
    {
        Some(country) => country.name().to_string(),
        None => String::new(),
    };

    let (team_id, team_name) = match player.team_id() {
        Some(tid) => {
            let team_opt = arlo_db::repositories::team::get_by_id(pool, tid)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            (Some(tid.to_string()), team_opt.map(|t| t.name().to_string()))
        }
        None => (None, None),
    };

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

    let current_date_unix_seconds = if let Some(row) = &save_calendar_row {
        (row.current_year - 1970) * 31_557_600 + (row.current_day_of_year as i64) * 86_400
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        now
    };

    let age = if current_date_unix_seconds > player.birthdate_unix_seconds() {
        ((current_date_unix_seconds - player.birthdate_unix_seconds()) / 31_557_600) as u32
    } else {
        0
    };

    let key_index = get_or_load_attribute_key_index(pool).await?;
    let current_ability = calculate_player_ability(&player, &key_index);

    let captaincy_role = player.captaincy_role().map(|r| match r {
        CaptaincyRole::Captain => "Captain".to_string(),
        CaptaincyRole::ViceCaptain => "ViceCaptain".to_string(),
    });

    let mut positions: Vec<PlayerPositionDto> = player
        .positions()
        .iter()
        .map(|p| PlayerPositionDto {
            position: position_to_code(p.position()).to_string(),
            proficiency: p.proficiency(),
        })
        .collect();
    positions.sort_by(|a, b| b.proficiency.cmp(&a.proficiency));

    let defs_map = get_or_load_attribute_definitions(pool).await?;

    let attr_rows = sqlx::query_as::<_, (String, i32)>(
        "SELECT attribute_definition_id, value FROM player_attributes WHERE player_id = ?",
    )
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    let mut attributes_by_category: HashMap<String, Vec<PlayerAttributeDto>> = HashMap::new();

    for (def_id_str, value) in attr_rows {
        if let Ok(def_id) = Uuid::parse_str(&def_id_str) {
            if let Some(def) = defs_map.get(&def_id) {
                let cat_name = format!("{:?}", def.category());
                let attr_dto = PlayerAttributeDto {
                    key: attribute_key_to_code(def.key()).to_string(),
                    name: def.display_name().to_string(),
                    value,
                };
                attributes_by_category
                    .entry(cat_name)
                    .or_default()
                    .push(attr_dto);
            }
        }
    }

    for attrs in attributes_by_category.values_mut() {
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
    }

    Ok(PlayerProfileDto {
        id: player.id().to_string(),
        name: player.name().to_string(),
        birthdate_unix_seconds: player.birthdate_unix_seconds(),
        age,
        height_m: player.height_m(),
        nationality_id: player.nationality_id().to_string(),
        nationality_name,
        squad_number: player.squad_number(),
        captaincy_role,
        team_id,
        team_name,
        positions,
        current_ability,
        attributes_by_category,
    })
}