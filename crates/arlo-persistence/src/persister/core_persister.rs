use crate::error::PersistenceResult;
use crate::models::{MatchLineupUsageRow, MatchRow, MatchTeamScoreRow};
use crate::persister::assigned_referees::assigned_referees;
use crate::persister::match_persistence_context::MatchPersistenceContext;
use crate::repositories;
use arlo_engine::{MatchInput, MatchState};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_match_core(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    input: &MatchInput,
    state: &MatchState,
    context: &MatchPersistenceContext,
) -> PersistenceResult<()> {
    let completed_at = context.completed_at_unix_seconds.unwrap_or(0);
    let created_at = context.created_at_unix_seconds.unwrap_or(0);
    let fixture_id = context
        .completed_fixture
        .as_ref()
        .and_then(|f| Uuid::parse_str(&f.id).ok());
    let (head_referee, peace_referee) = assigned_referees(input)?;
    let format = input.format();
    let pitch = input.pitch();
    let match_row = MatchRow::new(
        match_id,
        fixture_id,
        input.home().team_id(),
        input.away().team_id(),
        context.venue_id,
        pitch.length_mirim(),
        pitch.width_mirim(),
        input.seed(),
        format.regulation_periods(),
        format.regulation_period_duration_minutes() * 60,
        format.allows_overtime(),
        format.overtime_periods(),
        format.overtime_period_duration_minutes() * 60,
        head_referee.id(),
        peace_referee.id(),
        state.clock().period(),
        state.clock().period() > format.regulation_periods(),
        completed_at,
        created_at,
    );
    repositories::match_repo::insert(tx, &match_row).await?;

    let home_score_row = MatchTeamScoreRow::new(
        Uuid::new_v4(),
        match_id,
        input.home().team_id(),
        true,
        state.home().score().goal_points(),
        state.home().score().field_goals(),
        state.home().score().field_points(),
        state.home().score().total_points(),
    );
    let away_score_row = MatchTeamScoreRow::new(
        Uuid::new_v4(),
        match_id,
        input.away().team_id(),
        false,
        state.away().score().goal_points(),
        state.away().score().field_goals(),
        state.away().score().field_points(),
        state.away().score().total_points(),
    );
    repositories::match_team_score::insert_batch(tx, &[home_score_row, away_score_row]).await?;

    let home_lineup_usage = MatchLineupUsageRow::new(
        Uuid::new_v4(),
        match_id,
        input.home().team_id(),
        context.home_tactical_lineup_id,
        context.home_formation_id,
        context.home_team_tactical_profile_id,
        input.home().manager().id(),
    );
    let away_lineup_usage = MatchLineupUsageRow::new(
        Uuid::new_v4(),
        match_id,
        input.away().team_id(),
        context.away_tactical_lineup_id,
        context.away_formation_id,
        context.away_team_tactical_profile_id,
        input.away().manager().id(),
    );
    repositories::match_lineup_usage::insert_batch(tx, &[home_lineup_usage, away_lineup_usage])
        .await?;

    Ok(())
}
