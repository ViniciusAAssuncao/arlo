use super::AuditResult;
use arlo_controller::services::season::matchday::{build_matchday_setup, MatchdayCatalogs};
use arlo_persistence::models::season::FixtureRow;
use sqlx::SqlitePool;

pub async fn audit(
    pool: &SqlitePool,
    fixture: &FixtureRow,
    catalogs: &std::sync::Arc<MatchdayCatalogs>,
) -> AuditResult<()> {
    let team = &fixture.home_team_id;
    let positions: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT pp.player_id, pp.position, pp.proficiency FROM player_positions pp JOIN players p ON p.id = pp.player_id WHERE p.team_id = ?")
        .bind(team).fetch_all(pool).await?;
    let attributes: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT a.manager_id, a.attribute_definition_id, a.value FROM manager_attributes a JOIN managers m ON m.id = a.manager_id WHERE m.team_id = ?")
        .bind(team).fetch_all(pool).await?;
    let opposition: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT a.player_id, a.attribute_definition_id, a.value FROM player_attributes a JOIN players p ON p.id = a.player_id WHERE p.team_id = ?")
        .bind(&fixture.away_team_id).fetch_all(pool).await?;
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT OR REPLACE INTO player_positions (player_id, position, proficiency) SELECT p.id, fs.position, 10 FROM players p CROSS JOIN (SELECT DISTINCT position FROM formation_slots) fs WHERE p.team_id = ?")
        .bind(team).execute(&mut *tx).await?;
    sqlx::query("UPDATE manager_attributes SET value = 20 WHERE manager_id IN (SELECT id FROM managers WHERE team_id = ?)")
        .bind(team).execute(&mut *tx).await?;
    sqlx::query("UPDATE player_attributes SET value = 20 WHERE player_id IN (SELECT id FROM players WHERE team_id = ?)")
        .bind(&fixture.away_team_id).execute(&mut *tx).await?;
    tx.commit().await?;
    let prepared = build_matchday_setup(pool, fixture, catalogs).await;
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM player_positions WHERE player_id IN (SELECT id FROM players WHERE team_id = ?)")
        .bind(team).execute(&mut *tx).await?;
    for (player, position, value) in positions {
        sqlx::query(
            "INSERT INTO player_positions (player_id, position, proficiency) VALUES (?, ?, ?)",
        )
        .bind(player)
        .bind(position)
        .bind(value)
        .execute(&mut *tx)
        .await?;
    }
    for (manager, definition, value) in attributes {
        sqlx::query("UPDATE manager_attributes SET value = ? WHERE manager_id = ? AND attribute_definition_id = ?")
            .bind(value).bind(manager).bind(definition).execute(&mut *tx).await?;
    }
    for (player, definition, value) in opposition {
        sqlx::query("UPDATE player_attributes SET value = ? WHERE player_id = ? AND attribute_definition_id = ?")
            .bind(value).bind(player).bind(definition).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    let prepared = prepared?;
    let alternatives = prepared
        .input
        .home()
        .prepared_plans()
        .iter()
        .filter(|plan| plan.layout.formation.id() != prepared.input.home().formation().id())
        .count();
    if alternatives == 0 {
        return Err("Versatile squad prepared no alternative formation".into());
    }
    super::commanded_plan::audit(pool, &prepared.input, &prepared.play_calls).await?;
    let result = super::simulation::audit_match(&prepared.input, &prepared.play_calls)?;
    if !result.reproducible || result.formation_changes == 0 {
        return Err(
            "Versatile formation scenario lacks a reproducible automatic formation change".into(),
        );
    }
    println!("Versatile squad: {alternatives} alternative formations, {} spontaneous formation changes; {result:?}", result.formation_changes);
    super::stress::audit(&prepared.input, &prepared.play_calls)?;
    super::attributes::audit(&prepared.input, &prepared.play_calls)?;
    Ok(())
}
