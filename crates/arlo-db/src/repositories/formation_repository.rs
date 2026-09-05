use crate::error::DbResult;
use crate::models::position_code::position_to_code;
use crate::models::{FormationRow, FormationSlotRow};
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::Formation;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Formation>> {
    let row = fetch_optional_by_param::<FormationRow>(
        pool,
        "SELECT id, name FROM formations WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let slot_rows = sqlx::query_as::<_, FormationSlotRow>(
        "SELECT id, formation_id, slot_index, position, pitch_length_ratio, pitch_width_ratio, slot_role FROM formation_slots WHERE formation_id = ? ORDER BY slot_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let mut slots = Vec::with_capacity(slot_rows.len());
    for sr in slot_rows {
        slots.push(sr.to_domain()?);
    }

    Ok(Some(row.to_domain(slots)?))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Formation>> {
    let rows = fetch_all::<FormationRow>(pool, "SELECT id, name FROM formations").await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let slot_rows = sqlx::query_as::<_, FormationSlotRow>(
            "SELECT id, formation_id, slot_index, position, pitch_length_ratio, pitch_width_ratio, slot_role FROM formation_slots WHERE formation_id = ? ORDER BY slot_index ASC",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        let mut slots = Vec::with_capacity(slot_rows.len());
        for sr in slot_rows {
            slots.push(sr.to_domain()?);
        }

        results.push(row.to_domain(slots)?);
    }
    Ok(results)
}

pub async fn insert(pool: &SqlitePool, formation: &Formation) -> DbResult<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO formations (id, name) VALUES (?, ?)")
        .bind(formation.id().to_string())
        .bind(formation.name())
        .execute(&mut *tx)
        .await?;

    for (index, slot) in formation.slots().iter().enumerate() {
        let slot_id = Uuid::new_v4().to_string();
        let pos_code = position_to_code(slot.position());
        sqlx::query(
            "INSERT INTO formation_slots (id, formation_id, slot_index, position, pitch_length_ratio, pitch_width_ratio, slot_role) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(slot_id)
        .bind(formation.id().to_string())
        .bind(index as i32)
        .bind(pos_code)
        .bind(slot.pitch_length_ratio())
        .bind(slot.pitch_width_ratio())
        .bind("Standard")
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Uuid) -> DbResult<()> {
    sqlx::query("DELETE FROM formations WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
