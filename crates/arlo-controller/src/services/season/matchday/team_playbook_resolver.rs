use crate::error::{ControllerError, ControllerResult};
use arlo_tactics::{DecisionEmphasis, PlayCall, PlayCallCategory};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_team_playbook(
    pool: &SqlitePool,
    team_id: Uuid,
    tactical_lineup_id: Uuid,
) -> ControllerResult<Vec<PlayCall>> {
    let existing = arlo_tactics::play_call::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if !existing.is_empty() {
        return Ok(existing);
    }

    let default_play_call = PlayCall::new(
        Uuid::new_v4(),
        team_id,
        tactical_lineup_id,
        "Jogada Padrão",
        PlayCallCategory::OpenPlay,
        None,
        DecisionEmphasis::default(),
        Vec::new(),
        Vec::new(),
        None,
        None,
    );

    arlo_tactics::play_call::insert(pool, &default_play_call)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    Ok(vec![default_play_call])
}
