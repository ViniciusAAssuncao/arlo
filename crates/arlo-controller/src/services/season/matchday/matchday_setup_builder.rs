use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_manager_attribute_definitions;
use crate::repositories::formation::formation_cache::get_or_load_formations;
use crate::services::season::matchday::emergency_roster::ensure_minimum_roster;
use crate::services::season::matchday::matchday_catalog_cache::MatchdayCatalogs;
use crate::services::season::matchday::matchday_referee_selector::select_referees;
use crate::services::season::matchday::team_lineup_resolver::resolve_team_lineup;
use crate::services::season::matchday::team_playbook_resolver::resolve_team_playbook;
use crate::services::season::matchday::team_profile_resolver::resolve_team_instructions;
use crate::services::season::standings::random_tiebreak_resolver::seed_from_uuid;
use arlo_domain::{Formation, MatchFormatRules, Pitch, Player};
use arlo_engine::{MatchSetupParams, TeamSetupParams};
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::persister::MatchPersistenceContext;
use arlo_recovery::availability::resolve_batch_player_statuses;
use arlo_recovery::PlayerCondition;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct PreparedMatchdayFixture {
    pub setup_params: MatchSetupParams,
    pub persistence_context: MatchPersistenceContext,
    pub fixture_row: FixtureRow,
    pub seed: u64,
    pub stage_id: Uuid,
    pub initial_conditions: HashMap<Uuid, PlayerCondition>,
}

async fn filter_available_players(
    pool: &SqlitePool,
    players: Vec<Player>,
) -> ControllerResult<Vec<Player>> {
    if players.is_empty() {
        return Ok(Vec::new());
    }

    let player_ids: Vec<Uuid> = players.iter().map(|p| p.id()).collect();
    let statuses = resolve_batch_player_statuses(pool, &player_ids)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let available = players
        .into_iter()
        .filter(|p| {
            statuses
                .get(&p.id())
                .map(|s| s.is_available_for_selection())
                .unwrap_or(true)
        })
        .collect();

    Ok(available)
}

async fn resolve_and_ensure_available_players(
    pool: &SqlitePool,
    team_id: Uuid,
    fallback_formation: &Formation,
) -> ControllerResult<Vec<Player>> {
    let raw_players = arlo_db::repositories::player::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let existing_count = raw_players.len();
    let mut available = filter_available_players(pool, raw_players).await?;

    if available.len() < 14 {
        ensure_minimum_roster(pool, team_id, fallback_formation, available.len(), existing_count).await?;
        let refreshed = arlo_db::repositories::player::list_by_team_id(pool, team_id)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        available = filter_available_players(pool, refreshed).await?;
    }

    Ok(available)
}

