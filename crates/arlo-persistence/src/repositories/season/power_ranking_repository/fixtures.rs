use crate::error::PersistenceResult;
use crate::models::season::PowerRankingFixtureRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_completed_fixtures(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    through_year: i64,
    through_day_of_year: u32,
) -> PersistenceResult<Vec<PowerRankingFixtureRow>> {
    let rows = sqlx::query_as::<_, PowerRankingFixtureRow>(
        r#"SELECT
            f.id AS fixture_id,
            f.scheduled_year AS played_year,
            f.scheduled_day_of_year AS played_day_of_year,
            f.home_team_id,
            f.away_team_id,
            f.home_score,
            f.away_score,
            f.is_neutral_venue AS neutral_venue
        FROM fixtures f
        JOIN season_stages s ON s.id = f.season_stage_id
        WHERE s.season_instance_id = ?
          AND f.status = 'Completed'
          AND (f.scheduled_year < ? OR
               (f.scheduled_year = ? AND f.scheduled_day_of_year <= ?))
        ORDER BY f.scheduled_year, f.scheduled_day_of_year, f.id"#,
    )
    .bind(season_instance_id.to_string())
    .bind(through_year)
    .bind(through_year)
    .bind(through_day_of_year as i64)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
