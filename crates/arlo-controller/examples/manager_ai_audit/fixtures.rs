use super::AuditResult;
use arlo_persistence::models::season::FixtureRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load(pool: &SqlitePool, limit: i64) -> AuditResult<Vec<FixtureRow>> {
    let existing = sqlx::query_as::<_, FixtureRow>(
        "SELECT * FROM fixtures ORDER BY scheduled_year, scheduled_day_of_year, id LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    if !existing.is_empty() {
        return Ok(existing);
    }
    let teams: Vec<String> = sqlx::query_scalar("SELECT id FROM teams ORDER BY id")
        .fetch_all(pool)
        .await?;
    if teams.len() < 2 {
        return Err("The source database needs at least two teams".into());
    }
    let competition: String = sqlx::query_scalar("SELECT id FROM competitions ORDER BY id LIMIT 1")
        .fetch_one(pool)
        .await?;
    let season_id = Uuid::from_u128(0x61756469742d736561736f6e00000001).to_string();
    let stage_id = Uuid::from_u128(0x61756469742d73746167650000000001).to_string();
    sqlx::query(
        "INSERT INTO season_instances (id, competition_id, reference_year, current_stage_order_index, status, created_at_unix_seconds) VALUES (?, ?, 3628, 0, 'Active', 0)",
    ).bind(&season_id).bind(competition).execute(pool).await?;
    sqlx::query(
        "INSERT INTO season_stages (id, season_instance_id, stage_order_index, stage_type, status) VALUES (?, ?, 0, 'RoundRobin', 'Active')",
    ).bind(&stage_id).bind(season_id).execute(pool).await?;
    let mut fixtures = Vec::new();
    for index in 0..limit as usize {
        let home = &teams[index * 2 % teams.len()];
        let away = &teams[(index * 2 + 1) % teams.len()];
        fixtures.push(FixtureRow {
            id: Uuid::from_u128(0x61756469742d66697874757265000000 + index as u128).to_string(),
            season_stage_id: stage_id.clone(),
            round_index: index as i32,
            home_team_id: home.clone(),
            away_team_id: away.clone(),
            is_neutral_venue: false,
            venue_id: None,
            scheduled_year: 3628,
            scheduled_day_of_year: 1,
            status: "Scheduled".into(),
            home_score: None,
            away_score: None,
            home_goal_points: None,
            away_goal_points: None,
            home_field_goals: None,
            away_field_goals: None,
            home_field_points: None,
            away_field_points: None,
        });
    }
    println!(
        "Template mode: {} audit fixtures prepared from existing teams",
        fixtures.len()
    );
    Ok(fixtures)
}
