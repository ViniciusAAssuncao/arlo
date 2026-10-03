use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ForecastHistoryRow {
    pub competition_id: String,
    pub federation_id: String,
    pub home_rating: f64,
    pub away_rating: f64,
    pub neutral_venue: bool,
    pub home_score: i64,
    pub away_score: i64,
    pub home_goal_points: Option<i64>,
    pub away_goal_points: Option<i64>,
}

pub async fn first_fixture_date(
    pool: &SqlitePool,
    season_id: Uuid,
) -> PersistenceResult<Option<(i64, i64)>> {
    Ok(sqlx::query_as::<_, (i64, i64)>(
        "SELECT f.scheduled_year, f.scheduled_day_of_year FROM fixtures f JOIN season_stages s ON s.id = f.season_stage_id WHERE s.season_instance_id = ? ORDER BY f.scheduled_year, f.scheduled_day_of_year LIMIT 1",
    )
    .bind(season_id.to_string())
    .fetch_optional(pool)
    .await?)
}

pub async fn latest_pair_ratings(
    pool: &SqlitePool,
    season_id: Uuid,
    home_team_id: Uuid,
    away_team_id: Uuid,
    year: i64,
    day: u32,
    version: u32,
) -> PersistenceResult<Option<(String, f64, f64)>> {
    Ok(sqlx::query_as::<_, (String, f64, f64)>(
        "SELECT p.id, h.rating, a.rating FROM power_ranking_snapshots p JOIN power_ranking_entries h ON h.snapshot_id = p.id AND h.team_id = ? JOIN power_ranking_entries a ON a.snapshot_id = p.id AND a.team_id = ? WHERE p.season_instance_id = ? AND p.model_version = ? AND (p.year < ? OR (p.year = ? AND p.day_of_year <= ?)) ORDER BY p.year DESC, p.day_of_year DESC LIMIT 1",
    )
    .bind(home_team_id.to_string()).bind(away_team_id.to_string())
    .bind(season_id.to_string()).bind(version as i64).bind(year).bind(year).bind(day as i64)
    .fetch_optional(pool).await?)
}

pub async fn list_history(
    pool: &SqlitePool,
    cutoff_year: i64,
    cutoff_day: u32,
    version: u32,
    competition_id: Option<Uuid>,
    federation_id: Option<&str>,
    limit: u32,
) -> PersistenceResult<Vec<ForecastHistoryRow>> {
    Ok(sqlx::query_as::<_, ForecastHistoryRow>(
        r#"SELECT si.competition_id, c.federation_id,
            COALESCE((SELECT e.rating FROM power_ranking_entries e
                JOIN power_ranking_snapshots p ON p.id = e.snapshot_id
                WHERE p.season_instance_id = si.id AND p.model_version = ?
                  AND e.team_id = f.home_team_id
                  AND (p.year < f.scheduled_year OR (p.year = f.scheduled_year AND p.day_of_year < f.scheduled_day_of_year))
                ORDER BY p.year DESC, p.day_of_year DESC LIMIT 1),
                (SELECT e.initial_rating FROM power_ranking_entries e
                JOIN power_ranking_snapshots p ON p.id = e.snapshot_id
                WHERE p.season_instance_id = si.id AND p.model_version = ? AND e.team_id = f.home_team_id
                ORDER BY p.year, p.day_of_year LIMIT 1)) AS home_rating,
            COALESCE((SELECT e.rating FROM power_ranking_entries e
                JOIN power_ranking_snapshots p ON p.id = e.snapshot_id
                WHERE p.season_instance_id = si.id AND p.model_version = ?
                  AND e.team_id = f.away_team_id
                  AND (p.year < f.scheduled_year OR (p.year = f.scheduled_year AND p.day_of_year < f.scheduled_day_of_year))
                ORDER BY p.year DESC, p.day_of_year DESC LIMIT 1),
                (SELECT e.initial_rating FROM power_ranking_entries e
                JOIN power_ranking_snapshots p ON p.id = e.snapshot_id
                WHERE p.season_instance_id = si.id AND p.model_version = ? AND e.team_id = f.away_team_id
                ORDER BY p.year, p.day_of_year LIMIT 1)) AS away_rating,
            f.is_neutral_venue AS neutral_venue, f.home_score, f.away_score,
            f.home_goal_points, f.away_goal_points
        FROM fixtures f
        JOIN season_stages s ON s.id = f.season_stage_id
        JOIN season_instances si ON si.id = s.season_instance_id
        JOIN competitions c ON c.id = si.competition_id
        WHERE f.status = 'Completed' AND f.home_score IS NOT NULL AND f.away_score IS NOT NULL
          AND (f.scheduled_year < ? OR (f.scheduled_year = ? AND f.scheduled_day_of_year < ?))
          AND (? IS NULL OR si.competition_id = ?)
          AND (? IS NULL OR c.federation_id = ?)
          AND home_rating IS NOT NULL AND away_rating IS NOT NULL
        ORDER BY f.scheduled_year DESC, f.scheduled_day_of_year DESC, f.id DESC
        LIMIT ?"#,
    )
    .bind(version as i64).bind(version as i64).bind(version as i64).bind(version as i64)
    .bind(cutoff_year).bind(cutoff_year).bind(cutoff_day as i64)
    .bind(competition_id.map(|id| id.to_string())).bind(competition_id.map(|id| id.to_string()))
    .bind(federation_id).bind(federation_id).bind(limit as i64)
    .fetch_all(pool).await?)
}
