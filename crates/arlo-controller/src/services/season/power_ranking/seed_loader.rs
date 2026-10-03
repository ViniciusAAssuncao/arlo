use super::prior::calculate_preseason_seeds;
use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use arlo_analytics::{PowerRankingConfig, PowerRating, TeamPowerSeed, POWER_RANKING_MODEL_VERSION};
use arlo_domain::{AttributeKey, AttributeTarget};
use arlo_persistence::repositories::season::{power_rankings, season_instances};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub async fn load_preseason_seeds(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    as_of: CalendarDate,
    config: &PowerRankingConfig,
) -> ControllerResult<Vec<TeamPowerSeed>> {
    config.validate()?;
    let season = season_instances::get_by_id(pool, season_instance_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!("season instance {season_instance_id} not found"))
        })?;
    let team_ids = power_rankings::list_seed_team_ids(pool, season_instance_id)
        .await?
        .into_iter()
        .map(|id| Uuid::parse_str(&id).map_err(ControllerError::from))
        .collect::<ControllerResult<Vec<_>>>()?;
    if team_ids.is_empty() {
        return Err(ControllerError::InvalidData(format!(
            "season instance {season_instance_id} has no scheduled teams"
        )));
    }
    if let Some(snapshot) =
        power_rankings::get_latest(pool, season_instance_id, POWER_RANKING_MODEL_VERSION).await?
    {
        return load_published_seeds(pool, &team_ids, &snapshot.id).await;
    }
    let previous_rows = power_rankings::list_previous_ratings(
        pool,
        season.reference_year,
        as_of.year(),
        as_of.day_of_year(),
        POWER_RANKING_MODEL_VERSION,
    )
    .await?;
    let current_teams: HashSet<Uuid> = team_ids.iter().copied().collect();
    let mut previous_ratings = HashMap::with_capacity(previous_rows.len());
    for (team_id, rating) in previous_rows {
        let team_id = Uuid::parse_str(&team_id)?;
        if !current_teams.contains(&team_id) {
            continue;
        }
        if previous_ratings
            .insert(team_id, PowerRating::new(rating)?)
            .is_some()
        {
            return Err(ControllerError::InvalidData(format!(
                "duplicate historical rating for team {team_id}"
            )));
        }
    }
    if team_ids
        .iter()
        .all(|team_id| previous_ratings.contains_key(team_id))
    {
        return calculate_preseason_seeds(
            &team_ids,
            &[],
            &HashMap::new(),
            &previous_ratings,
            config,
        );
    }
    let definitions = arlo_db::repositories::attribute_definition::list_by_applies_to(
        pool,
        AttributeTarget::Player,
    )
    .await
    .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
    let keys: HashMap<Uuid, AttributeKey> = definitions
        .into_iter()
        .map(|definition| (definition.id(), definition.key()))
        .collect();
    let players = arlo_db::repositories::player::list_all_with_team(pool)
        .await
        .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
    calculate_preseason_seeds(&team_ids, &players, &keys, &previous_ratings, config)
}

async fn load_published_seeds(
    pool: &SqlitePool,
    team_ids: &[Uuid],
    snapshot_id: &str,
) -> ControllerResult<Vec<TeamPowerSeed>> {
    let rows = power_rankings::list_entries(pool, Uuid::parse_str(snapshot_id)?).await?;
    let expected: HashSet<Uuid> = team_ids.iter().copied().collect();
    if rows.len() != expected.len() {
        return Err(ControllerError::InvalidData(
            "published power ranking seeds differ from season teams".into(),
        ));
    }
    let mut seeds = Vec::with_capacity(rows.len());
    let mut seen = HashSet::new();
    for row in rows {
        let team_id = Uuid::parse_str(&row.team_id)?;
        if !expected.contains(&team_id) || !seen.insert(team_id) {
            return Err(ControllerError::InvalidData(format!(
                "invalid published power ranking seed for team {team_id}"
            )));
        }
        seeds.push(TeamPowerSeed::new(
            team_id,
            PowerRating::new(row.initial_rating)?,
        ));
    }
    seeds.sort_by_key(|seed| seed.team_id());
    Ok(seeds)
}
