use crate::error::ControllerResult;
use crate::repositories::performance::models::{
    MatchPlayerPerformanceRow, PlayerSeasonPerformanceSummaryRow,
};
use arlo_analytics::performance::rating::PlayerPerformanceSnapshot;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert_batch_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    snapshots: &[PlayerPerformanceSnapshot],
) -> ControllerResult<()> {
    for snapshot in snapshots {
        let id = Uuid::new_v4().to_string();
        let version = format!(
            "{}.{}.{}",
            snapshot.model_version().major(),
            snapshot.model_version().minor(),
            snapshot.model_version().patch()
        );
        sqlx::query(
            r#"INSERT INTO match_player_performance (
                id, match_id, player_id, team_id,
                offensive_position, defensive_position, slot_role,
                performance_rating, outcome_adjustment, final_rating, confidence,
                seconds_played, effective_opportunities,
                execution, production, defense, ball_security, discipline, high_impact,
                model_version
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id)
        .bind(match_id.to_string())
        .bind(snapshot.player_id().to_string())
        .bind(snapshot.team_id().to_string())
        .bind(format!("{:?}", snapshot.offensive_position()))
        .bind(format!("{:?}", snapshot.defensive_position()))
        .bind(format!("{:?}", snapshot.slot_role()))
        .bind(snapshot.performance_rating().value())
        .bind(snapshot.outcome_adjustment())
        .bind(snapshot.final_rating().value())
        .bind(snapshot.confidence().value())
        .bind(snapshot.seconds_played())
        .bind(snapshot.effective_opportunities() as i32)
        .bind(snapshot.breakdown().execution())
        .bind(snapshot.breakdown().production())
        .bind(snapshot.breakdown().defense())
        .bind(snapshot.breakdown().ball_security())
        .bind(snapshot.breakdown().discipline())
        .bind(snapshot.breakdown().high_impact())
        .bind(version)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

pub async fn insert_batch(
    pool: &SqlitePool,
    match_id: Uuid,
    snapshots: &[PlayerPerformanceSnapshot],
) -> ControllerResult<()> {
    let mut tx = pool.begin().await?;
    insert_batch_with_tx(&mut tx, match_id, snapshots).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<Vec<MatchPlayerPerformanceRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPerformanceRow>(
        r#"SELECT
            id, match_id, player_id, team_id,
            offensive_position, defensive_position, slot_role,
            performance_rating, outcome_adjustment, final_rating, confidence,
            seconds_played, effective_opportunities,
            execution, production, defense, ball_security, discipline, high_impact,
            model_version
        FROM match_player_performance
        WHERE match_id = ?
        ORDER BY final_rating DESC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_by_match_and_player(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<MatchPlayerPerformanceRow>> {
    let row = sqlx::query_as::<_, MatchPlayerPerformanceRow>(
        r#"SELECT
            id, match_id, player_id, team_id,
            offensive_position, defensive_position, slot_role,
            performance_rating, outcome_adjustment, final_rating, confidence,
            seconds_played, effective_opportunities,
            execution, production, defense, ball_security, discipline, high_impact,
            model_version
        FROM match_player_performance
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<Vec<MatchPlayerPerformanceRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPerformanceRow>(
        r#"SELECT
            id, match_id, player_id, team_id,
            offensive_position, defensive_position, slot_role,
            performance_rating, outcome_adjustment, final_rating, confidence,
            seconds_played, effective_opportunities,
            execution, production, defense, ball_security, discipline, high_impact,
            model_version
        FROM match_player_performance
        WHERE player_id = ?
        ORDER BY rowid DESC"#,
    )
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_season_performance_summary(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<Option<PlayerSeasonPerformanceSummaryRow>> {
    let row = sqlx::query_as::<_, PlayerSeasonPerformanceSummaryRow>(
        r#"SELECT
            COUNT(*) as matches_rated,
            COALESCE(AVG(p.final_rating), 5.5) as average_rating,
            COALESCE(AVG(p.performance_rating), 5.5) as average_performance_rating,
            COALESCE(AVG(p.confidence), 0.0) as average_confidence,
            COALESCE(MAX(p.final_rating), 5.5) as highest_rating,
            COALESCE(MIN(p.final_rating), 5.5) as lowest_rating,
            COALESCE(AVG(p.execution), 0.0) as avg_execution,
            COALESCE(AVG(p.production), 0.0) as avg_production,
            COALESCE(AVG(p.defense), 0.0) as avg_defense,
            COALESCE(AVG(p.ball_security), 0.0) as avg_ball_security,
            COALESCE(AVG(p.discipline), 0.0) as avg_discipline,
            COALESCE(AVG(p.high_impact), 0.0) as avg_high_impact
        FROM match_player_performance p
        JOIN matches m ON m.id = p.match_id
        JOIN fixtures f ON f.id = m.fixture_id
        JOIN season_stages s ON s.id = f.season_stage_id
        WHERE p.player_id = ? AND s.season_instance_id = ?
        HAVING COUNT(*) > 0"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}