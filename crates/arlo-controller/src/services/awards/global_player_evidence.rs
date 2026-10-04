use super::global_evidence_merge::merge_player_evidence;
use super::season_player_evidence::{
    load_season_player_evidence, load_season_player_evidence_between,
};
use crate::domain::calendar::CalendarDate;
use crate::error::ControllerResult;
use arlo_domain::{AwardCandidateEvidence, AwardMetric};
use sqlx::{Row, SqlitePool};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(super) struct GlobalSeasonSource {
    pub season_id: Uuid,
    pub competition_id: Uuid,
}

pub(super) struct DatedSeasonSource {
    pub source: GlobalSeasonSource,
    pub start: CalendarDate,
    pub end: CalendarDate,
    pub age_cutoff_season_id: Uuid,
    pub include_honors: bool,
}

pub(super) async fn load_dated_global_player_evidence(
    pool: &SqlitePool,
    sources: &[DatedSeasonSource],
) -> ControllerResult<Vec<AwardCandidateEvidence>> {
    let mut seasons = Vec::with_capacity(sources.len());
    for dated in sources {
        let mut evidence = load_season_player_evidence_between(
            pool,
            dated.source.season_id,
            dated.source.competition_id,
            dated.start,
            dated.end,
            dated.age_cutoff_season_id,
        )
        .await?;
        let title_winners = if dated.include_honors {
            title_winning_players(pool, &dated.source).await?
        } else {
            BTreeSet::new()
        };
        let individual_awards = if dated.include_honors {
            individual_award_winners(pool, &dated.source).await?
        } else {
            Vec::new()
        };
        for candidate in &mut evidence {
            candidate.metrics.push(AwardMetric {
                key: "competition_titles_won".into(),
                value: f64::from(title_winners.contains(&candidate.subject_id)),
            });
            candidate.metrics.push(AwardMetric {
                key: "individual_awards_won".into(),
                value: individual_awards
                    .iter()
                    .filter(|id| **id == candidate.subject_id)
                    .count() as f64,
            });
        }
        seasons.push(evidence);
    }
    let mut merged = merge_player_evidence(seasons);
    for candidate in &mut merged {
        candidate.metrics.push(AwardMetric {
            key: "competitions_played".into(),
            value: candidate.competition_ids.len() as f64,
        });
    }
    Ok(merged)
}

pub(super) async fn load_global_player_evidence(
    pool: &SqlitePool,
    sources: &[GlobalSeasonSource],
) -> ControllerResult<Vec<AwardCandidateEvidence>> {
    let mut seasons = Vec::with_capacity(sources.len());
    for source in sources {
        let mut evidence =
            load_season_player_evidence(pool, source.season_id, source.competition_id).await?;
        let title_winners = title_winning_players(pool, source).await?;
        let individual_awards = individual_award_winners(pool, source).await?;
        for candidate in &mut evidence {
            candidate.metrics.push(AwardMetric {
                key: "competition_titles_won".into(),
                value: f64::from(title_winners.contains(&candidate.subject_id)),
            });
            candidate.metrics.push(AwardMetric {
                key: "individual_awards_won".into(),
                value: individual_awards
                    .iter()
                    .filter(|id| **id == candidate.subject_id)
                    .count() as f64,
            });
        }
        seasons.push(evidence);
    }
    let mut merged = merge_player_evidence(seasons);
    for candidate in &mut merged {
        candidate.metrics.push(AwardMetric {
            key: "competitions_played".into(),
            value: candidate.competition_ids.len() as f64,
        });
    }
    Ok(merged)
}

async fn title_winning_players(
    pool: &SqlitePool,
    source: &GlobalSeasonSource,
) -> ControllerResult<BTreeSet<Uuid>> {
    let rows = sqlx::query(
        "SELECT DISTINCT perf.player_id FROM match_player_performance perf JOIN matches m ON m.id = perf.match_id JOIN fixtures f ON f.id = m.fixture_id JOIN season_stages ss ON ss.id = f.season_stage_id JOIN season_instances si ON si.id = ss.season_instance_id JOIN titles t ON t.competition_id = si.competition_id AND t.season_label = CAST(si.reference_year AS TEXT) AND t.winner_team_id = perf.team_id WHERE si.id = ? AND perf.effective_opportunities > 0",
    )
    .bind(source.season_id.to_string())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| Ok(Uuid::parse_str(row.try_get::<&str, _>("player_id")?)?))
        .collect()
}

async fn individual_award_winners(
    pool: &SqlitePool,
    source: &GlobalSeasonSource,
) -> ControllerResult<Vec<Uuid>> {
    let rows = sqlx::query(
        "SELECT r.subject_id FROM award_results r JOIN award_instances i ON i.id = r.award_instance_id WHERE i.period_key = ? AND r.subject_kind = 'Player' UNION ALL SELECT r.subject_id FROM award_roster_results r JOIN award_instances i ON i.id = r.award_instance_id WHERE i.period_key = ? AND r.subject_kind = 'Player'",
    )
    .bind(format!("season:{}", source.season_id))
    .bind(format!("season:{}", source.season_id))
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| Ok(Uuid::parse_str(row.try_get::<&str, _>("subject_id")?)?))
        .collect()
}
