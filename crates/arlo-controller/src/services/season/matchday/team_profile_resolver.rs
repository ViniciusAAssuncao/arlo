use crate::error::{ControllerError, ControllerResult};
use arlo_tactics::{Mentality, TeamInstructions, TeamTacticalProfile};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_team_instructions(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<TeamTacticalProfile> {
    if let Some(profile) = arlo_tactics::team_instructions::get_active_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
    {
        return Ok(profile);
    }

    let default_instructions = TeamInstructions::builder(Mentality::default()).build();

    let profile_id = arlo_tactics::team_instructions::insert_profile(
        pool,
        team_id,
        "Instruções Padrão",
        &default_instructions,
        None,
    )
    .await
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    arlo_tactics::team_instructions::set_active(pool, profile_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let profile = arlo_tactics::team_instructions::get_active_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| {
            ControllerError::NotFound(
                "Failed to load newly created team tactical profile".to_string(),
            )
        })?;

    Ok(profile)
}