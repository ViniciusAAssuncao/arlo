use crate::award_catalog::AwardCatalogError;
use arlo_domain::{AwardCriterionPreference, AwardSelectionPolicy, AwardVoterGroup};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub(crate) async fn load_selection(
    pool: &SqlitePool,
    definition_id: Uuid,
    kind: &str,
    temperature: Option<f64>,
) -> Result<AwardSelectionPolicy, AwardCatalogError> {
    match kind {
        "Utility" => Ok(AwardSelectionPolicy::Utility {
            temperature: temperature
                .ok_or_else(|| AwardCatalogError::InvalidValue("selection_temperature".into()))?,
        }),
        "RankedVoting" => {
            let points_rows = sqlx::query(
                "SELECT points FROM award_ballot_points WHERE award_definition_id = ? ORDER BY rank"
            ).bind(definition_id.to_string()).fetch_all(pool).await?;
            let ballot_points = points_rows
                .into_iter()
                .map(|row| {
                    u32::try_from(row.try_get::<i64, _>("points")?)
                        .map_err(|_| AwardCatalogError::InvalidValue("ballot points".into()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let group_rows = sqlx::query(
                "SELECT code, voter_count, result_weight FROM award_voter_groups WHERE award_definition_id = ? ORDER BY code"
            ).bind(definition_id.to_string()).fetch_all(pool).await?;
            let mut groups = Vec::with_capacity(group_rows.len());
            for group in group_rows {
                let code: String = group.try_get("code")?;
                let preference_rows = sqlx::query(
                    "SELECT criterion_key, multiplier FROM award_voter_preferences WHERE award_definition_id = ? AND group_code = ? ORDER BY criterion_key"
                ).bind(definition_id.to_string()).bind(&code).fetch_all(pool).await?;
                let criterion_preferences = preference_rows
                    .into_iter()
                    .map(|row| {
                        Ok(AwardCriterionPreference {
                            criterion_key: row.try_get("criterion_key")?,
                            multiplier: row.try_get("multiplier")?,
                        })
                    })
                    .collect::<Result<Vec<_>, sqlx::Error>>()?;
                groups.push(AwardVoterGroup {
                    code,
                    voter_count: u32::try_from(group.try_get::<i64, _>("voter_count")?)
                        .map_err(|_| AwardCatalogError::InvalidValue("voter_count".into()))?,
                    result_weight: group.try_get("result_weight")?,
                    criterion_preferences,
                });
            }
            Ok(AwardSelectionPolicy::RankedVoting {
                ballot_points,
                groups,
            })
        }
        other => Err(AwardCatalogError::InvalidValue(other.into())),
    }
}
