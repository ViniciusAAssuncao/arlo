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
    profile_days_for_roll(profile, age_years, natural_fitness, rand::thread_rng().gen())
}

fn profile_days_for_roll(
    profile: &RecoveryProfile,
    age_years: f64,
    natural_fitness: f64,
    roll: f64,
) -> u32 {
    let min = profile.minimum_days.max(1) as f64;
    let mode = profile.typical_days.max(profile.minimum_days) as f64;
    let max = profile.maximum_days.max(profile.typical_days) as f64;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::injury_recovery::register_injury;
    use crate::tuning::RecoveryTuningProfile;
    use arlo_domain::{BodyRegion, InjurySeverityGrade};

    #[tokio::test]
    async fn forced_grade_three_acl_respects_profile_and_athlete_condition() {
        let definition_id = Uuid::new_v4();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE injury_definitions (id TEXT PRIMARY KEY, code TEXT NOT NULL)")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO injury_definitions (id, code) VALUES (?, 'KNEE_ACL_TEAR_CONTACT')")
            .bind(definition_id.to_string()).execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../migrations/0100_create_injury_recovery_profiles.sql"))
            .execute(&pool).await.unwrap();
        let profiles = load_recovery_profiles(&pool).await.unwrap();
        let profile = profile_for(&profiles, definition_id, InjurySeverityGrade::Grade3, TreatmentKind::Surgical).unwrap();
        assert!(profile.mandatory_withdrawal);
        assert_eq!(profile.injury_extent.as_deref(), Some("Complete"));
        let tuning = RecoveryTuningProfile::default();
        for (age, fitness) in [(22.0, 19.0), (35.0, 3.0)] {
            let (injury, treatment) = register_injury(
                Uuid::new_v4(), definition_id, BodyRegion::Knee,
                InjurySeverityGrade::Grade3, fitness, age, &tuning, &profiles,
            ).unwrap();
            assert_eq!(injury.severity_grade(), InjurySeverityGrade::Grade3);
            let selected = profile_for(&profiles, definition_id, InjurySeverityGrade::Grade3, treatment).unwrap();
            assert!((selected.minimum_days as u32..=selected.maximum_days as u32).contains(&injury.days_remaining()));
        }
        let fit = profile_days_for_roll(profile, 26.0, 19.0, 0.5);
        let less_fit = profile_days_for_roll(profile, 26.0, 3.0, 0.5);
        let older = profile_days_for_roll(profile, 35.0, 19.0, 0.5);
        assert!(fit < less_fit);
        assert!(fit < older);
        println!("Grade 3 ACL, same surgical profile and draw: age 26 fitness 19 = {fit} days; age 26 fitness 3 = {less_fit} days; age 35 fitness 19 = {older} days");
    }
}
