use crate::error::PersistenceResult;
use arlo_awards::AwardResolution;
use arlo_domain::{AwardDefinition, AwardRecipientKind};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct AwardHistoryRow {
    pub instance_id: Uuid,
    pub definition_id: Uuid,
    pub award_code: String,
    pub award_name: String,
    pub period_key: String,
    pub subject_kind: String,
    pub subject_id: Uuid,
    pub rank: usize,
    pub is_winner: bool,
    pub organizer_league_id: Option<Uuid>,
}

pub async fn persist_resolution(
    tx: &mut Transaction<'_, Sqlite>,
    resolution: &AwardResolution,
    definition: &AwardDefinition,
    organizer_league_id: Option<Uuid>,
) -> PersistenceResult<Uuid> {
    let instance_id = Uuid::new_v4();
    let inserted = sqlx::query(
        "INSERT INTO award_instances (id, award_definition_id, period_key, scope_id, seed, selection_model_version, definition_snapshot, organizer_id, organizer_league_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(instance_id.to_string())
    .bind(resolution.definition_id.to_string())
    .bind(&resolution.period_key)
    .bind(resolution.scope_id.map(|id| id.to_string()))
    .bind(resolution.seed.to_string())
    .bind(i64::from(resolution.model_version))
    .bind(serde_json::to_string(definition)?)
    .bind(definition.organizer_id.map(|id| id.to_string()))
    .bind(organizer_league_id.map(|id| id.to_string()))
    .execute(&mut **tx).await?;
    if inserted.rows_affected() == 0 {
        let row = sqlx::query(
            "SELECT id FROM award_instances WHERE award_definition_id = ? AND period_key = ? AND scope_id IS ?"
        )
        .bind(resolution.definition_id.to_string())
        .bind(&resolution.period_key)
        .bind(resolution.scope_id.map(|id| id.to_string()))
        .fetch_one(&mut **tx).await?;
        return Ok(Uuid::parse_str(row.try_get::<&str, _>("id")?)?);
    }
    let kind = recipient_kind_code(definition.recipient_kind);
    for candidate in &resolution.candidates {
        sqlx::query(
            "INSERT INTO award_nominees (award_instance_id, subject_kind, subject_id, utility, selection_score, final_rank) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(instance_id.to_string())
        .bind(kind)
        .bind(candidate.subject_id.to_string())
        .bind(candidate.utility)
        .bind(candidate.selection_score)
        .bind(candidate.rank as i64)
        .execute(&mut **tx).await?;
    }
    for group in &resolution.electorates {
        sqlx::query(
            "INSERT INTO award_electorate_snapshots (award_instance_id, group_code, voter_count, result_weight) VALUES (?, ?, ?, ?)"
        )
        .bind(instance_id.to_string())
        .bind(&group.group_code)
        .bind(i64::from(group.voter_count))
        .bind(group.result_weight)
        .execute(&mut **tx).await?;
        for (index, subject_id) in group.subject_ids.iter().enumerate() {
            sqlx::query(
                "INSERT INTO award_vote_tallies (award_instance_id, group_code, subject_id, points, first_place_votes) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(instance_id.to_string())
            .bind(&group.group_code)
            .bind(subject_id.to_string())
            .bind(group.points[index] as i64)
            .bind(i64::from(group.first_place[index]))
            .execute(&mut **tx).await?;
        }
    }
    sqlx::query(
        "INSERT INTO award_results (award_instance_id, subject_kind, subject_id) VALUES (?, ?, ?)",
    )
    .bind(instance_id.to_string())
    .bind(kind)
    .bind(resolution.winner_id.to_string())
    .execute(&mut **tx)
    .await?;
    Ok(instance_id)
}

pub async fn winner_for_match(
    pool: &SqlitePool,
    award_code: &str,
    match_id: Uuid,
) -> PersistenceResult<Option<Uuid>> {
    let row = sqlx::query(
        "SELECT r.subject_id FROM award_results r JOIN award_instances i ON i.id = r.award_instance_id JOIN award_definitions d ON d.id = i.award_definition_id WHERE d.code = ? AND i.period_key = ?"
    )
    .bind(award_code)
    .bind(format!("match:{match_id}"))
    .fetch_optional(pool).await?;
    row.map(|row| Uuid::parse_str(row.try_get::<&str, _>("subject_id")?).map_err(Into::into))
        .transpose()
}

pub async fn list_history_for_subject(
    pool: &SqlitePool,
    subject_kind: AwardRecipientKind,
    subject_id: Uuid,
) -> PersistenceResult<Vec<AwardHistoryRow>> {
    let rows = sqlx::query(
        "SELECT i.id AS instance_id, i.definition_snapshot, i.period_key, n.subject_kind, n.subject_id, n.final_rank, r.subject_id AS winner_id, i.organizer_league_id FROM award_nominees n JOIN award_instances i ON i.id = n.award_instance_id JOIN award_results r ON r.award_instance_id = i.id WHERE n.subject_kind = ? AND n.subject_id = ? ORDER BY i.resolved_at DESC, i.period_key"
    )
    .bind(recipient_kind_code(subject_kind))
    .bind(subject_id.to_string())
    .fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| {
            let definition: AwardDefinition =
                serde_json::from_str(row.try_get("definition_snapshot")?)?;
            let subject_id = Uuid::parse_str(row.try_get::<&str, _>("subject_id")?)?;
            let winner_id = Uuid::parse_str(row.try_get::<&str, _>("winner_id")?)?;
            let rank = usize::try_from(row.try_get::<i64, _>("final_rank")?)
                .map_err(|_| crate::error::PersistenceError::InvalidData("award rank".into()))?;
            let organizer_league_id = row
                .try_get::<Option<String>, _>("organizer_league_id")?
                .map(|value| Uuid::parse_str(&value))
                .transpose()?;
            Ok(AwardHistoryRow {
                instance_id: Uuid::parse_str(row.try_get::<&str, _>("instance_id")?)?,
                definition_id: definition.id,
                award_code: definition.code,
                award_name: definition.name,
                period_key: row.try_get("period_key")?,
                subject_kind: row.try_get("subject_kind")?,
                subject_id,
                rank,
                is_winner: subject_id == winner_id,
                organizer_league_id,
            })
        })
        .collect()
}

fn recipient_kind_code(kind: AwardRecipientKind) -> &'static str {
    match kind {
        AwardRecipientKind::Player => "Player",
        AwardRecipientKind::Team => "Team",
        AwardRecipientKind::Manager => "Manager",
        AwardRecipientKind::Referee => "Referee",
        AwardRecipientKind::Person => "Person",
        AwardRecipientKind::Federation => "Federation",
    }
}
