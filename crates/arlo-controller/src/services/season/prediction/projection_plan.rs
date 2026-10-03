use super::goal_points::fit_goal_point_model;
use super::knockout_margin::fit_knockout_margin;
use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::power_ranking::load_preseason_seeds;
use arlo_analytics::prediction::preseason::{
    CompetitionProjectionPlan, PreseasonConfig, ProjectionFixture, ProjectionStage, ProjectionTeam,
    ProjectionTie,
};
use arlo_analytics::{
    CalibrationScope, ForecastParameters, PowerRankingConfig, PowerRating, TeamPowerSeed,
    POWER_RANKING_MODEL_VERSION,
};
use arlo_domain::QualificationPoolRule;
use arlo_persistence::repositories::season::{
    fixtures, forecast_calibrations, knockout_ties, power_rankings, season_stages,
};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub(super) async fn build_projection_plan(
    pool: &SqlitePool,
    season_id: Uuid,
    competition_id: Uuid,
    reference_year: i64,
    as_of: CalendarDate,
    cutoff: CalendarDate,
    parameters: ForecastParameters,
    calibration_scope: CalibrationScope,
    config: PreseasonConfig,
) -> ControllerResult<Option<CompetitionProjectionPlan>> {
    let league = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound("league calendar config unavailable".into()))?;
    let stage_rows = season_stages::list_by_season_instance_id(pool, season_id).await?;
    let opening = stage_rows
        .iter()
        .find(|stage| stage.stage_order_index == 0)
        .ok_or_else(|| ControllerError::NotFound("opening stage unavailable".into()))?;
    let fixture_rows = fixtures::list_by_stage_id(pool, Uuid::parse_str(&opening.id)?).await?;
    if fixture_rows.is_empty() {
        return Ok(None);
    }
    let mut external_winners = HashMap::new();
    for stage in league.stages() {
        for pool_rule in stage.entry_rule().pools() {
            if let QualificationPoolRule::ExternalCompetitionWinner { competition_id } = pool_rule {
                if external_winners.contains_key(competition_id) {
                    continue;
                }
                let title: Option<(Option<String>,)> = sqlx::query_as(
                    "SELECT winner_team_id FROM titles WHERE competition_id = ? AND season_label = CAST(CAST(season_label AS INTEGER) AS TEXT) AND CAST(season_label AS INTEGER) <= ? ORDER BY CAST(season_label AS INTEGER) DESC, created_at_unix_seconds DESC, rowid DESC LIMIT 1",
                )
                .bind(competition_id.to_string())
                .bind(reference_year)
                .fetch_optional(pool)
                .await?;
                let Some((Some(team_id),)) = title else {
                    return Ok(None);
                };
                external_winners.insert(*competition_id, Uuid::parse_str(&team_id)?);
            }
        }
    }
    let power_config = PowerRankingConfig::default();
    let seeds = load_preseason_seeds(pool, season_id, as_of, &power_config).await?;
    let initial_team_ids: Vec<_> = seeds.iter().map(|seed| seed.team_id()).collect();
    let seed_ids: HashSet<_> = seeds.iter().map(|seed| seed.team_id()).collect();
    let previous = power_rankings::list_previous_ratings(
        pool,
        reference_year,
        cutoff.year(),
        cutoff.day_of_year(),
        POWER_RANKING_MODEL_VERSION,
    )
    .await?;
    let mut previous_ratings = HashMap::with_capacity(previous.len());
    for (id, rating) in previous {
        previous_ratings.insert(Uuid::parse_str(&id)?, PowerRating::new(rating)?);
    }
    let mut teams: Vec<_> = seeds
        .into_iter()
        .map(|seed| ProjectionTeam {
            has_rating_history: previous_ratings.contains_key(&seed.team_id()),
            seed,
        })
        .collect();
    for &team_id in external_winners.values() {
        if seed_ids.contains(&team_id) || teams.iter().any(|team| team.seed.team_id() == team_id) {
            continue;
        }
        let previous = previous_ratings.get(&team_id).copied();
        let rating = match previous {
            Some(rating) => power_config.carry_over(rating)?,
            None => power_config.center()?,
        };
        teams.push(ProjectionTeam {
            seed: TeamPowerSeed::new(team_id, rating),
            has_rating_history: previous.is_some(),
        });
    }
    let known_fixtures = fixture_rows
        .into_iter()
        .map(|row| {
            Ok(ProjectionFixture {
                round_index: u32::try_from(row.round_index)
                    .map_err(|_| ControllerError::InvalidData("invalid fixture round".into()))?,
                home_team_id: Uuid::parse_str(&row.home_team_id)?,
                away_team_id: Uuid::parse_str(&row.away_team_id)?,
                neutral_venue: row.is_neutral_venue,
            })
        })
        .collect::<ControllerResult<Vec<_>>>()?;
    let tie_rows =
        knockout_ties::list_by_season_stage_id(pool, Uuid::parse_str(&opening.id)?).await?;
    let first_round = tie_rows.first().map(|tie| tie.round_index);
    let known_ties = tie_rows
        .into_iter()
        .filter(|tie| Some(tie.round_index) == first_round)
        .map(|tie| {
            Ok(ProjectionTie {
                high_team_id: Uuid::parse_str(&tie.high_seed_team_id)?,
                low_team_id: Uuid::parse_str(&tie.low_seed_team_id)?,
                high_seed_number: u32::try_from(tie.high_seed_number)
                    .map_err(|_| ControllerError::InvalidData("invalid high seed".into()))?,
                low_seed_number: u32::try_from(tie.low_seed_number)
                    .map_err(|_| ControllerError::InvalidData("invalid low seed".into()))?,
            })
        })
        .collect::<ControllerResult<Vec<_>>>()?;
    let stages = league
        .stages()
        .iter()
        .cloned()
        .map(|definition| ProjectionStage {
            known_fixtures: if definition.stage_order_index() == 0 {
                known_fixtures.clone()
            } else {
                Vec::new()
            },
            known_ties: if definition.stage_order_index() == 0 {
                known_ties.clone()
            } else {
                Vec::new()
            },
            definition,
        })
        .collect();
    let federation_id = if calibration_scope == CalibrationScope::Federation {
        Some(
            sqlx::query_scalar::<_, String>("SELECT federation_id FROM competitions WHERE id = ?")
                .bind(competition_id.to_string())
                .fetch_one(pool)
                .await?,
        )
    } else {
        None
    };
    let history = if calibration_scope == CalibrationScope::Prior {
        Vec::new()
    } else {
        forecast_calibrations::list_history(
            pool,
            cutoff.year(),
            cutoff.day_of_year(),
            POWER_RANKING_MODEL_VERSION,
            if calibration_scope == CalibrationScope::Competition {
                Some(competition_id)
            } else {
                None
            },
            federation_id.as_deref(),
            2000,
        )
        .await?
    };
    let goal_point_model = fit_goal_point_model(&history);
    let mut config = config;
    (config.knockout_margin_mean, config.knockout_margin_stddev) = fit_knockout_margin(
        &history,
        config.knockout_margin_mean,
        config.knockout_margin_stddev,
    );
    Ok(Some(CompetitionProjectionPlan {
        season_instance_id: season_id,
        teams,
        initial_team_ids,
        stages,
        groups: league.groups().to_vec(),
        external_winners,
        algorithm: league.algorithm(),
        spa_policy: *league.spa_scoring_policy(),
        qta_policy: *league.qta_weighting_policy(),
        tie_break_criteria: league.tie_break_criteria().to_vec(),
        forecast_parameters: parameters,
        goal_point_model,
        config,
    }))
}
