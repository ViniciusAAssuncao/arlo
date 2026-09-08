use crate::error::{TacticsError, TacticsResult};
use crate::persistence::models::artro_placement_code::artro_placement_to_code;
use crate::persistence::models::decision_kind_code::artrine_decision_kind_to_code;
use crate::persistence::models::play_call_category_code::play_call_category_to_code;
use crate::persistence::models::rows::{
    situational_profile_to_pairs, PlayCallDecisionEmphasisRow, PlayCallMisdirectionLinkRow,
    PlayCallRouteAssignmentRow, PlayCallRow, PlayCallSituationalParameterRow,
};
use crate::persistence::models::situational_parameter_key_code::situational_parameter_key_to_code;
use crate::persistence::models::slot_role_code::slot_role_to_code;
use crate::playcall::PlayCall;
use arlo_db::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::ArtrineDecisionKind;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> TacticsResult<Option<PlayCall>> {
    let row = fetch_optional_by_param::<PlayCallRow>(
        pool,
        "SELECT id, team_id, tactical_lineup_id, name, category, counter_play_id, created_at_unix_seconds FROM play_calls WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let situational_rows = sqlx::query_as::<_, PlayCallSituationalParameterRow>(
        "SELECT id, play_call_id, parameter_key, value FROM play_call_situational_parameters WHERE play_call_id = ?",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let emphasis_rows = sqlx::query_as::<_, PlayCallDecisionEmphasisRow>(
        "SELECT id, play_call_id, decision_kind, weight FROM play_call_decision_emphasis WHERE play_call_id = ?",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let route_rows = sqlx::query_as::<_, PlayCallRouteAssignmentRow>(
        "SELECT id, play_call_id, slot_index, has_route, target_channel, depth_ratio, break_ratio, read_priority, role_override FROM play_call_route_assignments WHERE play_call_id = ? ORDER BY slot_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let misdirection_row = sqlx::query_as::<_, PlayCallMisdirectionLinkRow>(
        "SELECT id, play_call_id, decoy_slot_index, true_carrier_slot_index, deception_intensity FROM play_call_misdirection_links WHERE play_call_id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(Some(row.to_domain(
        &situational_rows,
        &emphasis_rows,
        &route_rows,
        misdirection_row.as_ref(),
    )?))
}

