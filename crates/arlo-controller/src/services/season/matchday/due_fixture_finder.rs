use crate::error::ControllerResult;
use arlo_persistence::models::season::FixtureRow;
use sqlx::SqlitePool;

pub async fn find_due_fixtures(
    pool: &SqlitePool,
    year: i64,
    day_of_year: u32,
) -> ControllerResult<Vec<FixtureRow>> {
    let fixtures = arlo_persistence::repositories::season::fixtures::list_scheduled_on_date(
        pool,
        year,
        day_of_year,
    )
    .await?;
    Ok(fixtures)
}