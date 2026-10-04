use crate::award_roster_catalog::{load_dynamic_position_profiles, load_roster_slots};
use crate::award_selection_catalog::load_selection;
use arlo_domain::{
    AwardCriterion, AwardDefinition, AwardDynamicRosterPolicy, AwardEvaluationWindow,
    AwardNormalization, AwardOrganization, AwardOrganizerPolicy, AwardRecipientKind,
    AwardResultKind, AwardScopeKind, AwardTieBreak, AwardTieDirection, AwardTrigger,
};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AwardCatalogError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("invalid award catalog value: {0}")]
    InvalidValue(String),
}

pub async fn get_award_by_code(
    pool: &SqlitePool,
    code: &str,
) -> Result<Option<AwardDefinition>, AwardCatalogError> {
    let row = sqlx::query("SELECT * FROM award_definitions WHERE code = ? AND active = 1")
        .bind(code)
        .fetch_optional(pool)
        .await?;
    match row {
        Some(row) => Ok(Some(load_definition(pool, &row).await?)),
        None => Ok(None),
    }
}

pub async fn get_award_organization(
    pool: &SqlitePool,
    id: Uuid,
) -> Result<Option<AwardOrganization>, AwardCatalogError> {
    let row = sqlx::query(
        "SELECT id, code, name, country_id, continent_id, league_id FROM award_organizations WHERE id = ?"
    ).bind(id.to_string()).fetch_optional(pool).await?;
    row.map(|row| {
        Ok(AwardOrganization {
            id: Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
            code: row.try_get("code")?,
            name: row.try_get("name")?,
            country_id: optional_uuid(&row, "country_id")?,
            continent_id: optional_uuid(&row, "continent_id")?,
            league_id: optional_uuid(&row, "league_id")?,
        })
    })
    .transpose()
}

pub async fn list_active_awards(
    pool: &SqlitePool,
) -> Result<Vec<AwardDefinition>, AwardCatalogError> {
    let rows = sqlx::query("SELECT * FROM award_definitions WHERE active = 1 ORDER BY code")
        .fetch_all(pool)
        .await?;
    let mut definitions = Vec::with_capacity(rows.len());
    for row in &rows {
        definitions.push(load_definition(pool, row).await?);
    }
    Ok(definitions)
}

async fn load_definition(
    pool: &SqlitePool,
    row: &sqlx::sqlite::SqliteRow,
) -> Result<AwardDefinition, AwardCatalogError> {
    let id = Uuid::parse_str(row.try_get::<&str, _>("id")?)?;
    let criteria_rows = sqlx::query(
        "SELECT key, weight, normalization_scope FROM award_criteria WHERE award_definition_id = ? ORDER BY key"
    ).bind(id.to_string()).fetch_all(pool).await?;
    let mut criteria = Vec::with_capacity(criteria_rows.len());
    for criterion in criteria_rows {
        let scope: &str = criterion.try_get("normalization_scope")?;
        let normalization = match scope {
            "Global" => AwardNormalization::Global,
            "Position" => AwardNormalization::Position,
            "PositionFamily" => AwardNormalization::PositionFamily,
            "CandidatePool" => AwardNormalization::CandidatePool,
            "Competition" => AwardNormalization::Competition,
            other => return Err(AwardCatalogError::InvalidValue(other.into())),
        };
        criteria.push(AwardCriterion {
            key: criterion.try_get("key")?,
            weight: criterion.try_get("weight")?,
            normalization,
        });
    }
    let kind: &str = row.try_get("recipient_kind")?;
    let recipient_kind = match kind {
        "Player" => AwardRecipientKind::Player,
        "Team" => AwardRecipientKind::Team,
        "Manager" => AwardRecipientKind::Manager,
        "Referee" => AwardRecipientKind::Referee,
        "Person" => AwardRecipientKind::Person,
        "Federation" => AwardRecipientKind::Federation,
        other => return Err(AwardCatalogError::InvalidValue(other.into())),
    };
    let scope: &str = row.try_get("scope_kind")?;
    let scope = match scope {
        "Global" => AwardScopeKind::Global,
        "Competition" => AwardScopeKind::Competition,
        "Team" => AwardScopeKind::Team,
        "Player" => AwardScopeKind::Player,
        other => return Err(AwardCatalogError::InvalidValue(other.into())),
    };
    Ok(AwardDefinition {
        id,
        code: row.try_get("code")?,
        name: row.try_get("name")?,
        short_name: row.try_get("short_name")?,
        organizer_id: row
            .try_get::<Option<String>, _>("organizer_id")?
            .map(|value| Uuid::parse_str(&value))
            .transpose()?,
        organizer_policy: match row
            .try_get::<Option<&str>, _>("organizer_policy_override")?
            .unwrap_or(row.try_get::<&str, _>("organizer_policy")?)
        {
            "DefinedOrganization" => AwardOrganizerPolicy::DefinedOrganization,
            "LeagueCommittee" => AwardOrganizerPolicy::LeagueCommittee,
            "FederationCommittee" => AwardOrganizerPolicy::FederationCommittee,
            other => return Err(AwardCatalogError::InvalidValue(other.into())),
        },
        recipient_kind,
        prestige: row.try_get("prestige")?,
        scope,
        result_kind: match row.try_get::<&str, _>("result_kind")? {
            "SingleWinner" => AwardResultKind::SingleWinner,
            "Roster" => AwardResultKind::Roster,
            other => return Err(AwardCatalogError::InvalidValue(other.into())),
        },
        trigger: serde_json::from_str::<AwardTrigger>(row.try_get("trigger_policy")?)?,
        evaluation_window: serde_json::from_str::<AwardEvaluationWindow>(
            row.try_get("evaluation_window")?,
        )?,
        announcement_month_order_index: optional_u32(row, "announcement_month_order_index")?,
        announcement_day_of_month: optional_u32(row, "announcement_day_of_month")?,
        announcement_delay_days: read_positive_u32(row.try_get("announcement_delay_days")?)?,
        minimum_competition_prestige: optional_u32(row, "minimum_competition_prestige")?,
        dynamic_roster: optional_u32(row, "dynamic_roster_size")?
            .map(|slot_count| {
                Ok::<AwardDynamicRosterPolicy, AwardCatalogError>(AwardDynamicRosterPolicy {
                    slot_count,
                    minimum_position_seconds: row
                        .try_get::<Option<f64>, _>("dynamic_minimum_position_seconds")?
                        .ok_or_else(|| {
                            AwardCatalogError::InvalidValue(
                                "dynamic_minimum_position_seconds".into(),
                            )
                        })?,
                    minimum_position_candidates: optional_u32(
                        row,
                        "dynamic_minimum_position_candidates",
                    )?
                    .ok_or_else(|| {
                        AwardCatalogError::InvalidValue(
                            "dynamic_minimum_position_candidates".into(),
                        )
                    })?,
                    minimum_utility: row
                        .try_get::<Option<f64>, _>("dynamic_minimum_utility")?
                        .ok_or_else(|| {
                            AwardCatalogError::InvalidValue("dynamic_minimum_utility".into())
                        })?,
                })
            })
            .transpose()?,
        dynamic_position_profiles: load_dynamic_position_profiles(pool, id).await?,
        eligible_positions: list_strings(pool, "award_eligibility_positions", "position_code", id)
            .await?,
        minimum_age: row
            .try_get::<Option<i64>, _>("minimum_age")?
            .map(|age| {
                u32::try_from(age)
                    .map_err(|_| AwardCatalogError::InvalidValue("minimum_age".into()))
            })
            .transpose()?,
        maximum_age: row
            .try_get::<Option<i64>, _>("maximum_age")?
            .map(|age| {
                u32::try_from(age)
                    .map_err(|_| AwardCatalogError::InvalidValue("maximum_age".into()))
            })
            .transpose()?,
        eligible_countries: list_uuids(pool, "award_eligibility_countries", "country_id", id)
            .await?,
        eligible_continents: list_uuids(pool, "award_eligibility_continents", "continent_id", id)
            .await?,
        eligible_competitions: list_uuids(
            pool,
            "award_eligibility_competitions",
            "competition_id",
            id,
        )
        .await?,
        minimum_matches: read_positive_u32(row.try_get("minimum_matches")?)?,
        nomination_limit: row
            .try_get::<Option<i64>, _>("nomination_limit")?
            .map(|value| {
                usize::try_from(value)
                    .map_err(|_| AwardCatalogError::InvalidValue("nomination_limit".into()))
            })
            .transpose()?,
        criteria,
        roster_slots: load_roster_slots(pool, id).await?,
        tie_breaks: load_tie_breaks(pool, id).await?,
        selection: load_selection(
            pool,
            id,
            row.try_get("selection_kind")?,
            row.try_get("selection_temperature")?,
        )
        .await?,
        active: row.try_get::<i64, _>("active")? != 0,
    })
}

