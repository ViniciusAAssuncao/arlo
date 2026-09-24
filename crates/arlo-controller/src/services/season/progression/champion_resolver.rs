use crate::domain::season::KnockoutTie;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache::get_or_compute_standings;
use crate::services::season::persistence::{load_stage_knockout_ties, map_row_to_fixture};
use crate::services::season::stage::knockout_bracket_progress_detector::{
    detect_knockout_bracket_progress, KnockoutBracketProgress,
};
use arlo_domain::{StageType, Title, TitleWinner};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_season_champion(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
    champion_team_id: Option<Uuid>,
) -> ControllerResult<Title> {
    let season_row = arlo_persistence::repositories::season::season_instances::get_by_id(
        pool,
        season_instance_id,
    )
    .await?
    .ok_or_else(|| {
        ControllerError::NotFound(format!("Season instance {} not found", season_instance_id))
    })?;

    let reference_year = season_row.reference_year;

    let winner_team_id = if let Some(winner_id) = champion_team_id {
        winner_id
    } else {
        let config = get_or_load_league_calendar_config(pool, competition_id)
            .await?
            .ok_or_else(|| {
                ControllerError::NotFound(format!(
                    "League calendar config for competition {} not found",
                    competition_id
                ))
            })?;

        let last_stage_def = config
            .stages()
            .iter()
            .max_by_key(|s| s.stage_order_index())
            .ok_or_else(|| {
                ControllerError::NotFound(format!(
                    "No stages defined for competition {}",
                    competition_id
                ))
            })?;

        let stages =
            arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
                pool,
                season_instance_id,
            )
            .await?;

        let stage_row = stages
            .iter()
            .find(|s| s.stage_order_index == (last_stage_def.stage_order_index() as i32))
            .ok_or_else(|| {
                ControllerError::NotFound(format!(
                    "Stage with order index {} not found for season {}",
                    last_stage_def.stage_order_index(),
                    season_instance_id
                ))
            })?;

        let stage_id = Uuid::parse_str(&stage_row.id)?;

        match last_stage_def.stage_type() {
            StageType::KnockoutBracket => {
                let ties: Vec<KnockoutTie> = load_stage_knockout_ties(pool, stage_id).await?;
                let fixture_rows =
                    arlo_persistence::repositories::season::fixtures::list_by_stage_id(
                        pool, stage_id,
                    )
                    .await?;
                let mut fixtures = Vec::with_capacity(fixture_rows.len());
                for row in &fixture_rows {
                    fixtures.push(map_row_to_fixture(row)?);
                }

                match detect_knockout_bracket_progress(
                    &ties,
                    &fixtures,
                    config.tie_break_criteria(),
                ) {
                    KnockoutBracketProgress::BracketComplete { champion_team_id } => {
                        champion_team_id
                    }
                    _ => {
                        return Err(ControllerError::Validation(format!(
                            "Knockout bracket stage ended without a determined champion for season {}",
                            season_instance_id
                        )));
                    }
                }
            }
            StageType::RoundRobinTable | StageType::GroupedCompetitionTable => {
                let standings = get_or_compute_standings(pool, stage_id).await?;
                let top_entry = standings.first().ok_or_else(|| {
                    ControllerError::Validation(format!(
                        "Standings are empty for stage {}",
                        stage_id
                    ))
                })?;
                top_entry.team_id()
            }
        }
    };

    let season_label = reference_year.to_string();
    let title = Title::new(
        Uuid::new_v4(),
        competition_id,
        season_label,
        TitleWinner::Team(winner_team_id),
    )
    .map_err(|e| ControllerError::Validation(e.to_string()))?;

    Ok(title)
}
