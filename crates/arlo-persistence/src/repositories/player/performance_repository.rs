use crate::error::PersistenceResult;
use crate::models::MatchPlayerPerformanceRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "team_id",
    "performance_rating",
    "outcome_adjustment",
    "final_rating",
    "confidence",
    "seconds_played",
    "effective_opportunities",
    "offensive_position",
    "defensive_position",
    "slot_role",
    "execution_score",
    "production_score",
    "defense_score",
    "ball_security_score",
    "discipline_score",
    "high_impact_score",
    "model_version",
];

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerPerformanceRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_player_performance", COLUMNS, rows, |builder, row| {
        builder.push_bind(&row.id);
        builder.push_bind(&row.match_id);
        builder.push_bind(&row.player_id);
        builder.push_bind(&row.team_id);
        builder.push_bind(row.performance_rating);
        builder.push_bind(row.outcome_adjustment);
        builder.push_bind(row.final_rating);
        builder.push_bind(row.confidence);
        builder.push_bind(row.seconds_played);
        builder.push_bind(row.effective_opportunities);
        builder.push_bind(&row.offensive_position);
        builder.push_bind(&row.defensive_position);
        builder.push_bind(&row.slot_role);
        builder.push_bind(row.execution_score);
        builder.push_bind(row.production_score);
        builder.push_bind(row.defense_score);
        builder.push_bind(row.ball_security_score);
        builder.push_bind(row.discipline_score);
        builder.push_bind(row.high_impact_score);
        builder.push_bind(row.model_version);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerPerformanceRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPerformanceRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            team_id,
            performance_rating,
            outcome_adjustment,
            final_rating,
            confidence,
            seconds_played,
            effective_opportunities,
            offensive_position,
            defensive_position,
            slot_role,
            execution_score,
            production_score,
            defense_score,
            ball_security_score,
            discipline_score,
            high_impact_score,
            model_version
        FROM match_player_performance
        WHERE match_id = ?
        ORDER BY team_id ASC, player_id ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerPerformanceRow>> {
    let row = sqlx::query_as::<_, MatchPlayerPerformanceRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            team_id,
            performance_rating,
            outcome_adjustment,
            final_rating,
            confidence,
            seconds_played,
            effective_opportunities,
            offensive_position,
            defensive_position,
            slot_role,
            execution_score,
            production_score,
            defense_score,
            ball_security_score,
            discipline_score,
            high_impact_score,
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
