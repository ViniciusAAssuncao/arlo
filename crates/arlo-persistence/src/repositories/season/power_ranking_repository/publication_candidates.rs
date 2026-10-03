use crate::error::PersistenceResult;
use sqlx::SqlitePool;

pub async fn list_publication_candidates(
    pool: &SqlitePool,
    year: i64,
    day_of_year: u32,
    model_version: u32,
) -> PersistenceResult<Vec<(String, String)>> {
    let rows = sqlx::query_as::<_, (String, String)>(
        r#"SELECT si.id, si.status
        FROM season_instances si
        WHERE (si.status = 'Active' AND EXISTS (
               SELECT 1
               FROM fixtures f
               JOIN season_stages st ON st.id = f.season_stage_id
               WHERE st.season_instance_id = si.id
           ))
           OR (si.status = 'Completed' AND EXISTS (
               SELECT 1
               FROM fixtures f
               JOIN season_stages st ON st.id = f.season_stage_id
               WHERE st.season_instance_id = si.id
                 AND f.status = 'Completed'
                 AND (f.scheduled_year < ? OR
                      (f.scheduled_year = ? AND f.scheduled_day_of_year <= ?))
                 AND NOT EXISTS (
                     SELECT 1 FROM power_ranking_snapshots prs
                     WHERE prs.season_instance_id = si.id
                       AND prs.model_version = ?
                       AND (prs.year > f.scheduled_year OR
                            (prs.year = f.scheduled_year
                             AND prs.day_of_year >= f.scheduled_day_of_year))
                 )
           ))
        ORDER BY si.reference_year, si.id"#,
    )
    .bind(year)
    .bind(year)
    .bind(day_of_year as i64)
    .bind(model_version as i64)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
