use crate::dto::team::TeamTitleDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_domain::CompetitionKind;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn list_team_titles(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<Vec<TeamTitleDto>> {
    let titles = arlo_db::repositories::title::list_by_winner_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if titles.is_empty() {
        return Ok(Vec::new());
    }

    let competitions = arlo_db::repositories::competition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let competition_map: HashMap<Uuid, (String, String)> = competitions
        .into_iter()
        .map(|c| {
            let kind_str = match c.kind() {
                CompetitionKind::League => "League".to_string(),
                CompetitionKind::Cup => "Cup".to_string(),
                CompetitionKind::Friendly => "Friendly".to_string(),
            };
            (c.id(), (c.name().to_string(), kind_str))
        })
        .collect();

    let mut dtos = Vec::with_capacity(titles.len());

    for title in titles {
        let (competition_name, competition_kind) = competition_map
            .get(&title.competition_id())
            .cloned()
            .unwrap_or_else(|| {
                (
                    "Competição Desconhecida".to_string(),
                    "Desconhecido".to_string(),
                )
            });

        dtos.push(TeamTitleDto {
            id: title.id().to_string(),
            competition_id: title.competition_id().to_string(),
            competition_name,
            competition_kind,
            season_label: title.season_label().to_string(),
        });
    }

    Ok(dtos)
}