async fn list_strings(
    pool: &SqlitePool,
    table: &str,
    column: &str,
    id: Uuid,
) -> Result<Vec<String>, AwardCatalogError> {
    let query =
        format!("SELECT {column} FROM {table} WHERE award_definition_id = ? ORDER BY {column}");
    let rows = sqlx::query(&query)
        .bind(id.to_string())
        .fetch_all(pool)
        .await?;
    rows.into_iter()
        .map(|row| row.try_get::<String, _>(0).map_err(Into::into))
        .collect()
}

async fn list_uuids(
    pool: &SqlitePool,
    table: &str,
    column: &str,
    id: Uuid,
) -> Result<Vec<Uuid>, AwardCatalogError> {
    list_strings(pool, table, column, id)
        .await?
        .into_iter()
        .map(|value| Uuid::parse_str(&value).map_err(Into::into))
        .collect()
}

fn read_positive_u32(value: i64) -> Result<u32, AwardCatalogError> {
    u32::try_from(value).map_err(|_| AwardCatalogError::InvalidValue("minimum_matches".into()))
}

fn optional_u32(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Option<u32>, AwardCatalogError> {
    row.try_get::<Option<i64>, _>(column)?
        .map(|value| {
            u32::try_from(value).map_err(|_| AwardCatalogError::InvalidValue(column.into()))
        })
        .transpose()
}

async fn load_tie_breaks(
    pool: &SqlitePool,
    id: Uuid,
) -> Result<Vec<AwardTieBreak>, AwardCatalogError> {
    let rows = sqlx::query(
        "SELECT metric_key, direction FROM award_tie_breaks WHERE award_definition_id = ? ORDER BY priority"
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            let direction = match row.try_get::<&str, _>("direction")? {
                "Descending" => AwardTieDirection::Descending,
                "Ascending" => AwardTieDirection::Ascending,
                other => return Err(AwardCatalogError::InvalidValue(other.into())),
            };
            Ok(AwardTieBreak {
                metric_key: row.try_get("metric_key")?,
                direction,
            })
        })
        .collect()
}

fn optional_uuid(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Option<Uuid>, AwardCatalogError> {
    row.try_get::<Option<String>, _>(column)?
        .map(|value| Uuid::parse_str(&value).map_err(Into::into))
        .transpose()
}
