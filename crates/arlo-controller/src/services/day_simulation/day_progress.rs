use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use sqlx::{FromRow, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

#[derive(FromRow)]
pub struct DayProgress {
    pub previous_year: i64,
    pub previous_day_of_year: i64,
    pub target_year: i64,
    pub target_day_of_year: i64,
    pub matches_played_count: i64,
    pub phase: String,
}

impl DayProgress {
    pub fn target(&self) -> CalendarDate {
        CalendarDate::new(self.target_year, self.target_day_of_year as u32)
    }

    pub fn previous(&self) -> CalendarDate {
        CalendarDate::new(self.previous_year, self.previous_day_of_year as u32)
    }
}

pub async fn load_or_start(
    pool: &SqlitePool,
    save_uuid: Uuid,
    previous: &CalendarDate,
    target: &CalendarDate,
) -> ControllerResult<DayProgress> {
    sqlx::query(
        "INSERT INTO day_advancement_progress (save_uuid, previous_year, previous_day_of_year, target_year, target_day_of_year, phase) VALUES (?, ?, ?, ?, ?, 'Started') ON CONFLICT(save_uuid) DO NOTHING",
    )
    .bind(save_uuid.to_string())
    .bind(previous.year())
    .bind(previous.day_of_year() as i64)
    .bind(target.year())
    .bind(target.day_of_year() as i64)
    .execute(pool)
    .await?;
    let progress = sqlx::query_as::<_, DayProgress>(
        "SELECT previous_year, previous_day_of_year, target_year, target_day_of_year, matches_played_count, phase FROM day_advancement_progress WHERE save_uuid = ?",
    )
    .bind(save_uuid.to_string())
    .fetch_one(pool)
    .await?;
    if progress.previous() != *previous || progress.target() != *target {
        return Err(ControllerError::InvalidData(
            "Day progress disagrees with the saved calendar".into(),
        ));
    }
    Ok(progress)
}

pub async fn advance_phase(
    pool: &SqlitePool,
    save_uuid: Uuid,
    from: &str,
    to: &str,
) -> ControllerResult<()> {
    let affected = sqlx::query(
        "UPDATE day_advancement_progress SET phase = ? WHERE save_uuid = ? AND phase = ?",
    )
    .bind(to)
    .bind(save_uuid.to_string())
    .bind(from)
    .execute(pool)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(ControllerError::InvalidData("Invalid day progress transition".into()));
    }
    Ok(())
}

pub async fn advance_phase_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    save_uuid: Uuid,
    from: &str,
    to: &str,
) -> ControllerResult<()> {
    let affected = sqlx::query(
        "UPDATE day_advancement_progress SET phase = ? WHERE save_uuid = ? AND phase = ?",
    )
    .bind(to)
    .bind(save_uuid.to_string())
    .bind(from)
    .execute(&mut **tx)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(ControllerError::InvalidData("Invalid day progress transition".into()));
    }
    Ok(())
}

pub async fn record_matches_done(
    tx: &mut Transaction<'_, Sqlite>,
    save_uuid: Uuid,
    count: u32,
) -> ControllerResult<()> {
    let affected = sqlx::query(
        "UPDATE day_advancement_progress SET phase = 'MatchesDone', matches_played_count = ? WHERE save_uuid = ? AND phase = 'EventsDone'",
    )
    .bind(i64::from(count))
    .bind(save_uuid.to_string())
    .execute(&mut **tx)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(ControllerError::InvalidData("Invalid matchday progress transition".into()));
    }
    Ok(())
}

pub async fn finish(
    pool: &SqlitePool,
    save_uuid: Uuid,
    target: &CalendarDate,
) -> ControllerResult<()> {
    let mut tx = pool.begin().await?;
    let affected = sqlx::query(
        "UPDATE save_calendar_states SET current_year = ?, current_day_of_year = ? WHERE save_uuid = ?",
    )
    .bind(target.year())
    .bind(target.day_of_year() as i64)
    .bind(save_uuid.to_string())
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(ControllerError::InvalidData("Save calendar state is missing".into()));
    }
    let affected = sqlx::query(
        "DELETE FROM day_advancement_progress WHERE save_uuid = ? AND phase = 'MatchesDone'",
    )
    .bind(save_uuid.to_string())
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(ControllerError::InvalidData("Day progress is incomplete".into()));
    }
    tx.commit().await?;
    Ok(())
}
