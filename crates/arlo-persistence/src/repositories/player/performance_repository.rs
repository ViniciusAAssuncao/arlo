use crate::error::PersistenceResult;
use crate::models::MatchPlayerPerformanceRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{FromRow, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct PlayerSeasonPerformanceAggregateRow {
    pub matches_rated: i64,
    pub average_rating: f64,
    pub average_performance_rating: f64,
    pub average_confidence: f64,
    pub highest_rating: f64,
    pub lowest_rating: f64,
    pub average_execution: f64,
    pub average_production: f64,
    pub average_defense: f64,
    pub average_ball_security: f64,
    pub average_discipline: f64,
    pub average_high_impact: f64,
}

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
    "effective_opportunity_weight",
    "offensive_latent",
    "defensive_latent",
    "raw_latent",
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
        builder.push_bind(row.effective_opportunity_weight);
        builder.push_bind(row.offensive_latent);
        builder.push_bind(row.defensive_latent);
        builder.push_bind(row.raw_latent);
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
            effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            raw_latent,
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
            effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            raw_latent,
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

pub async fn get_player_season_performance(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerSeasonPerformanceAggregateRow> {
    let row = sqlx::query_as::<_, PlayerSeasonPerformanceAggregateRow>(
        r#"SELECT
            COUNT(*) AS matches_rated,
            COALESCE(AVG(perf.final_rating), 0.0) AS average_rating,
            COALESCE(AVG(perf.performance_rating), 0.0) AS average_performance_rating,
            COALESCE(AVG(perf.confidence), 0.0) AS average_confidence,
            COALESCE(MAX(perf.final_rating), 0.0) AS highest_rating,
            COALESCE(MIN(perf.final_rating), 0.0) AS lowest_rating,
            COALESCE(AVG(perf.execution_score), 0.0) AS average_execution,
            COALESCE(AVG(perf.production_score), 0.0) AS average_production,
            COALESCE(AVG(perf.defense_score), 0.0) AS average_defense,
            COALESCE(AVG(perf.ball_security_score), 0.0) AS average_ball_security,
            COALESCE(AVG(perf.discipline_score), 0.0) AS average_discipline,
            COALESCE(AVG(perf.high_impact_score), 0.0) AS average_high_impact
        FROM match_player_performance perf
        INNER JOIN matches m ON m.id = perf.match_id
        INNER JOIN fixtures f ON f.id = m.fixture_id
        INNER JOIN season_stages ss ON ss.id = f.season_stage_id
        WHERE perf.player_id = ?
          AND ss.season_instance_id = ?
          AND perf.model_version = (
              SELECT MAX(perf_latest.model_version)
              FROM match_player_performance perf_latest
              INNER JOIN matches m_latest ON m_latest.id = perf_latest.match_id
              INNER JOIN fixtures f_latest ON f_latest.id = m_latest.fixture_id
              INNER JOIN season_stages ss_latest ON ss_latest.id = f_latest.season_stage_id
              WHERE perf_latest.player_id = ?
                AND ss_latest.season_instance_id = ?
          )"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
