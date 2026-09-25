use arlo_domain::InjurySeverityGrade;
use rand::Rng;
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreatmentKind {
    Conservative,
    Surgical,
}

impl TreatmentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "Conservative",
            Self::Surgical => "Surgical",
        }
    }

    pub fn from_str(value: &str) -> Self {
        if value == "Surgical" {
            Self::Surgical
        } else {
            Self::Conservative
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct RecoveryProfile {
    pub injury_definition_id: String,
    pub severity_grade: String,
    pub injury_extent: Option<String>,
    pub treatment_kind: String,
    pub minimum_days: i32,
    pub typical_days: i32,
    pub maximum_days: i32,
    pub mandatory_withdrawal: bool,
}

pub type RecoveryProfiles = HashMap<(Uuid, String, String), RecoveryProfile>;

pub async fn load_recovery_profiles(pool: &SqlitePool) -> Result<RecoveryProfiles, sqlx::Error> {
    let rows = sqlx::query_as::<_, RecoveryProfile>(
        "SELECT injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal FROM injury_recovery_profiles",
    )
    .fetch_all(pool)
    .await?;
    let mut profiles = HashMap::with_capacity(rows.len());
    for row in rows {
        if let Ok(id) = Uuid::parse_str(&row.injury_definition_id) {
            profiles.insert(
                (id, row.severity_grade.clone(), row.treatment_kind.clone()),
                row,
            );
        }
    }
    Ok(profiles)
}

pub fn severity_code(grade: InjurySeverityGrade) -> &'static str {
    match grade {
        InjurySeverityGrade::Grade1 => "Grade1",
        InjurySeverityGrade::Grade2 => "Grade2",
        InjurySeverityGrade::Grade3 => "Grade3",
    }
}

pub fn profile_for<'a>(
    profiles: &'a RecoveryProfiles,
    definition_id: Uuid,
    grade: InjurySeverityGrade,
    treatment: TreatmentKind,
) -> Option<&'a RecoveryProfile> {
    profiles.get(&(definition_id, severity_code(grade).into(), treatment.as_str().into()))
}

pub fn choose_treatment(
    profiles: &RecoveryProfiles,
    definition_id: Uuid,
    grade: InjurySeverityGrade,
    age_years: f64,
    natural_fitness: f64,
) -> TreatmentKind {
    if profile_for(profiles, definition_id, grade, TreatmentKind::Surgical).is_none() {
        return TreatmentKind::Conservative;
    }
    if profile_for(profiles, definition_id, grade, TreatmentKind::Conservative).is_none() {
        return TreatmentKind::Surgical;
    }
    let grade_factor = if grade == InjurySeverityGrade::Grade3 { 0.70 } else { 0.18 };
    let athletic_demand = ((natural_fitness.clamp(1.0, 20.0) - 10.0) / 10.0) * 0.08;
    let age_factor = ((age_years - 27.0) / 20.0).clamp(-1.0, 1.0) * -0.06;
    let clinical_instability = rand::thread_rng().gen_range(-0.20..=0.20);
    let probability = (grade_factor + athletic_demand + age_factor + clinical_instability)
        .clamp(0.05, 0.95);
    if rand::thread_rng().gen_bool(probability) {
        TreatmentKind::Surgical
    } else {
        TreatmentKind::Conservative
    }
}

pub fn sample_profile_days(
    profile: &RecoveryProfile,
    age_years: f64,
    natural_fitness: f64,
) -> u32 {
    let min = profile.minimum_days.max(1) as f64;
    let mode = profile.typical_days.max(profile.minimum_days) as f64;
    let max = profile.maximum_days.max(profile.typical_days) as f64;
    let roll: f64 = rand::thread_rng().gen();
    let split = (mode - min) / (max - min).max(1.0);
    let sampled = if roll < split {
        min + (roll * (max - min) * (mode - min)).sqrt()
    } else {
        max - ((1.0 - roll) * (max - min) * (max - mode)).sqrt()
    };
    let fitness_adjustment = ((natural_fitness.clamp(1.0, 20.0) - 10.0) / 19.0) * -0.06;
    let age_adjustment = ((age_years - 27.0) / 40.0).clamp(-0.5, 0.5) * 0.08;
    (sampled * (1.0 + fitness_adjustment + age_adjustment))
        .round()
        .clamp(min, max) as u32
}
