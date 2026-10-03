mod attributes;
mod behavior;
mod commanded_plan;
mod commanded_realignment;
mod fixtures;
mod formation;
mod lifecycle;
mod plan_storage;
mod realignment;
mod roster;
mod scenario_input;
mod simulation;
mod stress;

use arlo_controller::services::season::matchday::{
    build_matchday_setup, get_or_load_matchday_catalogs,
};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::path::{Path, PathBuf};
use std::time::Instant;
use uuid::Uuid;

type AuditResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AuditResult<()> {
    let source = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "database/arlo.db".into()),
    );
    let limit: i64 = std::env::args()
        .nth(2)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(8);
    if !(1..=10_000).contains(&limit) {
        return Err("Choose between one and ten thousand audit fixtures".into());
    }
    if std::env::args().nth(3).as_deref() == Some("--prepared-copy") {
        let source = source.canonicalize()?;
        let allowed = std::env::current_dir()?
            .join("target/manager-audit")
            .canonicalize()?;
        if source.parent() != Some(allowed.as_path())
            || !source
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("arlo-manager-audit-"))
        {
            return Err("Prepared copies must be owned audit files in target/manager-audit".into());
        }
        return audit(&source, limit).await;
    }
    let copy = std::env::temp_dir().join(format!("arlo-manager-ai-audit-{}.db", Uuid::new_v4()));
    snapshot(&source, &copy).await?;
    let result = audit(&copy, limit).await;
    let cleanup = std::fs::remove_file(&copy);
    if let Err(error) = cleanup {
        eprintln!("Audit snapshot retained at {}: {error}", copy.display());
    }
    result
}

async fn snapshot(source: &Path, destination: &Path) -> AuditResult<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(SqliteConnectOptions::new().filename(source).read_only(true))
        .await?;
    let result = sqlx::query("VACUUM INTO ?")
        .bind(destination.to_string_lossy().as_ref())
        .execute(&pool)
        .await;
    pool.close().await;
    result?;
    Ok(())
}

async fn audit(path: &Path, limit: i64) -> AuditResult<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(path)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Memory)
                .synchronous(SqliteSynchronous::Off),
        )
        .await?;
    let result = audit_pool(&pool, limit).await;
    if let Ok(totals) = &result {
        std::fs::write(
            path.with_extension("report.json"),
            serde_json::to_vec_pretty(totals)?,
        )?;
    }
    pool.close().await;
    result.map(|_| ())
}

async fn audit_pool(pool: &sqlx::SqlitePool, limit: i64) -> AuditResult<simulation::AuditTotals> {
    arlo_db::save::migrations::run_migrations(pool).await?;
    arlo_controller::persistence::run_migrations(pool).await?;
    let catalogs = get_or_load_matchday_catalogs(pool).await?;
    let selector = std::env::args().nth(4);
    let fixtures = fixtures::load(pool, limit)
        .await?
        .into_iter()
        .filter(|fixture| selector.as_ref().is_none_or(|id| *id == fixture.id));
    let started = Instant::now();
    let mut totals = simulation::AuditTotals::default();
    let mut sample = None;
    for fixture in fixtures {
        let prepared = match build_matchday_setup(pool, &fixture, &catalogs).await {
            Ok(prepared) => prepared,
            Err(error) => {
                println!("Fixture {} preparation skipped: {error}", fixture.id);
                totals.skipped += 1;
                continue;
            }
        };
        if sample.is_none() {
            sample = Some((
                fixture.clone(),
                prepared.input.clone(),
                prepared.play_calls.clone(),
            ));
        }
        for team in [prepared.input.home(), prepared.input.away()] {
            let formation_plans = team
                .prepared_plans()
                .iter()
                .filter(|plan| plan.layout.formation.id() != team.formation().id())
                .count();
            println!(
                "Team {}: {} prepared plans, {} alternative formations",
                team.team_id(),
                team.prepared_plans().len(),
                formation_plans
            );
        }
        match simulation::audit_match(&prepared.input, &prepared.play_calls) {
            Ok(result) => {
                println!("Fixture {}: {result:?}", fixture.id);
                totals.include(&result);
            }
            Err(error) => {
                println!("Fixture {} simulation skipped: {error}", fixture.id);
                totals.skipped += 1;
            }
        }
    }
    println!("Totals: {totals:?}");
    println!("Elapsed: {:?}", started.elapsed());
    if totals.completed == 0 || totals.skipped > 0 || totals.non_reproducible > 0 {
        return Err("Audit contains skipped, non-reproducible or no completed fixtures".into());
    }
    if selector.is_none() {
        if let Some((fixture, input, calls)) = sample {
            behavior::audit(&input, &calls)?;
            commanded_plan::audit(pool, &input, &calls).await?;
            commanded_realignment::audit(pool, &input, &calls).await?;
            formation::audit(pool, &fixture, &catalogs).await?;
        }
    }
    Ok(totals)
}
