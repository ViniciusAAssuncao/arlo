use crate::error::{PersistenceError, PersistenceResult};
use arlo_analytics::{PowerRankingSnapshot, TeamPowerSeed};
use sqlx::{Sqlite, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub(super) fn validate_publication(
    snapshot: &PowerRankingSnapshot,
    seeds: &[TeamPowerSeed],
) -> PersistenceResult<HashMap<Uuid, f64>> {
    if snapshot.model_version() == 0 {
        return Err(PersistenceError::InvalidData(
            "power ranking model_version must be positive".into(),
        ));
    }
    if snapshot.entries().is_empty() || snapshot.entries().len() != seeds.len() {
        return Err(PersistenceError::InvalidData(
            "power ranking entries must match the season seeds".into(),
        ));
    }
    let mut initial_ratings = HashMap::with_capacity(seeds.len());
    for seed in seeds {
        let rating = seed.initial_rating().value();
        if !rating.is_finite() || initial_ratings.insert(seed.team_id(), rating).is_some() {
            return Err(PersistenceError::InvalidData(format!(
                "invalid or duplicate power ranking seed for team {}",
                seed.team_id()
            )));
        }
    }
    let mut seen_teams = std::collections::HashSet::new();
    for (index, entry) in snapshot.entries().iter().enumerate() {
        if entry.rank() as usize != index + 1
            || !entry.rating().value().is_finite()
            || !seen_teams.insert(entry.team_id())
            || !initial_ratings.contains_key(&entry.team_id())
        {
            return Err(PersistenceError::InvalidData(format!(
                "invalid power ranking entry for team {}",
                entry.team_id()
            )));
        }
    }
    Ok(initial_ratings)
}

pub(super) async fn ensure_existing_seeds(
    tx: &mut Transaction<'_, Sqlite>,
    season_instance_id: Uuid,
    model_version: u32,
    initial_ratings: &HashMap<Uuid, f64>,
) -> PersistenceResult<()> {
    let rows: Vec<(String, f64)> = sqlx::query_as(
        r#"SELECT team_id, initial_rating
        FROM power_ranking_entries
        WHERE snapshot_id = (
            SELECT s.id
            FROM power_ranking_snapshots s
            JOIN power_ranking_entries e ON e.snapshot_id = s.id
            WHERE s.season_instance_id = ? AND s.model_version = ?
            ORDER BY s.year, s.day_of_year
            LIMIT 1
        )"#,
    )
    .bind(season_instance_id.to_string())
    .bind(model_version as i64)
    .fetch_all(&mut **tx)
    .await?;
    if rows.is_empty() {
        return Ok(());
    }
    if rows.len() != initial_ratings.len() {
        return Err(PersistenceError::InvalidData(
            "power ranking season seeds differ from published seeds".into(),
        ));
    }
    for (team_id, rating) in rows {
        if initial_ratings.get(&Uuid::parse_str(&team_id)?) != Some(&rating) {
            return Err(PersistenceError::InvalidData(format!(
                "power ranking season seed changed for team {team_id}"
            )));
        }
    }
    Ok(())
}
