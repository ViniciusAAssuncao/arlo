use super::global_player_evidence::GlobalSeasonSource;
use crate::error::ControllerResult;
use arlo_domain::AwardDefinition;
use sqlx::{FromRow, SqlitePool};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(FromRow)]
struct SeasonSourceRow {
    id: String,
    competition_id: String,
    status: String,
    prestige: i64,
}

pub(super) async fn ready_sources(
    pool: &SqlitePool,
    definition: &AwardDefinition,
    year: i64,
) -> ControllerResult<Option<Vec<GlobalSeasonSource>>> {
    let rows = sqlx::query_as::<_, SeasonSourceRow>(
        "SELECT si.id, si.competition_id, si.status, c.prestige FROM season_instances si JOIN competitions c ON c.id = si.competition_id WHERE si.reference_year = ? ORDER BY si.competition_id, si.id",
    )
    .bind(year)
    .fetch_all(pool)
    .await?;
    let mut sources = Vec::new();
    let mut represented = HashSet::new();
    for row in rows {
        let competition_id = Uuid::parse_str(&row.competition_id)?;
        if !definition.eligible_competitions.is_empty()
            && !definition.eligible_competitions.contains(&competition_id)
        {
            continue;
        }
        if definition
            .minimum_competition_prestige
            .is_some_and(|minimum| row.prestige < i64::from(minimum))
        {
            continue;
        }
        if row.status != "Completed" {
            return Ok(None);
        }
        represented.insert(competition_id);
        sources.push(GlobalSeasonSource {
            season_id: Uuid::parse_str(&row.id)?,
            competition_id,
        });
    }
    if sources.is_empty()
        || definition
            .eligible_competitions
            .iter()
            .any(|competition| !represented.contains(competition))
    {
        return Ok(None);
    }
    for source in &sources {
        if uses_metric(definition, "competition_titles_won") {
            let title_exists = sqlx::query_scalar::<_, i64>(
                "SELECT 1 FROM titles WHERE competition_id = ? AND season_label = ? LIMIT 1",
            )
            .bind(source.competition_id.to_string())
            .bind(year.to_string())
            .fetch_optional(pool)
            .await?
            .is_some();
            if !title_exists {
                return Ok(None);
            }
        }
        if uses_metric(definition, "individual_awards_won") {
            let pending = sqlx::query_scalar::<_, i64>(
                "SELECT 1 FROM award_season_jobs WHERE season_instance_id = ? AND status = 'Pending' LIMIT 1",
            )
            .bind(source.season_id.to_string())
            .fetch_optional(pool)
            .await?
            .is_some();
            if pending {
                return Ok(None);
            }
        }
    }
    Ok(Some(sources))
}

fn uses_metric(definition: &AwardDefinition, key: &str) -> bool {
    definition.criteria.iter().any(|item| item.key == key)
        || definition
            .roster_slots
            .iter()
            .flat_map(|slot| slot.criteria.iter())
            .any(|item| item.key == key)
        || definition
            .dynamic_position_profiles
            .iter()
            .flat_map(|profile| profile.criteria.iter())
            .any(|item| item.key == key)
}
