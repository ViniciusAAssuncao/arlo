use crate::error::{RecoveryError, RecoveryResult};
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use rand::Rng;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OutsideMatchCondition {
    pub definition_id: Uuid,
    pub body_region: BodyRegion,
    pub severity_grade: InjurySeverityGrade,
    weight: f64,
}

#[derive(Debug, Clone, Default)]
pub struct OutsideMatchCatalog {
    conditions: Vec<OutsideMatchCondition>,
    total_weight: f64,
}

#[derive(FromRow)]
struct OutsideMatchRow {
    injury_definition_id: String,
    body_region: String,
    severity_grade: String,
    daily_weight: f64,
}

impl OutsideMatchCatalog {
    pub async fn load(pool: &SqlitePool) -> RecoveryResult<Self> {
        let rows = sqlx::query_as::<_, OutsideMatchRow>(
            "SELECT o.injury_definition_id, d.body_region, o.severity_grade, o.daily_weight FROM outside_match_injury_definitions o JOIN injury_definitions d ON d.id = o.injury_definition_id WHERE o.daily_weight > 0",
        )
        .fetch_all(pool)
        .await?;
        let mut conditions = Vec::with_capacity(rows.len());
        let mut total_weight = 0.0;
        for row in rows {
            let severity_grade = match row.severity_grade.as_str() {
                "Grade1" => InjurySeverityGrade::Grade1,
                "Grade2" => InjurySeverityGrade::Grade2,
                "Grade3" => InjurySeverityGrade::Grade3,
                _ => return Err(RecoveryError::InvalidData(row.severity_grade)),
            };
            let body_region = arlo_db::models::body_region_code::parse_body_region(&row.body_region)
                .map_err(|error| RecoveryError::InvalidData(error.to_string()))?;
            conditions.push(OutsideMatchCondition {
                definition_id: Uuid::parse_str(&row.injury_definition_id)
                    .map_err(|error| RecoveryError::InvalidData(error.to_string()))?,
                body_region,
                severity_grade,
                weight: row.daily_weight,
            });
            total_weight += row.daily_weight;
        }
        Ok(Self { conditions, total_weight })
    }

    pub fn sample(&self, probability: f64) -> Option<&OutsideMatchCondition> {
        if self.total_weight <= 0.0 || !rand::thread_rng().gen_bool(probability.clamp(0.0, 1.0)) {
            return None;
        }
        let mut draw = rand::thread_rng().gen_range(0.0..self.total_weight);
        self.conditions.iter().find(|condition| {
            draw -= condition.weight;
            draw < 0.0
        }).or_else(|| self.conditions.last())
    }
}
