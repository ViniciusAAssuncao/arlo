use crate::error::{ControllerError, ControllerResult};
use arlo_domain::Referee;
use sqlx::SqlitePool;

pub async fn select_referees(
    pool: &SqlitePool,
    seed: u64,
) -> ControllerResult<(Referee, Referee)> {
    let referees = arlo_db::repositories::referee::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if referees.len() < 2 {
        return Err(ControllerError::NotFound(
            "At least 2 referees are required to officiate a match".to_string(),
        ));
    }

    let head_idx = (seed as usize) % referees.len();
    let offset = ((seed >> 16) as usize % (referees.len() - 1)) + 1;
    let peace_idx = (head_idx + offset) % referees.len();

    let head_referee = referees[head_idx].clone();
    let peace_referee = referees[peace_idx].clone();

    Ok((head_referee, peace_referee))
}