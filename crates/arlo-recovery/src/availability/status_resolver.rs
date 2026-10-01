use crate::domain::{InjuryRecord, InjuryStatusKind};
use crate::error::{RecoveryError, RecoveryResult};
use arlo_domain::InjurySeverityGrade;
use arlo_persistence::models::condition::PlayerInjuryHistoryRow;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerMedicalStatus {
    pub player_id: Uuid,
    pub status: InjuryStatusKind,
    pub injury_record: Option<InjuryRecord>,
    pub expected_recovery_days: Option<u32>,
}

impl PlayerMedicalStatus {
    pub fn healthy(player_id: Uuid) -> Self {
        Self {
            player_id,
            status: InjuryStatusKind::Healthy,
            injury_record: None,
            expected_recovery_days: None,
        }
    }

    pub fn is_injured(&self) -> bool {
        self.status == InjuryStatusKind::Injured
    }

    pub fn is_available_for_selection(&self) -> bool {
        crate::injury_recovery::return_to_play_evaluator::is_available_for_selection(self.status)
    }

    pub fn display_status(&self) -> &'static str {
        match self.status {
            InjuryStatusKind::Healthy => "Healthy",
            InjuryStatusKind::Injured => "Injured",
            InjuryStatusKind::Observation => "Observation",
        }
    }

    pub fn active_days_remaining(&self) -> Option<u32> {
        match self.status {
            InjuryStatusKind::Injured => self.injury_record.as_ref().map(|r| r.days_remaining()),
            InjuryStatusKind::Observation => self
                .injury_record
                .as_ref()
                .map(|r| r.observation_days_remaining()),
            InjuryStatusKind::Healthy => None,
        }
    }

    pub fn body_region_code(&self) -> Option<&'static str> {
        self.injury_record
            .as_ref()
            .map(|r| arlo_db::models::body_region_code::body_region_to_code(r.body_region()))
    }

    pub fn severity_grade_code(&self) -> Option<&'static str> {
        self.injury_record
            .as_ref()
            .map(|r| match r.severity_grade() {
                InjurySeverityGrade::Grade1 => "Grade1",
                InjurySeverityGrade::Grade2 => "Grade2",
                InjurySeverityGrade::Grade3 => "Grade3",
            })
    }
}

pub fn injury_record_from_row(row: &PlayerInjuryHistoryRow) -> RecoveryResult<InjuryRecord> {
    let id = Uuid::parse_str(&row.id).map_err(|e| RecoveryError::InvalidData(e.to_string()))?;
    let injury_definition_id = Uuid::parse_str(&row.injury_definition_id)
        .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;
    let body_region = arlo_db::models::body_region_code::parse_body_region(&row.body_region)
        .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;
    let severity_grade = match row.severity_grade.as_str() {
        "Grade1" | "grade1" | "Grade_1" | "grade_1" | "1" => InjurySeverityGrade::Grade1,
        "Grade2" | "grade2" | "Grade_2" | "grade_2" | "2" => InjurySeverityGrade::Grade2,
        "Grade3" | "grade3" | "Grade_3" | "grade_3" | "3" => InjurySeverityGrade::Grade3,
        other => {
            return Err(RecoveryError::InvalidData(format!(
                "Invalid injury severity grade: {other}"
            )))
        }
    };
    let origin_id = row
        .origin_record_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;

    let is_relapse = row.is_relapse;
    let original_id = if is_relapse && origin_id.is_none() {
        Some(id)
    } else if !is_relapse && origin_id.is_some() {
        None
    } else {
        origin_id
    };

    let record = InjuryRecord::new(
        id,
        injury_definition_id,
        body_region,
        severity_grade,
        row.days_remaining.max(0) as u32,
        row.observation_days_remaining.max(0) as u32,
        is_relapse,
        original_id,
    )?;

    Ok(record)
}

pub fn derive_injury_status(record: Option<&InjuryRecord>) -> InjuryStatusKind {
    match record {
        Some(injury) if injury.days_remaining() > 0 => InjuryStatusKind::Injured,
        Some(injury) if injury.observation_days_remaining() > 0 => InjuryStatusKind::Observation,
        _ => InjuryStatusKind::Healthy,
    }
}

pub fn resolve_status_from_row(
    player_id: Uuid,
    row: Option<&PlayerInjuryHistoryRow>,
) -> RecoveryResult<PlayerMedicalStatus> {
    match row {
        Some(r) => {
            let record = injury_record_from_row(r)?;
            let status = derive_injury_status(Some(&record));
            Ok(PlayerMedicalStatus {
                player_id,
                status,
                injury_record: Some(record),
                expected_recovery_days: Some(r.expected_recovery_days.max(0) as u32),
            })
        }
        None => Ok(PlayerMedicalStatus::healthy(player_id)),
    }
}

pub async fn resolve_player_status(
    pool: &SqlitePool,
    player_id: Uuid,
) -> RecoveryResult<PlayerMedicalStatus> {
    let row =
        arlo_persistence::repositories::condition::player_injury_history::get_active_by_player_id(
            pool, player_id,
        )
        .await?;

    resolve_status_from_row(player_id, row.as_ref())
}
