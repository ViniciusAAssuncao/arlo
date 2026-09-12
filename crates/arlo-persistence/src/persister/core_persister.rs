use crate::error::PersistenceResult;
use crate::models::{MatchLineupUsageRow, MatchRow, MatchTeamScoreRow};
use crate::persister::match_persistence_context::MatchPersistenceContext;
use crate::repositories;
use arlo_engine::MatchState;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_match_core(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    state: &MatchState,
    context: &MatchPersistenceContext,
) -> PersistenceResult<()> {
    let completed_at = context.completed_at_unix_seconds.unwrap_or(0);
    let created_at = context.created_at_unix_seconds.unwrap_or(0);
    let match_row = MatchRow::new(
        match_id,
        state.home_team_id(),
        state.away_team_id(),
        context.venue_id,
        state.pitch().length_mirim(),
        state.pitch().width_mirim(),
        state.rng_provider().seed().value(),
        state.format_rules().regulation_periods(),
        state.format_rules().regulation_period_duration_minutes() * 60,
        state.format_rules().allows_overtime(),
        state.format_rules().overtime_periods(),
        state.format_rules().overtime_period_duration_minutes() * 60,
        state.head_referee().id(),
        state.peace_referee().id(),
        state.clock().period(),
        state.clock().period() > state.format_rules().regulation_periods(),
        completed_at,
        created_at,
    );
    repositories::match_repo::insert(tx, &match_row).await?;

    let home_score_row = MatchTeamScoreRow::new(
        Uuid::new_v4(),
        match_id,
        state.home_team_id(),
        true,
        state.home_score().goal_points,
        state.home_score().field_goals,
        state.home_score().field_points,
        state.home_score().total_points,
    );
    let away_score_row = MatchTeamScoreRow::new(
        Uuid::new_v4(),
        match_id,
        state.away_team_id(),
        false,
        state.away_score().goal_points,
        state.away_score().field_goals,
        state.away_score().field_points,
        state.away_score().total_points,
    );
    repositories::match_team_score::insert_batch(tx, &[home_score_row, away_score_row]).await?;

    let home_lineup_usage = MatchLineupUsageRow::new(
        Uuid::new_v4(),
        match_id,
        state.home_team_id(),
        context.home_tactical_lineup_id,
        context.home_formation_id,
        context.home_team_tactical_profile_id,
        state.home_manager().id(),
    );
    let away_lineup_usage = MatchLineupUsageRow::new(
        Uuid::new_v4(),
        match_id,
        state.away_team_id(),
        context.away_tactical_lineup_id,
        context.away_formation_id,
        context.away_team_tactical_profile_id,
        state.away_manager().id(),
    );
    repositories::match_lineup_usage::insert_batch(tx, &[home_lineup_usage, away_lineup_usage])
        .await?;

    Ok(())
}
