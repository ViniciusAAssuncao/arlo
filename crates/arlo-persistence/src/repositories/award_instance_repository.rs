use crate::error::{PersistenceError, PersistenceResult};
use arlo_domain::{AwardDefinition, AwardOrganizerPolicy};
use sqlx::{Row, Sqlite, Transaction};
use uuid::Uuid;

pub(crate) async fn insert_instance(
    tx: &mut Transaction<'_, Sqlite>,
    definition: &AwardDefinition,
    period_key: &str,
    scope_id: Option<Uuid>,
    seed: u64,
    model_version: u32,
    organizer_league_id: Option<Uuid>,
) -> PersistenceResult<(Uuid, bool)> {
    let organizer_federation_id = if definition.organizer_policy
        == AwardOrganizerPolicy::FederationCommittee
    {
        let competition_id = scope_id.ok_or_else(|| {
            PersistenceError::InvalidData("federation award requires a competition scope".into())
        })?;
        let row = sqlx::query("SELECT federation_id FROM competitions WHERE id = ?")
            .bind(competition_id.to_string())
            .fetch_one(&mut **tx)
            .await?;
        Some(Uuid::parse_str(row.try_get::<&str, _>("federation_id")?)?)
    } else {
        None
    };
    let instance_id = Uuid::new_v4();
    let inserted = sqlx::query(
        "INSERT INTO award_instances (id, award_definition_id, period_key, scope_id, seed, selection_model_version, definition_snapshot, organizer_id, organizer_league_id, organizer_federation_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(instance_id.to_string())
    .bind(definition.id.to_string())
    .bind(period_key)
    .bind(scope_id.map(|id| id.to_string()))
    .bind(seed.to_string())
    .bind(i64::from(model_version))
    .bind(serde_json::to_string(definition)?)
    .bind(definition.organizer_id.map(|id| id.to_string()))
    .bind(
        (definition.organizer_policy == AwardOrganizerPolicy::LeagueCommittee)
            .then_some(organizer_league_id)
            .flatten()
            .map(|id| id.to_string()),
    )
    .bind(organizer_federation_id.map(|id| id.to_string()))
    .execute(&mut **tx)
    .await?;
    if inserted.rows_affected() > 0 {
        return Ok((instance_id, true));
    }
    let row = sqlx::query(
        "SELECT id FROM award_instances WHERE award_definition_id = ? AND period_key = ? AND scope_id IS ?",
    )
    .bind(definition.id.to_string())
    .bind(period_key)
    .bind(scope_id.map(|id| id.to_string()))
    .fetch_one(&mut **tx)
    .await?;
    Ok((Uuid::parse_str(row.try_get::<&str, _>("id")?)?, false))
}
