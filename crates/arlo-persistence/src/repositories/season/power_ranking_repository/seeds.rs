use crate::error::PersistenceResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_seed_team_ids(
    pool: &SqlitePool,
    season_instance_id: Uuid,
) -> PersistenceResult<Vec<String>> {
    let rows = sqlx::query_scalar::<_, String>(
        r#"WITH season_fixtures AS (
            SELECT f.home_team_id, f.away_team_id
            FROM fixtures f
            JOIN season_stages s ON s.id = f.season_stage_id
            WHERE s.season_instance_id = ?
        )
        SELECT home_team_id AS team_id FROM season_fixtures
        UNION
        SELECT away_team_id AS team_id FROM season_fixtures
        ORDER BY team_id"#,
    )
    .bind(season_instance_id.to_string())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_previous_ratings(
    pool: &SqlitePool,
    before_reference_year: i64,
    before_year: i64,
    before_day_of_year: u32,
    model_version: u32,
) -> PersistenceResult<Vec<(String, f64)>> {
    let rows = sqlx::query_as::<_, (String, f64)>(
        r#"SELECT team_id, rating FROM (
            SELECT e.team_id, e.rating,
                ROW_NUMBER() OVER (
                    PARTITION BY e.team_id
                    ORDER BY s.year DESC, s.day_of_year DESC,
                             si.reference_year DESC, s.id DESC
                ) AS row_number
            FROM power_ranking_entries e
            JOIN power_ranking_snapshots s ON s.id = e.snapshot_id
            JOIN season_instances si ON si.id = s.season_instance_id
            WHERE si.reference_year < ?
              AND si.status = 'Completed'
              AND s.model_version = ?
              AND (s.year < ? OR (s.year = ? AND s.day_of_year < ?))
        ) WHERE row_number = 1"#,
    )
    .bind(before_reference_year)
    .bind(model_version as i64)
    .bind(before_year)
    .bind(before_year)
    .bind(before_day_of_year as i64)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
