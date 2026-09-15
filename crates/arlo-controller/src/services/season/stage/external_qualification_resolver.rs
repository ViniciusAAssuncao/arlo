use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{QualificationPoolRule, StageEntryRule, TitleWinner};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn resolve_external_winners(
    pool: &SqlitePool,
    entry_rule: &StageEntryRule,
) -> ControllerResult<HashMap<Uuid, Uuid>> {
    let mut winners = HashMap::new();
    for rule in entry_rule.pools() {
        if let QualificationPoolRule::ExternalCompetitionWinner { competition_id } = rule {
            if winners.contains_key(competition_id) {
                continue;
            }

            let title = arlo_db::repositories::title::get_latest_by_competition_id(pool, *competition_id)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?
                .ok_or_else(|| {
                    ControllerError::NotFound(format!(
                        "No title found for external competition {}",
                        competition_id
                    ))
                })?;

            match title.winner() {
                TitleWinner::Team(team_id) => {
                    winners.insert(*competition_id, team_id);
                }
                TitleWinner::Federation(_) => {
                    return Err(ControllerError::Validation(format!(
                        "Latest title for competition {} was won by a federation, not a team",
                        competition_id
                    )));
                }
            }
        }
    }
    Ok(winners)
}
