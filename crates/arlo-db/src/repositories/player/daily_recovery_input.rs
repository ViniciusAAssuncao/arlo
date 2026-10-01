use crate::error::{DbResult};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DailyRecoveryInput {
    pub id: Uuid,
    pub birthdate_unix_seconds: i64,
    pub stamina: f64,
    pub natural_fitness: f64,
    pub determination: f64,
    pub composure: f64,
    pub consistency: f64,
}

#[derive(FromRow)]
struct DailyRecoveryInputRow {
    id: String,
    birthdate_unix_seconds: i64,
    stamina: i32,
    natural_fitness: i32,
    determination: i32,
    composure: i32,
    consistency: i32,
}

pub async fn list_daily_recovery_inputs(pool: &SqlitePool) -> DbResult<Vec<DailyRecoveryInput>> {
    let rows = sqlx::query_as::<_, DailyRecoveryInputRow>(
        "SELECT p.id, p.birthdate_unix_seconds, \
         COALESCE(st.value, 10) AS stamina, \
         COALESCE(nf.value, 10) AS natural_fitness, \
         COALESCE(de.value, 10) AS determination, \
         COALESCE(co.value, 10) AS composure, \
         COALESCE(cs.value, 10) AS consistency \
         FROM players p \
         LEFT JOIN player_attributes st ON st.player_id = p.id AND st.attribute_definition_id = (SELECT id FROM attribute_definitions WHERE key = 'stamina' AND applies_to = 'Player') \
         LEFT JOIN player_attributes nf ON nf.player_id = p.id AND nf.attribute_definition_id = (SELECT id FROM attribute_definitions WHERE key = 'natural_fitness' AND applies_to = 'Player') \
         LEFT JOIN player_attributes de ON de.player_id = p.id AND de.attribute_definition_id = (SELECT id FROM attribute_definitions WHERE key = 'determination' AND applies_to = 'Player') \
         LEFT JOIN player_attributes co ON co.player_id = p.id AND co.attribute_definition_id = (SELECT id FROM attribute_definitions WHERE key = 'composure' AND applies_to = 'Player') \
         LEFT JOIN player_attributes cs ON cs.player_id = p.id AND cs.attribute_definition_id = (SELECT id FROM attribute_definitions WHERE key = 'consistency' AND applies_to = 'Player') \
         WHERE p.team_id IS NOT NULL",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| Ok(DailyRecoveryInput {
        id: Uuid::parse_str(&row.id)?,
        birthdate_unix_seconds: row.birthdate_unix_seconds,
        stamina: f64::from(row.stamina),
        natural_fitness: f64::from(row.natural_fitness),
        determination: f64::from(row.determination),
        composure: f64::from(row.composure),
        consistency: f64::from(row.consistency),
    })).collect()
}
