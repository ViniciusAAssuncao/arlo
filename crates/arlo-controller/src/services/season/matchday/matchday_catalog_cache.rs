use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{
    AttributeDefinition, AttributeKey, FaultCatalog, FaultPunishmentOption, InjuryCatalog,
    InjuryDefinition,
    InjurySeverityGrade,
};
use sqlx::{Row, SqlitePool};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchdayCatalogs {
    pub fault_catalog: Arc<FaultCatalog>,
    pub injury_catalog: Arc<InjuryCatalog>,
    pub attribute_keys_by_id: Arc<HashMap<Uuid, AttributeKey>>,
    pub attribute_definitions_by_id: Arc<HashMap<Uuid, AttributeDefinition>>,
}

static MATCHDAY_CATALOGS: LazyLock<RwLock<Option<Arc<MatchdayCatalogs>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_matchday_catalogs(
    pool: &SqlitePool,
) -> ControllerResult<Arc<MatchdayCatalogs>> {
    {
        let read_guard = MATCHDAY_CATALOGS.read().await;
        if let Some(catalogs) = read_guard.as_ref() {
            return Ok(catalogs.clone());
        }
    }

    let fault_defs = arlo_db::repositories::fault_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut all_options: Vec<FaultPunishmentOption> = Vec::new();
    for def in &fault_defs {
        let options = arlo_db::repositories::fault_punishment_option::list_by_fault_definition_id(
            pool,
            def.id(),
        )
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        all_options.extend(options);
    }

    let activations = arlo_db::repositories::fault_activation::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let fault_catalog = Arc::new(FaultCatalog::new(fault_defs, all_options).with_activations(activations));

    let injury_defs: Vec<InjuryDefinition> =
        arlo_db::repositories::injury_definition::list_all(pool)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let withdrawal_rows = sqlx::query(
        "SELECT injury_definition_id, severity_grade FROM injury_recovery_profiles GROUP BY injury_definition_id, severity_grade HAVING MIN(mandatory_withdrawal) = 1",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
    let mut withdrawal_rules = HashSet::new();
    for row in withdrawal_rows {
        let id: String = row.try_get("injury_definition_id")
            .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        let grade: String = row.try_get("severity_grade")
            .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        let grade = match grade.as_str() {
            "Grade1" => InjurySeverityGrade::Grade1,
            "Grade2" => InjurySeverityGrade::Grade2,
            "Grade3" => InjurySeverityGrade::Grade3,
            _ => return Err(ControllerError::InvalidData(format!("Invalid injury grade {grade}"))),
        };
        withdrawal_rules.insert((Uuid::parse_str(&id).map_err(|error| ControllerError::InvalidData(error.to_string()))?, grade));
    }
    let injury_catalog = Arc::new(InjuryCatalog::new(injury_defs).with_mandatory_withdrawals(withdrawal_rules));

    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut keys_by_id = HashMap::with_capacity(attr_defs.len());
    let mut defs_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        keys_by_id.insert(def.id(), def.key());
        defs_by_id.insert(def.id(), def);
    }

    let catalogs = Arc::new(MatchdayCatalogs {
        fault_catalog,
        injury_catalog,
        attribute_keys_by_id: Arc::new(keys_by_id),
        attribute_definitions_by_id: Arc::new(defs_by_id),
    });

    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(catalogs.clone());
    Ok(catalogs)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<MatchdayCatalogs>> {
    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    *write_guard = None;
    drop(write_guard);
    get_or_load_matchday_catalogs(pool).await
}

pub async fn invalidate() {
    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    *write_guard = None;
}
