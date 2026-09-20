use crate::dto::player::PlayerMedicalConditionDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_recovery::availability::resolve_player_status;
use arlo_recovery::{
    resolve_player_readiness, InjuryStatusKind, ReadinessLevel, ReadinessTuningProfile,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
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

    let current_unix_seconds = if let Some(row) = &save_calendar_row {
        (row.current_year - 1970) * 31_557_600 + (row.current_day_of_year as i64) * 86_400
    } else {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    };

    let readiness_tuning = ReadinessTuningProfile::default();
    let readiness = resolve_player_readiness(
        pool,
        player_id,
        current_unix_seconds,
        &readiness_tuning,
    )
    .await
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let readiness_score = readiness.score;
    let readiness_level = readiness.level.as_str().to_string();
    let caution_recommended = matches!(
        readiness.level,
        ReadinessLevel::RecentReturn | ReadinessLevel::CautionRecommended | ReadinessLevel::HighRisk
    );

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
        readiness_score,
        readiness_level,
        caution_recommended,
    })
}