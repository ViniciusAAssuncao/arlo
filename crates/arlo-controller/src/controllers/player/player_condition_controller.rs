use crate::dto::player::PlayerMedicalConditionDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_recovery::availability::resolve_player_status;
use arlo_recovery::InjuryStatusKind;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_player_medical_condition(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<PlayerMedicalConditionDto> {
    let _player = arlo_db::repositories::player::get_by_id(pool, player_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Player {} not found", player_id)))?;

    let condition_row = arlo_persistence::repositories::condition::player_condition::get_by_player_id(
        pool,
        player_id,
    )
    .await?;

    let medical_status = resolve_player_status(pool, player_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let (energy_level, anaerobic_reserve, impulse_value, impulse_baseline, conditioning_score) =
        match condition_row {
            Some(c) => (
                c.energy_level,
                c.anaerobic_reserve,
                c.impulse_current_value as u8,
                c.impulse_baseline,
                c.conditioning_score,
            ),
            None => (1.0, 1.0, 50u8, 50.0, 0.5),
        };

    let injury_defs = arlo_db::repositories::injury_definition::list_all(pool)
        .await
        .unwrap_or_default();
    let def_map: HashMap<Uuid, String> = injury_defs
        .into_iter()
        .map(|d| (d.id(), d.description().to_string()))
        .collect();

    let (
        status,
        is_injured,
        injury_name,
        body_region,
        severity_grade,
        days_remaining,
        observation_days_remaining,
        expected_recovery_days,
        is_relapse,
    ) = match (&medical_status.injury_record, medical_status.status) {
        (Some(record), InjuryStatusKind::Injured | InjuryStatusKind::Observation) => {
            let name = def_map.get(&record.injury_definition_id()).cloned();
            (
                medical_status.display_status().to_string(),
                medical_status.is_injured(),
                name,
                medical_status.body_region_code().map(str::to_string),
                medical_status.severity_grade_code().map(str::to_string),
                Some(record.days_remaining()),
                Some(record.observation_days_remaining()),
                medical_status.expected_recovery_days,
                record.is_relapse(),
            )
        }
        _ => (
            medical_status.display_status().to_string(),
            false,
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        ),
    };

    Ok(PlayerMedicalConditionDto {
        player_id: player_id.to_string(),
        energy_level,
        anaerobic_reserve,
        impulse_value,
        impulse_baseline,
        conditioning_score,
        status,
        is_injured,
        injury_name,
        body_region,
        severity_grade,
        days_remaining,
        observation_days_remaining,
        expected_recovery_days,
        is_relapse,
    })
}