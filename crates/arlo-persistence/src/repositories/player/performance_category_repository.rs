use crate::error::PersistenceResult;
use crate::models::MatchPlayerPerformanceCategoryRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "team_id",
    "category",
    "latent_contribution",
    "observations",
    "opportunity_weight",
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
    rows: &[MatchPlayerPerformanceCategoryRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_player_performance_category_contributions",
        COLUMNS,
        rows,
        |builder, row| {
            builder.push_bind(&row.id);
            builder.push_bind(&row.match_id);
            builder.push_bind(&row.player_id);
            builder.push_bind(&row.team_id);
            builder.push_bind(&row.category);
            builder.push_bind(row.latent_contribution);
            builder.push_bind(row.observations);
            builder.push_bind(row.opportunity_weight);
            builder.push_bind(row.execution_score);
            builder.push_bind(row.production_score);
            builder.push_bind(row.defense_score);
            builder.push_bind(row.ball_security_score);
            builder.push_bind(row.discipline_score);
            builder.push_bind(row.high_impact_score);
            builder.push_bind(row.model_version);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerPerformanceCategoryRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPerformanceCategoryRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            team_id,
            category,
            latent_contribution,
            observations,
            opportunity_weight,
            execution_score,
            production_score,
            defense_score,
            ball_security_score,
            discipline_score,
            high_impact_score,
            model_version
        FROM match_player_performance_category_contributions
        WHERE match_id = ?
        ORDER BY player_id ASC, category ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerPerformanceCategoryRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPerformanceCategoryRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            team_id,
            category,
            latent_contribution,
            observations,
            opportunity_weight,
            execution_score,
            production_score,
            defense_score,
            ball_security_score,
            discipline_score,
            high_impact_score,
            model_version
        FROM match_player_performance_category_contributions
        WHERE match_id = ? AND player_id = ?
        ORDER BY category ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