pub async fn build_matchday_setup(
    pool: &SqlitePool,
    fixture: &FixtureRow,
    catalogs: &Arc<MatchdayCatalogs>,
) -> ControllerResult<PreparedMatchdayFixture> {
    let fixture_id = Uuid::parse_str(&fixture.id)?;
    let stage_id = Uuid::parse_str(&fixture.season_stage_id)?;
    let home_team_id = Uuid::parse_str(&fixture.home_team_id)?;
    let away_team_id = Uuid::parse_str(&fixture.away_team_id)?;

    let seed = seed_from_uuid(fixture_id);

    let home_team = arlo_db::repositories::team::get_by_id(pool, home_team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Home team {} not found", home_team_id)))?;

    let _away_team = arlo_db::repositories::team::get_by_id(pool, away_team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Away team {} not found", away_team_id)))?;

    let formations = get_or_load_formations(pool).await?;
    if formations.is_empty() {
        return Err(ControllerError::NotFound(
            "No formations available in database".to_string(),
        ));
    }

    let home_players = resolve_and_ensure_available_players(pool, home_team_id, &formations[0]).await?;
    let away_players = resolve_and_ensure_available_players(pool, away_team_id, &formations[0]).await?;

    let (home_lineup, home_formation) =
        resolve_team_lineup(pool, home_team_id, &home_players, &formations).await?;
    let (away_lineup, away_formation) =
        resolve_team_lineup(pool, away_team_id, &away_players, &formations).await?;

    let mut all_player_ids = Vec::with_capacity(home_players.len() + away_players.len());
    all_player_ids.extend(home_players.iter().map(|p| p.id()));
    all_player_ids.extend(away_players.iter().map(|p| p.id()));

    let initial_conditions = arlo_recovery::orchestration::match_condition_bridge::load_conditions_for_players(
        pool,
        &all_player_ids,
    )
    .await
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let manager_defs = get_or_load_manager_attribute_definitions(pool).await?;

    let home_managers = arlo_db::repositories::manager::list_by_team_id(pool, home_team_id, &manager_defs)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let home_manager = home_managers.into_iter().next().ok_or_else(|| {
        ControllerError::NotFound(format!("Manager for home team {} not found", home_team_id))
    })?;

    let away_managers = arlo_db::repositories::manager::list_by_team_id(pool, away_team_id, &manager_defs)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let away_manager = away_managers.into_iter().next().ok_or_else(|| {
        ControllerError::NotFound(format!("Manager for away team {} not found", away_team_id))
    })?;

    let home_profile = resolve_team_instructions(pool, home_team_id).await?;
    let away_profile = resolve_team_instructions(pool, away_team_id).await?;

    let home_playbook = resolve_team_playbook(pool, home_team_id, home_lineup.id()).await?;
    let away_playbook = resolve_team_playbook(pool, away_team_id, away_lineup.id()).await?;

    let (head_referee, peace_referee) = select_referees(pool, seed).await?;

    let resolved_venue_id = if fixture.is_neutral_venue {
        fixture
            .venue_id
            .as_deref()
            .and_then(|vid| Uuid::parse_str(vid).ok())
    } else {
        home_team.home_venue_id().or_else(|| {
            fixture
                .venue_id
                .as_deref()
                .and_then(|vid| Uuid::parse_str(vid).ok())
        })
    };

    let pitch = if let Some(venue_id) = resolved_venue_id {
        let venue = arlo_db::repositories::venue::get_by_id(pool, venue_id)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

        match venue {
            Some(v) if v.pitch_length_mirim().is_some() && v.pitch_width_mirim().is_some() => {
                Pitch::from_mirim(
                    v.pitch_length_mirim().unwrap(),
                    v.pitch_width_mirim().unwrap(),
                )
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?
            }
            _ => Pitch::from_mirim(145.0, 85.0)
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?,
        }
    } else {
        Pitch::from_mirim(145.0, 85.0)
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?
    };

    let format_rules = MatchFormatRules::default_ruleset();

    let home_team_params = TeamSetupParams::new(
        home_team_id,
        home_lineup.clone(),
        home_formation.clone(),
        home_players,
        home_profile.clone(),
        home_manager,
        vec![home_profile.clone()],
        home_playbook,
    );

    let away_team_params = TeamSetupParams::new(
        away_team_id,
        away_lineup.clone(),
        away_formation.clone(),
        away_players,
        away_profile.clone(),
        away_manager,
        vec![away_profile.clone()],
        away_playbook,
    );

    let setup_params = MatchSetupParams::new(
        home_team_params,
        away_team_params,
        head_referee,
        peace_referee,
        pitch,
        (*catalogs.attribute_keys_by_id).clone(),
        format_rules,
        Arc::clone(&catalogs.fault_catalog),
        Arc::clone(&catalogs.injury_catalog),
        seed.into(),
    );

    let persistence_context = MatchPersistenceContext::new(
        home_lineup.id(),
        away_lineup.id(),
        home_formation.id(),
        away_formation.id(),
        Some(home_profile.id()),
        Some(away_profile.id()),
        resolved_venue_id,
    );

    Ok(PreparedMatchdayFixture {
        setup_params,
        persistence_context,
        fixture_row: fixture.clone(),
        seed,
        stage_id,
        initial_conditions,
    })
}