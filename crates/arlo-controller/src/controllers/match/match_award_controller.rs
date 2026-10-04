use crate::error::{ControllerError, ControllerResult};
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_awards::{resolve_award, AwardResolution};
use arlo_domain::{
    AwardCandidateEvidence, AwardDefinition, AwardInstanceContext, AwardMetric, AwardRecipientKind,
};
use arlo_match_runner::MatchRunResult;
use arlo_persistence::repositories::award_repository;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub(crate) async fn load_match_mvp_definition(
    pool: &SqlitePool,
) -> ControllerResult<Option<AwardDefinition>> {
    Ok(arlo_catalog::get_award_by_code(pool, "match-mvp").await?)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MatchAwardScope {
    pub competition_id: Uuid,
    pub league_id: Option<Uuid>,
}

pub(crate) async fn load_match_award_scope(
    pool: &SqlitePool,
    fixture_id: Uuid,
) -> ControllerResult<MatchAwardScope> {
    let row = sqlx::query(
        "SELECT si.competition_id, l.competition_id AS league_id FROM fixtures f JOIN season_stages ss ON ss.id = f.season_stage_id JOIN season_instances si ON si.id = ss.season_instance_id LEFT JOIN leagues l ON l.competition_id = si.competition_id WHERE f.id = ?"
    )
    .bind(fixture_id.to_string())
    .fetch_one(pool).await?;
    let competition_id = Uuid::parse_str(row.try_get::<&str, _>("competition_id")?)?;
    let league_id = row
        .try_get::<Option<String>, _>("league_id")?
        .map(|value| Uuid::parse_str(&value))
        .transpose()?;
    Ok(MatchAwardScope {
        competition_id,
        league_id,
    })
}

pub(crate) fn resolve_match_mvp(
    definition: Option<&AwardDefinition>,
    match_id: Uuid,
    scope: MatchAwardScope,
    seed: u64,
    result: &MatchRunResult,
) -> ControllerResult<Option<AwardResolution>> {
    let Some(definition) = definition else {
        return Ok(None);
    };
    let aggregator = result
        .aggregators
        .get::<PlayerPerformanceAggregator>()
        .ok_or_else(|| {
            ControllerError::InvalidData("player performance aggregator is missing".into())
        })?;
    if !aggregator.is_finalized() {
        return Err(ControllerError::InvalidData(
            "player performance aggregator is not finalized".into(),
        ));
    }
    let evidence = aggregator
        .all_player_snapshots()
        .iter()
        .filter(|snapshot| {
            snapshot.effective_opportunities() > 0 && snapshot.confidence().value() > 0.0
        })
        .map(|snapshot| {
            let breakdown = snapshot.breakdown();
            AwardCandidateEvidence {
                subject_kind: AwardRecipientKind::Player,
                subject_id: snapshot.player_id(),
                position: Some(format!("{:?}", snapshot.offensive_position())),
                position_family: None,
                country_id: None,
                continent_id: None,
                competition_id: Some(scope.competition_id),
                team_id: Some(snapshot.team_id()),
                matches_played: 1,
                metrics: vec![
                    metric("performance_rating", snapshot.performance_rating().value()),
                    metric("high_impact", breakdown.high_impact()),
                    metric("production", breakdown.production()),
                    metric("defense", breakdown.defense()),
                    metric("discipline", breakdown.discipline()),
                ],
            }
        })
        .collect::<Vec<_>>();
    if evidence.is_empty() {
        return Ok(None);
    }
    let context = AwardInstanceContext {
        period_key: format!("match:{match_id}"),
        scope_id: Some(scope.competition_id),
        selection_model_version: 1,
    };
    let mut award_seed = seed;
    for byte in match_id.as_bytes().iter().chain(definition.id.as_bytes()) {
        award_seed = award_seed.wrapping_mul(0x100000001b3) ^ u64::from(*byte);
    }
    Ok(Some(resolve_award(
        definition, &context, &evidence, award_seed,
    )?))
}

pub(crate) async fn persist_match_mvp(
    tx: &mut Transaction<'_, Sqlite>,
    definition: Option<&AwardDefinition>,
    resolution: Option<&AwardResolution>,
    scope: MatchAwardScope,
) -> ControllerResult<()> {
    if let (Some(definition), Some(resolution)) = (definition, resolution) {
        award_repository::persist_resolution(tx, resolution, definition, scope.league_id).await?;
    }
    Ok(())
}

fn metric(key: &str, value: f64) -> AwardMetric {
    AwardMetric {
        key: key.into(),
        value,
    }
}
