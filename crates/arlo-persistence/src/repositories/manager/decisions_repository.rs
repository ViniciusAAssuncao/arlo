use crate::error::PersistenceResult;
use crate::models::{
    MatchManagerDecisionRow, MatchManagerPlayCallByCategoryRow, MatchManagerSubstitutionByReasonRow,
};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerDecisionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_decisions (
            id,
            match_id,
            team_id,
            substitutions_made,
            time_calls_used,
            challenges_won,
            challenges_lost,
            tactical_profile_switches
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.substitutions_made)
    .bind(row.time_calls_used)
    .bind(row.challenges_won)
    .bind(row.challenges_lost)
    .bind(row.tactical_profile_switches)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerDecisionRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_substitutions_by_reason(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerSubstitutionByReasonRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_substitutions_by_reason (
            id,
            match_id,
            team_id,
            reason,
            substitutions_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.reason)
    .bind(row.substitutions_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_substitutions_by_reason_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerSubstitutionByReasonRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_substitutions_by_reason(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_play_calls_by_category(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerPlayCallByCategoryRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_play_calls_by_category (
            id,
            match_id,
            team_id,
            category,
            play_calls_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.category)
    .bind(row.play_calls_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_play_calls_by_category_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerPlayCallByCategoryRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_play_calls_by_category(tx, row).await?;
    }
    Ok(())
}