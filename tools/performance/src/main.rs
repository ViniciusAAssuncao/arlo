mod allocations;
mod comparison;
mod simulation;
mod sql_trace;
mod wire;

use arlo_controller::services::season::matchday::{
    build_matchday_setup, get_or_load_matchday_catalogs,
};
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::persister::MatchPersister;
use std::path::PathBuf;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[global_allocator]
static ALLOCATOR: allocations::CountingAllocator = allocations::CountingAllocator;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        return Err("capture|run|verify|cpu owned-copy.db output-dir [limit] [repeats]".into());
    }
    let path = PathBuf::from(&args[2]).canonicalize()?;
    let allowed = std::env::current_dir()?
        .join("target/performance")
        .canonicalize()?;
    if !path.starts_with(&allowed) || path.extension().is_none_or(|v| v != "db") {
        return Err("Database must be an owned copy under target/performance".into());
    }
    let output = PathBuf::from(&args[3]);
    std::fs::create_dir_all(&output)?;
    let limit: i64 = args.get(4).map(|v| v.parse()).transpose()?.unwrap_or(12);
    let repeats: usize = args.get(5).map(|v| v.parse()).transpose()?.unwrap_or(5);
    if !(1..=10000).contains(&limit) || !(1..=50).contains(&repeats) {
        return Err("Invalid sample size".into());
    }
    let started = Instant::now();
    sql_trace::enable()?;
    let database_url = format!(
        "sqlite://{}",
        path.to_string_lossy().trim_start_matches(r"\\?\")
    );
    let pool = arlo_db::open_pool(&database_url).await?;
    arlo_controller::persistence::run_migrations(&pool).await?;
    let startup_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let catalogs = get_or_load_matchday_catalogs(&pool).await?;
    let catalogs_ms = started.elapsed().as_secs_f64() * 1000.0;
    let fixtures = sqlx::query_as::<_, FixtureRow>(
        "SELECT * FROM fixtures ORDER BY scheduled_year, scheduled_day_of_year, id LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&pool)
    .await?;
    let mut reports = Vec::new();
    for fixture in fixtures {
        let input_path = output.join(format!("{}.input.json", fixture.id));
        if args[1] == "capture" {
            sql_trace::take();
            let started = Instant::now();
            let prepared = build_matchday_setup(&pool, &fixture, &catalogs).await?;
            let preparation_ms = started.elapsed().as_secs_f64() * 1000.0;
            let captured = wire::CapturedFixture::capture(&prepared);
            std::fs::write(input_path, serde_json::to_vec(&captured)?)?;
            let sql = sql_trace::take();
            reports.push(serde_json::json!({"fixture":fixture.id,"preparation_ms":preparation_ms,"sql":sql}));
            println!("{} preparation {:.3} ms", fixture.id, preparation_ms);
            continue;
        }
        if !matches!(args[1].as_str(), "run" | "verify" | "cpu") {
            return Err("Unknown mode".into());
        }
        let captured: wire::CapturedFixture = serde_json::from_slice(&std::fs::read(input_path)?)?;
        let input = captured.restore(&catalogs)?;
        let started = Instant::now();
        let warmup = simulation::run(&input, &captured.calls)?;
        let first_run_ms = started.elapsed().as_secs_f64() * 1000.0;
        let semantic = simulation::semantic_result(&warmup.0, &warmup.1)?;
        let expected_path = output.join(format!("{}.expected.json", fixture.id));
        comparison::verify(&expected_path, &semantic)?;
        if args[1] == "cpu" {
            reports.push(serde_json::json!({"fixture":fixture.id,"match_id":captured.match_id,"simulation_ms":[first_run_ms]}));
            println!("{} verified {:.3} ms", fixture.id, first_run_ms);
            continue;
        }
        if args[1] == "verify" {
            sql_trace::take();
            let mut tx = pool.begin().await?;
            let started = Instant::now();
            MatchPersister::persist_completed_match(
                &mut tx,
                &input,
                &warmup.0,
                &warmup.1,
                &captured.persistence_context(),
            )
            .await?;
            let write_ms = started.elapsed().as_secs_f64() * 1000.0;
            let started = Instant::now();
            tx.commit().await?;
            let commit_ms = started.elapsed().as_secs_f64() * 1000.0;
            let sql = sql_trace::take();
            reports.push(serde_json::json!({"fixture":fixture.id,"match_id":captured.match_id,"simulation_ms":[first_run_ms],"persistence":[{"write_ms":write_ms,"finish_ms":commit_ms,"committed":true,"sql":sql}]}));
            println!("{} verified {:.3} ms", fixture.id, first_run_ms);
            continue;
        }
        drop(warmup);
        let mut times = Vec::new();
        let mut persist = Vec::new();
        for index in 0..repeats {
            let started = Instant::now();
            let (state, result) = simulation::run(&input, &captured.calls)?;
            times.push(started.elapsed().as_secs_f64() * 1000.0);
            let actual = simulation::semantic_result(&state, &result)?;
            if !comparison::compare(&semantic, &actual) {
                std::fs::write(
                    output.join("divergence-expected.json"),
                    serde_json::to_vec(&semantic)?,
                )?;
                std::fs::write(
                    output.join("divergence-actual.json"),
                    serde_json::to_vec(&actual)?,
                )?;
                return Err("Non deterministic simulation".into());
            }
            if std::env::var_os("ARLO_PERF_NO_WRITE").is_some() {
                continue;
            }
            let mut tx = pool.begin().await?;
            let started = Instant::now();
            MatchPersister::persist_completed_match(
                &mut tx,
                &input,
                &state,
                &result,
                &captured.persistence_context(),
            )
            .await?;
            let write_ms = started.elapsed().as_secs_f64() * 1000.0;
            let started = Instant::now();
            if index + 1 == repeats {
                tx.commit().await?;
            } else {
                tx.rollback().await?;
            }
            persist.push(serde_json::json!({"write_ms":write_ms,"finish_ms":started.elapsed().as_secs_f64()*1000.0,"committed":index+1==repeats}));
        }
        let (profile, assessments) = simulation::profile(&input, &captured.calls)?;
        let (allocation_result, allocation_count, allocated_bytes) =
            allocations::measure(|| simulation::run(&input, &captured.calls));
        drop(allocation_result?);
        comparison::verify(
            &output.join(format!("{}.assessments.json", fixture.id)),
            &serde_json::Value::Array(assessments),
        )?;
        println!(
            "{} simulation {:?} ms, dispatch {:.3}, analytics {:.3}, assessments {:.3}",
            fixture.id, times, profile.dispatch_ms, profile.aggregation_ms, profile.assessment_ms
        );
        reports.push(serde_json::json!({"fixture":fixture.id,"match_id":captured.match_id,"first_run_ms":first_run_ms,"simulation_ms":times,"persistence":persist,"profile":profile,"allocation_count":allocation_count,"allocated_bytes":allocated_bytes}));
    }
    let label = std::env::var("ARLO_PERF_LABEL").unwrap_or_else(|_| args[1].clone());
    std::fs::write(
        output.join(format!("{}.report.json", label)),
        serde_json::to_vec_pretty(
            &serde_json::json!({"startup_ms":startup_ms,"catalogs_ms":catalogs_ms,"fixtures":reports}),
        )?,
    )?;
    pool.close().await;
    Ok(())
}
