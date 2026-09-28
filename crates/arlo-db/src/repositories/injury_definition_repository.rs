use crate::error::{DbError, DbResult};
use crate::models::injury_mechanism_code::injury_mechanism_to_code;
use crate::models::InjuryDefinitionRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::{InjuryDefinition, InjuryMechanism};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<InjuryDefinition>> {
    let row = fetch_optional_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn get_by_code(pool: &SqlitePool, code: &str) -> DbResult<Option<InjuryDefinition>> {
    let row = fetch_optional_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE code = ?",
        code,
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<InjuryDefinition>> {
    let rows = fetch_all::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_match_eligible(pool: &SqlitePool) -> DbResult<Vec<InjuryDefinition>> {
    let rows = fetch_all::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE id NOT IN (SELECT injury_definition_id FROM outside_match_injury_definitions) AND EXISTS (SELECT 1 FROM injury_recovery_profiles WHERE injury_definition_id = injury_definitions.id)",
    )
    .await?;
    rows.iter().map(InjuryDefinitionRow::to_domain).collect()
}

pub async fn validate_recovery_profile_coverage(pool: &SqlitePool) -> DbResult<()> {
    let uncovered_definition: Option<String> = sqlx::query_scalar(
        "SELECT code FROM injury_definitions d WHERE NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = d.id) LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    if let Some(code) = uncovered_definition {
        return Err(DbError::InvalidData(format!(
            "Missing injury recovery profile for {code}"
        )));
    }
    let outside_table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'outside_match_injury_definitions')",
    )
    .fetch_one(pool)
    .await?;
    if outside_table_exists {
        let uncovered_outside_grade: Option<String> = sqlx::query_scalar(
            "SELECT d.code FROM outside_match_injury_definitions o JOIN injury_definitions d ON d.id = o.injury_definition_id WHERE NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = o.injury_definition_id AND p.severity_grade = o.severity_grade) LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;
        if let Some(code) = uncovered_outside_grade {
            return Err(DbError::InvalidData(format!(
                "Missing outside-match injury grade profile for {code}"
            )));
        }
    }
    if outside_table_exists {
        let unsupported_named_grade: Option<String> = sqlx::query_scalar(
            r#"
            SELECT d.code FROM injury_definitions d
            WHERE d.id NOT IN (SELECT injury_definition_id FROM outside_match_injury_definitions)
              AND (
                ((instr(d.code, '_GRADE1') > 0 OR instr(d.code, '_MILD') > 0)
                  AND NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = d.id AND p.severity_grade = 'Grade1'))
                OR ((instr(d.code, '_GRADE2') > 0 OR instr(d.code, '_MODERATE') > 0)
                  AND NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = d.id AND p.severity_grade = 'Grade2'))
                OR ((instr(d.code, '_GRADE3') > 0 OR instr(d.code, '_SEVERE') > 0)
                  AND NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = d.id AND p.severity_grade = 'Grade3'))
                OR (instr(d.code, 'ACL_TEAR') > 0
                  AND NOT EXISTS (SELECT 1 FROM injury_recovery_profiles p WHERE p.injury_definition_id = d.id AND p.severity_grade IN ('Grade2', 'Grade3')))
              )
            LIMIT 1
            "#,
        )
        .fetch_optional(pool)
        .await?;
        if let Some(code) = unsupported_named_grade {
            return Err(DbError::InvalidData(format!(
                "Missing named match injury grade profile for {code}"
            )));
        }
        let unsupported_grade_floor: Option<String> = sqlx::query_scalar(
            r#"
            SELECT d.code
            FROM match_injury_grade_floors f
            JOIN injury_definitions d ON d.id = f.injury_definition_id
            WHERE NOT EXISTS (
                SELECT 1 FROM injury_recovery_profiles p
                WHERE p.injury_definition_id = d.id
                  AND (CASE p.severity_grade WHEN 'Grade1' THEN 1 WHEN 'Grade2' THEN 2 ELSE 3 END)
                    >= (CASE f.minimum_grade WHEN 'Grade1' THEN 1 WHEN 'Grade2' THEN 2 ELSE 3 END)
            )
            LIMIT 1
            "#,
        )
        .fetch_optional(pool)
        .await?;
        if let Some(code) = unsupported_grade_floor {
            return Err(DbError::InvalidData(format!(
                "Missing permitted match injury grade profile for {code}"
            )));
        }
    }
    Ok(())
}

pub async fn list_by_mechanism(
    pool: &SqlitePool,
    mechanism: InjuryMechanism,
) -> DbResult<Vec<InjuryDefinition>> {
    let mech_str = injury_mechanism_to_code(mechanism);
    let rows = fetch_all_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE mechanism = ?",
        mech_str,
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
