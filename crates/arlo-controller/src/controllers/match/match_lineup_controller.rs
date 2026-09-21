use crate::dto::r#match::{MatchLineupDto, MatchSquadSlotDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_manager_attribute_definitions;
use crate::repositories::formation::formation_cache::get_or_load_formations;
use crate::services::r#match::match_clock_math::{
    calculate_total_elapsed_seconds, calculate_total_match_duration_seconds, format_elapsed_time,
    MatchClockDurationConfig,
};
use crate::services::r#match::minutes_played_calculator::calculate_player_minutes_played;
use arlo_db::models::position_code::position_to_code;
use arlo_domain::{Formation, Player};
use arlo_persistence::models::{MatchRow, MatchSquadSelectionRow, MatchSubstitutionRow};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_match_lineups(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<(MatchLineupDto, MatchLineupDto)> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let substitutions = arlo_persistence::repositories::match_substitutions::list_by_match_id(
        pool,
        match_id,
    )
    .await?;

    build_match_lineups(pool, &match_row, &substitutions).await
}

pub async fn build_match_lineups(
    pool: &SqlitePool,
    match_row: &MatchRow,
    substitutions: &[MatchSubstitutionRow],
) -> ControllerResult<(MatchLineupDto, MatchLineupDto)> {
    let match_uuid = Uuid::parse_str(&match_row.id)?;
    let home_team_uuid = Uuid::parse_str(&match_row.home_team_id)?;
    let away_team_uuid = Uuid::parse_str(&match_row.away_team_id)?;

    let usage_rows =
        arlo_persistence::repositories::match_lineup_usage::list_by_match_id(pool, match_uuid)
            .await?;
    let home_usage = usage_rows
        .iter()
        .find(|u| u.team_id == match_row.home_team_id);
    let away_usage = usage_rows
        .iter()
        .find(|u| u.team_id == match_row.away_team_id);

    let formations = get_or_load_formations(pool).await?;

    let squad_rows =
        arlo_persistence::repositories::match_squad_selection::list_by_match_id(pool, match_uuid)
            .await?;

    let mut player_cache: HashMap<Uuid, Player> = HashMap::new();
    if let Ok(home_players) =
        arlo_db::repositories::player::list_by_team_id(pool, home_team_uuid).await
    {
        for p in home_players {
            player_cache.insert(p.id(), p);
        }
    }
    if let Ok(away_players) =
        arlo_db::repositories::player::list_by_team_id(pool, away_team_uuid).await
    {
        for p in away_players {
            player_cache.insert(p.id(), p);
        }
    }

    for s in &squad_rows {
        if let Ok(pid) = Uuid::parse_str(&s.player_id) {
            if !player_cache.contains_key(&pid) {
                if let Ok(Some(p)) = arlo_db::repositories::player::get_by_id(pool, pid).await {
                    player_cache.insert(pid, p);
                }
            }
        }
    }

    for sub in substitutions {
        if let Ok(pid) = Uuid::parse_str(&sub.player_in_id) {
            if !player_cache.contains_key(&pid) {
                if let Ok(Some(p)) = arlo_db::repositories::player::get_by_id(pool, pid).await {
                    player_cache.insert(pid, p);
                }
            }
        }
        if let Ok(pid) = Uuid::parse_str(&sub.player_out_id) {
            if !player_cache.contains_key(&pid) {
                if let Ok(Some(p)) = arlo_db::repositories::player::get_by_id(pool, pid).await {
                    player_cache.insert(pid, p);
                }
            }
        }
    }

    let scoring_rows =
        arlo_persistence::repositories::match_player_scoring_attempts::list_by_match_id(
            pool, match_uuid,
        )
        .await?;
    let assist_rows =
        arlo_persistence::repositories::match_player_assists::list_by_match_id(pool, match_uuid)
            .await?;
    let physical_rows =
        arlo_persistence::repositories::match_player_physical::list_by_match_id(pool, match_uuid)
            .await?;

    let scoring_map: HashMap<String, _> = scoring_rows
        .into_iter()
        .map(|r| (r.player_id.clone(), r))
        .collect();
    let assist_map: HashMap<String, _> = assist_rows
        .into_iter()
        .map(|r| (r.player_id.clone(), r))
        .collect();
    let physical_map: HashMap<String, f64> = physical_rows
        .into_iter()
        .map(|r| (r.player_id.clone(), r.end_energy_level))
        .collect();

    let manager_defs = get_or_load_manager_attribute_definitions(pool).await?;

    let home_managers =
        arlo_db::repositories::manager::list_by_team_id(pool, home_team_uuid, &manager_defs)
            .await
            .unwrap_or_default();
    let home_manager_name = home_managers
        .into_iter()
        .find(|m| Some(m.id().to_string()) == home_usage.map(|u| u.manager_id.clone()))
        .map(|m| m.person().name().to_string())
        .unwrap_or_else(|| "Treinador Mandante".to_string());

    let away_managers =
        arlo_db::repositories::manager::list_by_team_id(pool, away_team_uuid, &manager_defs)
            .await
            .unwrap_or_default();
    let away_manager_name = away_managers
        .into_iter()
        .find(|m| Some(m.id().to_string()) == away_usage.map(|u| u.manager_id.clone()))
        .map(|m| m.person().name().to_string())
        .unwrap_or_else(|| "Treinador Visitante".to_string());

    let home_team = arlo_db::repositories::team::get_by_id(pool, home_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let away_team = arlo_db::repositories::team::get_by_id(pool, away_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let home_team_name = home_team
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Mandante".to_string());
    let away_team_name = away_team
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Visitante".to_string());

    let clock_config = MatchClockDurationConfig::from_match_row(match_row);
    let total_match_duration = calculate_total_match_duration_seconds(
        &clock_config,
        match_row.final_period.max(1) as u32,
    );

    let home_formation = home_usage
        .and_then(|u| Uuid::parse_str(&u.formation_id).ok())
        .and_then(|fid| formations.iter().find(|f| f.id() == fid).cloned());
    let away_formation = away_usage
        .and_then(|u| Uuid::parse_str(&u.formation_id).ok())
        .and_then(|fid| formations.iter().find(|f| f.id() == fid).cloned());

    let home_lineup = build_team_lineup_dto(
        &match_row.home_team_id,
        home_team_name,
        home_usage.map(|u| u.formation_id.clone()).unwrap_or_default(),
        home_formation.as_ref(),
        home_usage.map(|u| u.manager_id.clone()).unwrap_or_default(),
        home_manager_name,
        &squad_rows,
        substitutions,
        &player_cache,
        &scoring_map,
        &assist_map,
        &physical_map,
        &clock_config,
        total_match_duration,
    );

    let away_lineup = build_team_lineup_dto(
        &match_row.away_team_id,
        away_team_name,
        away_usage.map(|u| u.formation_id.clone()).unwrap_or_default(),
        away_formation.as_ref(),
        away_usage.map(|u| u.manager_id.clone()).unwrap_or_default(),
        away_manager_name,
        &squad_rows,
        substitutions,
        &player_cache,
        &scoring_map,
        &assist_map,
        &physical_map,
        &clock_config,
        total_match_duration,
    );

    Ok((home_lineup, away_lineup))
}

#[allow(clippy::too_many_arguments)]
fn build_team_lineup_dto(
    team_id: &str,
    team_name: String,
    formation_id: String,
    formation: Option<&Formation>,
    manager_id: String,
    manager_name: String,
    all_squad_rows: &[MatchSquadSelectionRow],
    substitutions: &[MatchSubstitutionRow],
    player_cache: &HashMap<Uuid, Player>,
    scoring_map: &HashMap<String, arlo_persistence::models::MatchPlayerScoringAttemptRow>,
    assist_map: &HashMap<String, arlo_persistence::models::MatchPlayerAssistRow>,
    physical_map: &HashMap<String, f64>,
    clock_config: &MatchClockDurationConfig,
    total_match_duration: f64,
) -> MatchLineupDto {
    let formation_name = formation
        .map(|f| f.name().to_string())
        .unwrap_or_else(|| "Formação Padrão".to_string());

    let mut starters = Vec::new();
    let mut bench = Vec::new();

    for squad_row in all_squad_rows {
        if squad_row.team_id != team_id {
            continue;
        }

        let player_uuid = Uuid::parse_str(&squad_row.player_id).unwrap_or_default();
        let player_opt = player_cache.get(&player_uuid);
        let player_name = player_opt
            .map(|p| p.name().to_string())
            .unwrap_or_else(|| "Jogador".to_string());
        let squad_number = player_opt.and_then(|p| p.squad_number());
        let position = player_opt
            .and_then(|p| {
                p.positions()
                    .iter()
                    .max_by_key(|pos| pos.proficiency())
                    .map(|pos| position_to_code(pos.position()).to_string())
            })
            .unwrap_or_else(|| "N/A".to_string());

        let min_result = calculate_player_minutes_played(
            squad_row.was_starter,
            &squad_row.player_id,
            substitutions,
            &squad_row.final_availability_status,
            squad_row.final_suspended_remaining_seconds,
            clock_config,
            total_match_duration,
        );

        let scoring = scoring_map.get(&squad_row.player_id);
        let goal_points_scored = scoring.map(|s| s.goal_points_scored as u32).unwrap_or(0);
        let field_goals_scored = scoring.map(|s| s.field_goals_scored as u32).unwrap_or(0);
        let field_points_scored = scoring.map(|s| s.field_points_scored as u32).unwrap_or(0);

        let goalpoint_assists = assist_map
            .get(&squad_row.player_id)
            .map(|a| a.goalpoint_assists as u32)
            .unwrap_or(0);

        let end_energy_level = physical_map.get(&squad_row.player_id).copied();

        let sub_out = substitutions
            .iter()
            .find(|s| s.player_out_id == squad_row.player_id);
        let sub_in = substitutions
            .iter()
            .find(|s| s.player_in_id == squad_row.player_id);

        let (substituted_by_player_id, substituted_by_player_name) = match sub_out {
            Some(s) => {
                let name = Uuid::parse_str(&s.player_in_id)
                    .ok()
                    .and_then(|id| player_cache.get(&id))
                    .map(|p| p.name().to_string());
                (Some(s.player_in_id.clone()), name)
            }
            None => (None, None),
        };

        let (substituted_in_for_player_id, substituted_in_for_player_name) = match sub_in {
            Some(s) => {
                let name = Uuid::parse_str(&s.player_out_id)
                    .ok()
                    .and_then(|id| player_cache.get(&id))
                    .map(|p| p.name().to_string());
                (Some(s.player_out_id.clone()), name)
            }
            None => (None, None),
        };

        let (substitution_minute, substitution_reason) = if let Some(s) = sub_out {
            let elapsed = calculate_total_elapsed_seconds(
                clock_config,
                s.period.max(0) as u32,
                s.seconds_in_period.max(0.0),
            );
            (Some(format_elapsed_time(elapsed)), Some(s.reason.clone()))
        } else if let Some(s) = sub_in {
            let elapsed = calculate_total_elapsed_seconds(
                clock_config,
                s.period.max(0) as u32,
                s.seconds_in_period.max(0.0),
            );
            (Some(format_elapsed_time(elapsed)), Some(s.reason.clone()))
        } else {
            (None, None)
        };

        let (pitch_length_ratio, pitch_width_ratio) = if squad_row.was_starter {
            squad_row
                .formation_slot_index
                .and_then(|idx| formation.and_then(|f| f.slots().get(idx as usize)))
                .map(|slot| (slot.pitch_length_ratio(), slot.pitch_width_ratio()))
                .unwrap_or((None, None))
        } else {
            (None, None)
        };

        let slot_dto = MatchSquadSlotDto {
            player_id: squad_row.player_id.clone(),
            player_name,
            squad_number,
            position,
            slot_role: squad_row.slot_role.clone(),
            was_starter: squad_row.was_starter,
            was_used: squad_row.was_used,
            formation_slot_index: squad_row.formation_slot_index,
            pitch_length_ratio,
            pitch_width_ratio,
            minutes_played: min_result.minutes_played,
            end_energy_level,
            goal_points_scored,
            field_goals_scored,
            field_points_scored,
            goalpoint_assists,
            substituted_by_player_id,
            substituted_by_player_name,
            substituted_in_for_player_id,
            substituted_in_for_player_name,
            substitution_minute,
            substitution_reason,
            final_availability_status: squad_row.final_availability_status.clone(),
        };

        if squad_row.was_starter {
            starters.push(slot_dto);
        } else {
            bench.push(slot_dto);
        }
    }

    starters.sort_by_key(|s| s.formation_slot_index.unwrap_or(999));
    bench.sort_by_key(|s| (!s.was_used, s.squad_number.unwrap_or(999), s.player_name.clone()));

    MatchLineupDto {
        team_id: team_id.to_string(),
        team_name,
        formation_id,
        formation_name,
        manager_id,
        manager_name,
        starters,
        bench,
    }
}