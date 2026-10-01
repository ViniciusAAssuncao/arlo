use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{Formation, Position};
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn ensure_minimum_roster(
    pool: &SqlitePool,
    team_id: Uuid,
    formation: &Formation,
    available_player_count: usize,
    existing_players_count: usize,
) -> ControllerResult<()> {
    let required_count = formation.slots().len().max(14);
    if available_player_count >= required_count {
        return Ok(());
    }

    let needed_count = required_count - available_player_count;

    let team = arlo_db::repositories::team::get_by_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let country_id = match team {
        Some(t) => t.country_id(),
        None => {
            let countries = arlo_db::repositories::country::list_all(pool)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            countries
                .into_iter()
                .next()
                .map(|c| c.id())
                .unwrap_or_else(Uuid::new_v4)
        }
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    #[derive(sqlx::FromRow)]
    struct ColumnInfo {
        name: String,
    }

    let player_cols: Vec<ColumnInfo> = sqlx::query_as("PRAGMA table_info(players)")
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    let player_col_names: HashSet<String> = player_cols.into_iter().map(|c| c.name).collect();

    let pos_cols: Vec<ColumnInfo> = sqlx::query_as("PRAGMA table_info(player_positions)")
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    let pos_col_names: HashSet<String> = pos_cols.into_iter().map(|c| c.name).collect();

    let attr_cols: Vec<ColumnInfo> = sqlx::query_as("PRAGMA table_info(player_attributes)")
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    let attr_col_names: HashSet<String> = attr_cols.into_iter().map(|c| c.name).collect();

    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .unwrap_or_default();

    let mut tx = pool.begin().await?;

    for i in 0..needed_count {
        let player_id = Uuid::new_v4();
        let first_name = "Jogador";
        let last_name = format!("Emergencial {}", existing_players_count + i + 1);

        let slot_idx = (existing_players_count + i) % formation.slots().len();
        let slot = &formation.slots()[slot_idx];
        let target_pos = if slot.defensive_position() == Position::Goalguard
            || slot.position() == Position::Goalguard
            || slot.offensive_position() == Position::Goalguard
        {
            Position::Goalguard
        } else {
            slot.position()
        };

        let pos_str = format!("{:?}", target_pos);

        if !player_col_names.is_empty() {
            let mut cols = Vec::new();
            let mut placeholders = Vec::new();

            if player_col_names.contains("id") {
                cols.push("id");
                placeholders.push("?");
            }
            if player_col_names.contains("team_id") {
                cols.push("team_id");
                placeholders.push("?");
            }
            if player_col_names.contains("first_name") {
                cols.push("first_name");
                placeholders.push("?");
            }
            if player_col_names.contains("last_name") {
                cols.push("last_name");
                placeholders.push("?");
            }
            if player_col_names.contains("nickname") {
                cols.push("nickname");
                placeholders.push("?");
            }
            if player_col_names.contains("country_id") {
                cols.push("country_id");
                placeholders.push("?");
            }
            if player_col_names.contains("birthdate_unix_seconds") {
                cols.push("birthdate_unix_seconds");
                placeholders.push("?");
            }
            if player_col_names.contains("birth_date_unix_seconds") {
                cols.push("birth_date_unix_seconds");
                placeholders.push("?");
            }
            if player_col_names.contains("birth_year") {
                cols.push("birth_year");
                placeholders.push("?");
            }
            if player_col_names.contains("height_m") {
                cols.push("height_m");
                placeholders.push("?");
            }
            if player_col_names.contains("height_cm") {
                cols.push("height_cm");
                placeholders.push("?");
            }
            if player_col_names.contains("weight_kg") {
                cols.push("weight_kg");
                placeholders.push("?");
            }
            if player_col_names.contains("dominant_hand") {
                cols.push("dominant_hand");
                placeholders.push("?");
            }
            if player_col_names.contains("personality") {
                cols.push("personality");
                placeholders.push("?");
            }
            if player_col_names.contains("captaincy_role") {
                cols.push("captaincy_role");
                placeholders.push("?");
            }
            if player_col_names.contains("current_ability") {
                cols.push("current_ability");
                placeholders.push("?");
            }
            if player_col_names.contains("potential_ability") {
                cols.push("potential_ability");
                placeholders.push("?");
            }
            if player_col_names.contains("ca") {
                cols.push("ca");
                placeholders.push("?");
            }
            if player_col_names.contains("pa") {
                cols.push("pa");
                placeholders.push("?");
            }
            if player_col_names.contains("created_at_unix_seconds") {
                cols.push("created_at_unix_seconds");
                placeholders.push("?");
            }

            let sql = format!(
                "INSERT INTO players ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            );

            let mut query = sqlx::query(&sql);
            for &col in &cols {
                match col {
                    "id" => query = query.bind(player_id.to_string()),
                    "team_id" => query = query.bind(team_id.to_string()),
                    "first_name" => query = query.bind(first_name),
                    "last_name" => query = query.bind(&last_name),
                    "nickname" => query = query.bind(None::<String>),
                    "country_id" => query = query.bind(country_id.to_string()),
                    "birthdate_unix_seconds" | "birth_date_unix_seconds" => {
                        query = query.bind(now - (22 * 31557600));
                    }
                    "birth_year" => query = query.bind(2002i64),
                    "height_m" => query = query.bind(1.85f64),
                    "height_cm" => query = query.bind(185i32),
                    "weight_kg" => query = query.bind(80.0f64),
                    "dominant_hand" => query = query.bind("Right"),
                    "personality" => query = query.bind("Balanced"),
                    "captaincy_role" => query = query.bind(None::<String>),
                    "current_ability" | "ca" => query = query.bind(1i32),
                    "potential_ability" | "pa" => query = query.bind(1i32),
                    "created_at_unix_seconds" => query = query.bind(now),
                    _ => {}
                }
            }
            query.execute(&mut *tx).await?;
        }

        if !pos_col_names.is_empty() {
            let mut cols = Vec::new();
            let mut placeholders = Vec::new();

            if pos_col_names.contains("id") {
                cols.push("id");
                placeholders.push("?");
            }
            if pos_col_names.contains("player_id") {
                cols.push("player_id");
                placeholders.push("?");
            }
            if pos_col_names.contains("position") {
                cols.push("position");
                placeholders.push("?");
            }
            if pos_col_names.contains("proficiency") {
                cols.push("proficiency");
                placeholders.push("?");
            }

            let sql = format!(
                "INSERT OR IGNORE INTO player_positions ({}) VALUES ({})",
                cols.join(", "),
                placeholders.join(", ")
            );

            let mut query = sqlx::query(&sql);
            for &col in &cols {
                match col {
                    "id" => query = query.bind(Uuid::new_v4().to_string()),
                    "player_id" => query = query.bind(player_id.to_string()),
                    "position" => query = query.bind(&pos_str),
                    "proficiency" => query = query.bind(20i32),
                    _ => {}
                }
            }
            let _ = query.execute(&mut *tx).await;
        }

        if !attr_col_names.is_empty() && !attr_defs.is_empty() {
            for def in &attr_defs {
                if attr_col_names.contains("id") {
                    let sql = "INSERT OR IGNORE INTO player_attributes (id, player_id, attribute_definition_id, value) VALUES (?, ?, ?, ?)";
                    let _ = sqlx::query(sql)
                        .bind(Uuid::new_v4().to_string())
                        .bind(player_id.to_string())
                        .bind(def.id().to_string())
                        .bind(1i32)
                        .execute(&mut *tx)
                        .await;
                } else {
                    let sql = "INSERT OR IGNORE INTO player_attributes (player_id, attribute_definition_id, value) VALUES (?, ?, ?)";
                    let _ = sqlx::query(sql)
                        .bind(player_id.to_string())
                        .bind(def.id().to_string())
                        .bind(1i32)
                        .execute(&mut *tx)
                        .await;
                }
            }
        }
    }

    tx.commit().await?;

    Ok(())
}
