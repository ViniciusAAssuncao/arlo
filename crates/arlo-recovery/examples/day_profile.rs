use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::PathBuf;
use std::time::Instant;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| "database/arlo.db".into()));
    let days: u32 = std::env::args().nth(2).map(|value| value.parse()).transpose()?.unwrap_or(2);
    let copy = std::env::temp_dir().join(format!("arlo-day-profile-{}.db", Uuid::new_v4()));
    std::fs::copy(&source, &copy)?;
    let result = profile(&copy, days).await;
    for _ in 0..20 {
        if std::fs::remove_file(&copy).is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    result
}

async fn profile(path: &PathBuf, days: u32) -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new().max_connections(1)
        .connect_with(SqliteConnectOptions::new().filename(path)).await?;
    let has_outside_catalog: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'outside_match_injury_definitions'",
    ).fetch_one(&pool).await?;
    if has_outside_catalog == 0 {
        sqlx::raw_sql(include_str!("../../../migrations/0103_add_outside_match_conditions.sql"))
            .execute(&pool).await?;
    }
    let mut full_scan = Vec::new();
    let mut compact_scan = Vec::new();
    for _ in 0..3 {
        let start = Instant::now();
        let players = arlo_db::repositories::player::list_all_with_team(&pool).await?;
        full_scan.push((players.len(), start.elapsed()));
        let start = Instant::now();
        let players = arlo_db::repositories::player::list_daily_recovery_inputs(&pool).await?;
        compact_scan.push((players.len(), start.elapsed()));
    }
    let day_one = Instant::now();
    if days >= 1 {
        arlo_recovery::orchestration::advance_all_players_one_day(&pool, 3628, 1).await?;
    }
    let day_one = day_one.elapsed();
    let day_two = Instant::now();
    if days >= 2 {
        arlo_recovery::orchestration::advance_all_players_one_day(&pool, 3628, 2).await?;
    }
    let day_two = day_two.elapsed();
    let remaining = Instant::now();
    for day in 3..=days {
        arlo_recovery::orchestration::advance_all_players_one_day(&pool, 3628, day).await?;
    }
    let remaining = remaining.elapsed();
    let outside_incidents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM player_injury_history WHERE injury_definition_id IN (SELECT injury_definition_id FROM outside_match_injury_definitions WHERE daily_weight > 0)",
    ).fetch_one(&pool).await?;
    let by_diagnosis: Vec<(String, i64, i32, i32)> = sqlx::query_as(
        "SELECT d.code, COUNT(*), MIN(h.expected_recovery_days), MAX(h.expected_recovery_days) FROM player_injury_history h JOIN injury_definitions d ON d.id = h.injury_definition_id JOIN outside_match_injury_definitions o ON o.injury_definition_id = d.id GROUP BY d.code ORDER BY COUNT(*) DESC",
    ).fetch_all(&pool).await?;
    println!("Full player scan: {full_scan:?}");
    println!("Compact recovery scan: {compact_scan:?}");
    println!("Daily recovery: first {day_one:?}, steady {day_two:?}");
    println!("{days} days: {outside_incidents} outside-match incidents, remaining days {remaining:?}");
    println!("Diagnoses: {by_diagnosis:?}");
    pool.close().await;
    drop(pool);
    Ok(())
}
