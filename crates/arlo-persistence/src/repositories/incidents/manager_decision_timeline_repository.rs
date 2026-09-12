use crate::error::PersistenceResult;
use crate::models::{
    MatchChallengeRow, MatchPlayCallSelectionRow, MatchTacticalProfileActivationRow,
    MatchTimeCallRow,
};
use sqlx::{Sqlite, Transaction};

pub async fn insert_time_call(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTimeCallRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_time_calls (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            remaining_time_calls_after,
            reason
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(row.remaining_time_calls_after)
    .bind(&row.reason)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_time_calls_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTimeCallRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_time_call(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_challenge(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchChallengeRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_challenges (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            call_kind,
            success,
            remaining_challenges_after
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(&row.call_kind)
    .bind(row.success)
    .bind(row.remaining_challenges_after)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_challenges_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchChallengeRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_challenge(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_tactical_profile_activation(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTacticalProfileActivationRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_tactical_profile_activations (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            profile_id,
            profile_name
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(&row.profile_id)
    .bind(&row.profile_name)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_tactical_profile_activations_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTacticalProfileActivationRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_tactical_profile_activation(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_play_call_selection(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayCallSelectionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_play_call_selections (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            play_call_id,
            play_call_name,
            category
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(&row.play_call_id)
    .bind(&row.play_call_name)
    .bind(&row.category)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_play_call_selections_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayCallSelectionRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_play_call_selection(tx, row).await?;
    }
    Ok(())
}