pub async fn list_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Vec<PlayCall>> {
    let rows = fetch_all_by_param::<PlayCallRow>(
        pool,
        "SELECT id, team_id, tactical_lineup_id, name, category, counter_play_id, created_at_unix_seconds FROM play_calls WHERE team_id = ? ORDER BY created_at_unix_seconds ASC",
        &team_id.to_string(),
    )
    .await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let situational_rows = sqlx::query_as::<_, PlayCallSituationalParameterRow>(
            "SELECT id, play_call_id, parameter_key, value FROM play_call_situational_parameters WHERE play_call_id = ?",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        let emphasis_rows = sqlx::query_as::<_, PlayCallDecisionEmphasisRow>(
            "SELECT id, play_call_id, decision_kind, weight FROM play_call_decision_emphasis WHERE play_call_id = ?",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        let route_rows = sqlx::query_as::<_, PlayCallRouteAssignmentRow>(
            "SELECT id, play_call_id, slot_index, has_route, target_channel, depth_ratio, break_ratio, read_priority, role_override FROM play_call_route_assignments WHERE play_call_id = ? ORDER BY slot_index ASC",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        let misdirection_row = sqlx::query_as::<_, PlayCallMisdirectionLinkRow>(
            "SELECT id, play_call_id, decoy_slot_index, true_carrier_slot_index, deception_intensity FROM play_call_misdirection_links WHERE play_call_id = ?",
        )
        .bind(&row.id)
        .fetch_optional(pool)
        .await?;

        results.push(row.to_domain(
            &situational_rows,
            &emphasis_rows,
            &route_rows,
            misdirection_row.as_ref(),
        )?);
    }

    Ok(results)
}

pub async fn insert(pool: &SqlitePool, play_call: &PlayCall) -> TacticsResult<()> {
    let mut tx = pool.begin().await?;

    if let Some(counter_play_id) = play_call.counter_play_id() {
        let counter_row = sqlx::query_as::<_, PlayCallRow>(
            "SELECT id, team_id, tactical_lineup_id, name, category, counter_play_id, created_at_unix_seconds FROM play_calls WHERE id = ?",
        )
        .bind(counter_play_id.to_string())
        .fetch_optional(&mut *tx)
        .await?;

        match counter_row {
            Some(cr) => {
                if cr.tactical_lineup_id != play_call.tactical_lineup_id().to_string() {
                    return Err(TacticsError::InvalidPlayCall(
                        "Counter play references a play from a different tactical lineup".to_string(),
                    ));
                }
            }
            None => {
                return Err(TacticsError::InvalidPlayCall(format!(
                    "Counter play {} not found",
                    counter_play_id
                )));
            }
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let category_code = play_call_category_to_code(play_call.category());
    let counter_play_str = play_call.counter_play_id().map(|id| id.to_string());

    sqlx::query(
        "INSERT INTO play_calls (id, team_id, tactical_lineup_id, name, category, counter_play_id, created_at_unix_seconds) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(play_call.id().to_string())
    .bind(play_call.team_id().to_string())
    .bind(play_call.tactical_lineup_id().to_string())
    .bind(play_call.name())
    .bind(category_code)
    .bind(counter_play_str)
    .bind(timestamp)
    .execute(&mut *tx)
    .await?;

    if let Some(profile) = play_call.situational_profile() {
        let pairs = situational_profile_to_pairs(profile);
        for (key, val) in pairs {
            let param_id = Uuid::new_v4().to_string();
            let key_code = situational_parameter_key_to_code(key);
            sqlx::query(
                "INSERT INTO play_call_situational_parameters (id, play_call_id, parameter_key, value) VALUES (?, ?, ?, ?)",
            )
            .bind(param_id)
            .bind(play_call.id().to_string())
            .bind(key_code)
            .bind(val)
            .execute(&mut *tx)
            .await?;
        }
    }

    let emphasis_entries: [(ArtrineDecisionKind, f64); 5] = [
        (
            ArtrineDecisionKind::SelfCarry,
            play_call.decision_emphasis().self_carry().value(),
        ),
        (
            ArtrineDecisionKind::ShortPass,
            play_call.decision_emphasis().short_pass().value(),
        ),
        (
            ArtrineDecisionKind::LongLaunch,
            play_call.decision_emphasis().long_launch().value(),
        ),
        (
            ArtrineDecisionKind::Cross,
            play_call.decision_emphasis().cross().value(),
        ),
        (
            ArtrineDecisionKind::SelfFinish,
            play_call.decision_emphasis().self_finish().value(),
        ),
    ];

    for (kind, weight) in emphasis_entries {
        let emp_id = Uuid::new_v4().to_string();
        let kind_code = artrine_decision_kind_to_code(kind);
        sqlx::query(
            "INSERT INTO play_call_decision_emphasis (id, play_call_id, decision_kind, weight) VALUES (?, ?, ?, ?)",
        )
        .bind(emp_id)
        .bind(play_call.id().to_string())
        .bind(kind_code)
        .bind(weight)
        .execute(&mut *tx)
        .await?;
    }

    let mut role_override_map = HashMap::new();
    for (slot_idx, role) in play_call.role_overrides() {
        role_override_map.insert(*slot_idx, *role);
    }

    let mut processed_slots = HashSet::new();

    for route in play_call.routes() {
        let route_id = Uuid::new_v4().to_string();
        let slot_idx = route.slot_index();
        processed_slots.insert(slot_idx);

        let target_channel = artro_placement_to_code(route.target_channel());
        let role_override_code = role_override_map.get(&slot_idx).map(|r| slot_role_to_code(*r));

        sqlx::query(
            "INSERT INTO play_call_route_assignments (id, play_call_id, slot_index, has_route, target_channel, depth_ratio, break_ratio, read_priority, role_override) VALUES (?, ?, ?, 1, ?, ?, ?, ?, ?)",
        )
        .bind(route_id)
        .bind(play_call.id().to_string())
        .bind(slot_idx as i32)
        .bind(target_channel)
        .bind(route.depth_ratio())
        .bind(route.break_ratio())
        .bind(route.read_priority().value())
        .bind(role_override_code)
        .execute(&mut *tx)
        .await?;
    }

    for (slot_idx, role) in play_call.role_overrides() {
        if !processed_slots.contains(slot_idx) {
            let route_id = Uuid::new_v4().to_string();
            let role_override_code = slot_role_to_code(*role);

            sqlx::query(
                "INSERT INTO play_call_route_assignments (id, play_call_id, slot_index, has_route, target_channel, depth_ratio, break_ratio, read_priority, role_override) VALUES (?, ?, ?, 0, NULL, NULL, NULL, NULL, ?)",
            )
            .bind(route_id)
            .bind(play_call.id().to_string())
            .bind(*slot_idx as i32)
            .bind(role_override_code)
            .execute(&mut *tx)
            .await?;
        }
    }

    if let Some(m) = play_call.misdirection() {
        let link_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO play_call_misdirection_links (id, play_call_id, decoy_slot_index, true_carrier_slot_index, deception_intensity) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(link_id)
        .bind(play_call.id().to_string())
        .bind(m.decoy_slot_index() as i32)
        .bind(m.true_carrier_slot_index() as i32)
        .bind(m.deception_intensity().value())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Uuid) -> TacticsResult<()> {
    sqlx::query("DELETE FROM play_calls WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
