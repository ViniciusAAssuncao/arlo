use crate::dto::r#match::{MatchSummaryDto, MatchTeamScoreDto, QuarterScoreDto};
use crate::dto::team::TeamVenueDto;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_referee_attribute_definitions;
use crate::services::r#match::match_quarter_score_calculator::calculate_quarter_scores;
use arlo_persistence::models::incidents::{MatchAddedTimeRow, MatchScoringPlayRow};
use arlo_persistence::models::{MatchRow, MatchTeamScoreRow};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_match_summary(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchSummaryDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let team_scores =
        arlo_persistence::repositories::match_team_score::list_by_match_id(pool, match_id).await?;

    let scoring_plays =
        arlo_persistence::repositories::match_scoring_plays::list_by_match_id(pool, match_id)
            .await?;

    let added_time_records =
        arlo_persistence::repositories::match_added_time::list_by_match_id(pool, match_id).await?;

    build_match_summary_with_incidents(
        pool,
        &match_row,
        &team_scores,
        &scoring_plays,
        &added_time_records,
    )
    .await
}

pub async fn build_match_summary_with_incidents(
    pool: &SqlitePool,
    match_row: &MatchRow,
    team_scores: &[MatchTeamScoreRow],
    scoring_plays: &[MatchScoringPlayRow],
    added_time_records: &[MatchAddedTimeRow],
) -> ControllerResult<MatchSummaryDto> {
    let home_team_uuid = Uuid::parse_str(&match_row.home_team_id)?;
    let away_team_uuid = Uuid::parse_str(&match_row.away_team_id)?;

    let home_team = arlo_db::repositories::team::get_by_id(pool, home_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let away_team = arlo_db::repositories::team::get_by_id(pool, away_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let home_team_name = home_team
        .as_ref()
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Mandante".to_string());
    let home_team_primary_color_hex = home_team
        .as_ref()
        .and_then(|t| t.primary_color_hex().map(str::to_string));
    let home_team_secondary_color_hex = home_team
        .as_ref()
        .and_then(|t| t.secondary_color_hex().map(str::to_string));

    let away_team_name = away_team
        .as_ref()
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Visitante".to_string());
    let away_team_primary_color_hex = away_team
        .as_ref()
        .and_then(|t| t.primary_color_hex().map(str::to_string));
    let away_team_secondary_color_hex = away_team
        .as_ref()
        .and_then(|t| t.secondary_color_hex().map(str::to_string));

    let venue_uuid = match_row
        .venue_id
        .as_deref()
        .and_then(|vid| Uuid::parse_str(vid).ok())
        .or_else(|| home_team.as_ref().and_then(|t| t.home_venue_id()));

    let venue = match venue_uuid {
        Some(vid) => {
            let venue_opt = arlo_db::repositories::venue::get_by_id(pool, vid)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            venue_opt.map(|v| TeamVenueDto {
                id: v.id().to_string(),
                name: v.name().to_string(),
                capacity: v.capacity(),
                pitch_length_mirim: v.pitch_length_mirim(),
                pitch_width_mirim: v.pitch_width_mirim(),
            })
        }
        None => None,
    };

    let head_referee_uuid = Uuid::parse_str(&match_row.head_referee_id)?;
    let peace_referee_uuid = Uuid::parse_str(&match_row.peace_referee_id)?;
    let referee_defs = get_or_load_referee_attribute_definitions(pool).await?;

    let head_ref =
        arlo_db::repositories::referee::get_by_id(pool, head_referee_uuid, &referee_defs)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let head_referee_name = head_ref
        .map(|r| r.person().name().to_string())
        .unwrap_or_else(|| "Árbitro Principal".to_string());

    let peace_ref =
        arlo_db::repositories::referee::get_by_id(pool, peace_referee_uuid, &referee_defs)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let peace_referee_name = peace_ref
        .map(|r| r.person().name().to_string())
        .unwrap_or_else(|| "Árbitro de Paz".to_string());

    let home_score_row = team_scores
        .iter()
        .find(|s| s.is_home || s.team_id == match_row.home_team_id);
    let away_score_row = team_scores
        .iter()
        .find(|s| !s.is_home || s.team_id == match_row.away_team_id);

    let home_score = home_score_row.map(|s| s.total_points).unwrap_or(0);
    let home_goal_points = home_score_row.map(|s| s.goal_points).unwrap_or(0);
    let home_field_goals = home_score_row.map(|s| s.field_goals).unwrap_or(0);
    let home_field_points = home_score_row.map(|s| s.field_points).unwrap_or(0);
    let home_score_formatted = format!(
        "{}–{}–{} ({})",
        home_goal_points, home_field_goals, home_field_points, home_score
    );

    let away_score = away_score_row.map(|s| s.total_points).unwrap_or(0);
    let away_goal_points = away_score_row.map(|s| s.goal_points).unwrap_or(0);
    let away_field_goals = away_score_row.map(|s| s.field_goals).unwrap_or(0);
    let away_field_points = away_score_row.map(|s| s.field_points).unwrap_or(0);
    let away_score_formatted = format!(
        "{}–{}–{} ({})",
        away_goal_points, away_field_goals, away_field_points, away_score
    );

    let final_period = match_row.final_period.max(1) as u32;
    let quarter_scores_domain = calculate_quarter_scores(
        &match_row.home_team_id,
        &match_row.away_team_id,
        scoring_plays,
        added_time_records,
        final_period,
    );
    let quarter_scores = quarter_scores_domain
        .into_iter()
        .map(QuarterScoreDto::from)
        .collect();

    let home_team_score = MatchTeamScoreDto {
        team_id: match_row.home_team_id.clone(),
        team_name: home_team_name,
        is_home: true,
        primary_color_hex: home_team_primary_color_hex,
        secondary_color_hex: home_team_secondary_color_hex,
        total_points: home_score,
        goal_points: home_goal_points,
        field_goals: home_field_goals,
        field_points: home_field_points,
        formatted_score: home_score_formatted,
    };

    let away_team_score = MatchTeamScoreDto {
        team_id: match_row.away_team_id.clone(),
        team_name: away_team_name,
        is_home: false,
        primary_color_hex: away_team_primary_color_hex,
        secondary_color_hex: away_team_secondary_color_hex,
        total_points: away_score,
        goal_points: away_goal_points,
        field_goals: away_field_goals,
        field_points: away_field_points,
        formatted_score: away_score_formatted,
    };

    Ok(MatchSummaryDto {
        match_id: match_row.id.clone(),
        fixture_id: match_row.fixture_id.clone(),
        home_team: home_team_score,
        away_team: away_team_score,
        venue,
        head_referee_id: match_row.head_referee_id.clone(),
        head_referee_name,
        peace_referee_id: match_row.peace_referee_id.clone(),
        peace_referee_name,
        quarter_scores,
        final_period,
        went_to_overtime: match_row.went_to_overtime,
        attendance: None,
        completed_at_unix_seconds: match_row.completed_at_unix_seconds,
    })
}
