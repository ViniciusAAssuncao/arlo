use crate::domain::season::{FixtureStatus, SeasonStageInstance, StageStatus};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache;
use crate::services::event_scheduling::handlers::stage_transition_handler::handle_stage_transition;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::active_season_resolver::resolve_active_season;
use crate::services::season::persistence::{
    activate_stage, advance_season_stage, load_stage_knockout_ties, map_row_to_fixture,
    mark_stage_completed,
};
use crate::services::season::progression::knockout_round_advancer::advance_knockout_round;
use crate::services::season::progression::season_finalizer::finalize_season;
use crate::services::season::progression::stage_completion_detector::is_stage_complete;
use crate::services::season::stage::knockout_bracket_progress_detector::{
    detect_knockout_bracket_progress, KnockoutBracketProgress,
};
use crate::services::season::stage::stage_schedule_generator::GeneratedStageSchedule;
use arlo_domain::StageType;
use sqlx::SqlitePool;
use std::collections::BTreeSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum ProgressionOutcome {
    NoActiveSeason,
    StageNotComplete {
        stage_order_index: u32,
    },
    KnockoutRoundAdvanced {
        stage_order_index: u32,
        next_round_index: u32,
        schedule: GeneratedStageSchedule,
    },
    TransitionedToNextStage {
        next_stage_order_index: u32,
        schedule: GeneratedStageSchedule,
    },
    SeasonFinalized {
        season_instance_id: Uuid,
    },
}

pub async fn progress_season(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    trigger_store: Arc<PendingTriggerStore>,
    reference_year: i64,
) -> ControllerResult<ProgressionOutcome> {
    let config_arc = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let active_season = match resolve_active_season(pool, competition_id).await? {
        Some(s) => s,
        None => return Ok(ProgressionOutcome::NoActiveSeason),
    };

    let season_instance_id = Uuid::parse_str(&active_season.id)?;
    let current_stage_order_index = active_season.current_stage_order_index as u32;

    let stages = arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
        pool,
        season_instance_id,
    )
    .await?;

    let current_stage_row = match stages
        .iter()
        .find(|s| s.stage_order_index == current_stage_order_index as i32)
    {
        Some(s) => s,
        None => {
            return Err(ControllerError::NotFound(format!(
                "Stage instance with order_index {} not found for season {}",
                current_stage_order_index, season_instance_id
            )))
        }
    };

    let current_stage_id = Uuid::parse_str(&current_stage_row.id)?;
    let stage_type =
        arlo_db::models::league_calendar::parse_stage_type(&current_stage_row.stage_type)
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let stage_status = match current_stage_row.status.as_str() {
        "Pending" => StageStatus::Pending,
        "Active" => StageStatus::Active,
        "Completed" => StageStatus::Completed,
        _ => StageStatus::Pending,
    };

    let current_stage_instance = SeasonStageInstance::new(
        current_stage_id,
        season_instance_id,
        current_stage_order_index,
        stage_type,
        stage_status,
    );

    let stage_def = config_arc
        .stages()
        .iter()
        .find(|s| s.stage_order_index() == current_stage_order_index)
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Stage definition with order_index {} not found in config",
                current_stage_order_index
            ))
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let fixture_rows =
        arlo_persistence::repositories::season::fixtures::list_by_stage_id(pool, current_stage_id)
            .await?;

    let mut domain_fixtures = Vec::with_capacity(fixture_rows.len());
    for row in &fixture_rows {
        domain_fixtures.push(map_row_to_fixture(row)?);
    }

    let mut domain_ties = load_stage_knockout_ties(pool, current_stage_id).await?;

    if stage_type == StageType::KnockoutBracket {
        if let Some(schedule) = advance_knockout_round(
            pool,
            competition_id,
            calendar_system_id,
            season_instance_id,
            current_stage_id,
            stage_def,
            &config_arc,
            calendar,
            Arc::clone(&trigger_store),
            reference_year,
            &domain_fixtures,
            domain_ties.clone(),
        )
        .await?
        {
            let next_round_index = schedule
                .fixtures
                .iter()
                .map(|f| f.round_index())
                .min()
                .unwrap_or(0);

            return Ok(ProgressionOutcome::KnockoutRoundAdvanced {
                stage_order_index: current_stage_order_index,
                next_round_index,
                schedule,
            });
        }
        
        domain_ties = load_stage_knockout_ties(pool, current_stage_id).await?;
    }

    if !is_stage_complete(
        &current_stage_instance,
        &domain_fixtures,
        &domain_ties,
        config_arc.tie_break_criteria(),
    ) {
        let latest_pending = domain_fixtures.iter()
            .filter(|f| f.status() == FixtureStatus::Scheduled || f.status() == FixtureStatus::Postponed)
            .map(|f| f.scheduled_date())
            .max();

        if let Some(latest_date) = latest_pending {
            let next_check_date = crate::services::calendar::date_advancer::advance(calendar, &latest_date, 1);
            trigger_store.insert(crate::domain::event_scheduling::PendingTrigger::new(
                next_check_date,
                competition_id,
                crate::domain::event_scheduling::TriggerKind::StageTransitionCheckDue,
            )).await;
        }

        return Ok(ProgressionOutcome::StageNotComplete {
            stage_order_index: current_stage_order_index,
        });
    }

    mark_stage_completed(pool, current_stage_id).await?;

    let next_stage_order_index = current_stage_order_index + 1;
    let next_stage_exists = config_arc
        .stages()
        .iter()
        .any(|s| s.stage_order_index() == next_stage_order_index);

    if next_stage_exists {
        let mut team_ids_set = BTreeSet::new();
        for f in &domain_fixtures {
            team_ids_set.insert(f.home_team_id());
            team_ids_set.insert(f.away_team_id());
        }
        let participating_team_ids: Vec<Uuid> = team_ids_set.into_iter().collect();

        let start_round_index = domain_fixtures
            .iter()
            .map(|f| f.round_index())
            .max()
            .map(|m| m + 1)
            .unwrap_or(0);

        let new_stage_instance_id = Uuid::new_v4();

        let schedule = handle_stage_transition(
            pool,
            competition_id,
            calendar_system_id,
            season_instance_id,
            new_stage_instance_id,
            next_stage_order_index,
            &domain_fixtures,
            &participating_team_ids,
            reference_year,
            start_round_index,
            trigger_store,
        )
        .await?;

        activate_stage(pool, new_stage_instance_id).await?;

        advance_season_stage(pool, season_instance_id, next_stage_order_index).await?;

        Ok(ProgressionOutcome::TransitionedToNextStage {
            next_stage_order_index,
            schedule,
        })
    } else {
        let champion = if stage_type == StageType::KnockoutBracket {
            match detect_knockout_bracket_progress(
                &domain_ties,
                &domain_fixtures,
                config_arc.tie_break_criteria(),
            ) {
                KnockoutBracketProgress::BracketComplete { champion_team_id } => {
                    Some(champion_team_id)
                }
                _ => None,
            }
        } else {
            let standings = standings_cache::get_or_compute_standings_with_fixtures(
                pool,
                current_stage_id,
                Some(&domain_fixtures),
            )
            .await?;
            standings.first().map(|e| e.team_id())
        };

        finalize_season(
            pool,
            competition_id,
            season_instance_id,
            champion,
            calendar,
            trigger_store,
        )
        .await?;
        Ok(ProgressionOutcome::SeasonFinalized { season_instance_id })
    }
}