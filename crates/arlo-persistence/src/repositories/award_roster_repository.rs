use crate::error::PersistenceResult;
use crate::repositories::award_instance_repository::insert_instance;
use crate::repositories::award_repository::recipient_kind_code;
use arlo_awards::AwardRosterResolution;
use arlo_domain::AwardDefinition;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct AwardRosterResultRow {
    pub slot_index: u32,
    pub position_code: String,
    pub selection_group: Option<String>,
    pub slot_role: Option<String>,
    pub subject_kind: String,
    pub subject_id: Uuid,
    pub utility: f64,
    pub selection_score: f64,
}

pub async fn list_roster_results(
    pool: &SqlitePool,
    instance_id: Uuid,
) -> PersistenceResult<Vec<AwardRosterResultRow>> {
    let rows = sqlx::query(
        "SELECT slot_index, position_code, selection_group, slot_role, subject_kind, subject_id, utility, selection_score FROM award_roster_results WHERE award_instance_id = ? ORDER BY slot_index",
    )
    .bind(instance_id.to_string())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok(AwardRosterResultRow {
                slot_index: u32::try_from(row.try_get::<i64, _>("slot_index")?).map_err(|_| {
                    crate::error::PersistenceError::InvalidData("roster slot index".into())
                })?,
                position_code: row.try_get("position_code")?,
                selection_group: row.try_get("selection_group")?,
                slot_role: row.try_get("slot_role")?,
                subject_kind: row.try_get("subject_kind")?,
                subject_id: Uuid::parse_str(row.try_get::<&str, _>("subject_id")?)?,
                utility: row.try_get("utility")?,
                selection_score: row.try_get("selection_score")?,
            })
        })
        .collect()
}

pub async fn persist_roster_resolution(
    tx: &mut Transaction<'_, Sqlite>,
    resolution: &AwardRosterResolution,
    definition: &AwardDefinition,
    organizer_league_id: Option<Uuid>,
) -> PersistenceResult<Uuid> {
    let (instance_id, inserted) = insert_instance(
        tx,
        definition,
        &resolution.period_key,
        resolution.scope_id,
        resolution.seed,
        resolution.model_version,
        organizer_league_id,
    )
    .await?;
    if !inserted {
        return Ok(instance_id);
    }
    let kind = recipient_kind_code(definition.recipient_kind);
    for selection in &resolution.selections {
        if let (Some(usage_score), Some(evidence_score)) =
            (selection.usage_score, selection.evidence_score)
        {
            sqlx::query("INSERT INTO award_roster_instance_slots (award_instance_id, slot_index, position_code, usage_score, evidence_score) VALUES (?, ?, ?, ?, ?)")
                .bind(instance_id.to_string())
                .bind(i64::from(selection.slot.slot_index))
                .bind(&selection.slot.position_code)
                .bind(usage_score)
                .bind(evidence_score)
                .execute(&mut **tx)
                .await?;
        }
        for candidate in &selection.candidates {
            sqlx::query(
                "INSERT INTO award_roster_candidates (award_instance_id, slot_index, subject_kind, subject_id, utility, selection_score, final_rank) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(instance_id.to_string())
            .bind(i64::from(selection.slot.slot_index))
            .bind(kind)
            .bind(candidate.subject_id.to_string())
            .bind(candidate.utility)
            .bind(candidate.selection_score)
            .bind(candidate.rank as i64)
            .execute(&mut **tx)
            .await?;
        }
        for electorate in &selection.electorates {
            sqlx::query(
                "INSERT INTO award_roster_electorates (award_instance_id, slot_index, group_code, voter_count, result_weight) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(instance_id.to_string())
            .bind(i64::from(selection.slot.slot_index))
            .bind(&electorate.group_code)
            .bind(i64::from(electorate.voter_count))
            .bind(electorate.result_weight)
            .execute(&mut **tx)
            .await?;
            for (index, subject_id) in electorate.subject_ids.iter().enumerate() {
                sqlx::query(
                    "INSERT INTO award_roster_vote_tallies (award_instance_id, slot_index, group_code, subject_id, points, first_place_votes) VALUES (?, ?, ?, ?, ?, ?)",
                )
                .bind(instance_id.to_string())
                .bind(i64::from(selection.slot.slot_index))
                .bind(&electorate.group_code)
                .bind(subject_id.to_string())
                .bind(electorate.points[index] as i64)
                .bind(i64::from(electorate.first_place[index]))
                .execute(&mut **tx)
                .await?;
            }
        }
        sqlx::query(
            "INSERT INTO award_nominees (award_instance_id, subject_kind, subject_id, utility, selection_score, final_rank) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(instance_id.to_string())
        .bind(kind)
        .bind(selection.result.subject_id.to_string())
        .bind(selection.result.utility)
        .bind(selection.result.selection_score)
        .bind(i64::from(selection.slot.slot_index))
        .execute(&mut **tx)
        .await?;
        sqlx::query(
            "INSERT INTO award_roster_results (award_instance_id, slot_index, position_code, selection_group, slot_role, subject_kind, subject_id, utility, selection_score) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(instance_id.to_string())
        .bind(i64::from(selection.slot.slot_index))
        .bind(&selection.slot.position_code)
        .bind(&selection.slot.selection_group)
        .bind(&selection.slot.slot_role)
        .bind(kind)
        .bind(selection.result.subject_id.to_string())
        .bind(selection.result.utility)
        .bind(selection.result.selection_score)
        .execute(&mut **tx)
        .await?;
    }
    Ok(instance_id)
}
