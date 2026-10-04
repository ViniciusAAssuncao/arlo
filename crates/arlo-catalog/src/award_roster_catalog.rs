use crate::award_catalog::AwardCatalogError;
use arlo_domain::{AwardCriterion, AwardNormalization, AwardRosterSlot};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub(crate) async fn load_roster_slots(
    pool: &SqlitePool,
    definition_id: Uuid,
) -> Result<Vec<AwardRosterSlot>, AwardCatalogError> {
    let rows = sqlx::query(
        "SELECT slot_index, position_code, selection_group, slot_role, selection_profile_id FROM award_roster_slots WHERE award_definition_id = ? ORDER BY slot_index",
    )
    .bind(definition_id.to_string())
    .fetch_all(pool)
    .await?;
    let mut slots = Vec::with_capacity(rows.len());
    for row in rows {
        let criteria = match row.try_get::<Option<String>, _>("selection_profile_id")? {
            Some(profile_id) => {
                let criteria = load_profile_criteria(pool, &profile_id).await?;
                if criteria.is_empty() {
                    return Err(AwardCatalogError::InvalidValue(
                        "selection profile has no criteria".into(),
                    ));
                }
                criteria
            }
            None => Vec::new(),
        };
        slots.push(AwardRosterSlot {
            slot_index: u32::try_from(row.try_get::<i64, _>("slot_index")?)
                .map_err(|_| AwardCatalogError::InvalidValue("slot_index".into()))?,
            position_code: row.try_get("position_code")?,
            selection_group: row.try_get("selection_group")?,
            slot_role: row.try_get("slot_role")?,
            criteria,
        });
    }
    Ok(slots)
}

async fn load_profile_criteria(
    pool: &SqlitePool,
    profile_id: &str,
) -> Result<Vec<AwardCriterion>, AwardCatalogError> {
    let rows = sqlx::query(
        "SELECT key, weight, normalization_scope FROM award_selection_profile_criteria WHERE profile_id = ? ORDER BY key",
    )
    .bind(profile_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            let normalization = match row.try_get::<&str, _>("normalization_scope")? {
                "Global" => AwardNormalization::Global,
                "Position" => AwardNormalization::Position,
                "PositionFamily" => AwardNormalization::PositionFamily,
                "CandidatePool" => AwardNormalization::CandidatePool,
                "Competition" => AwardNormalization::Competition,
                other => return Err(AwardCatalogError::InvalidValue(other.into())),
            };
            Ok(AwardCriterion {
                key: row.try_get("key")?,
                weight: row.try_get("weight")?,
                normalization,
            })
        })
        .collect()
}
