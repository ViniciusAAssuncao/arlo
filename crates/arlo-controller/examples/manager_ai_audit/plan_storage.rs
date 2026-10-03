use super::AuditResult;
use arlo_engine::MatchInput;
use arlo_events::{MatchEvent, MatchEventEnvelope};
use arlo_persistence::models::{MatchPreparedPlanRow, MatchTacticalPlanActivationRow};
use arlo_persistence::repositories::match_prepared_plans;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn audit(
    pool: &SqlitePool,
    input: &MatchInput,
    events: &[MatchEventEnvelope],
) -> AuditResult<()> {
    let id: Option<String> = sqlx::query_scalar("SELECT id FROM matches ORDER BY id LIMIT 1")
        .fetch_optional(pool)
        .await?;
    let Some(id) = id else { return Ok(()) };
    let id = Uuid::parse_str(&id)?;
    let sequence: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sequence_number), 0) + 1 FROM match_tactical_plan_activations WHERE match_id = ?")
        .bind(id.to_string()).fetch_one(pool).await?;
    let mut tx = pool.begin().await?;
    for team in [input.home(), input.away()] {
        for plan in team.prepared_plans() {
            let profile = team
                .tactical_profiles()
                .find(|profile| profile.id() == plan.profile_id)
                .ok_or("Missing profile")?;
            match_prepared_plans::insert_plan(
                &mut tx,
                &MatchPreparedPlanRow {
                    match_id: id.to_string(),
                    team_id: team.team_id().to_string(),
                    plan_id: plan.id.to_string(),
                    plan: plan.clone(),
                    profile: profile.clone(),
                },
            )
            .await?;
        }
    }
    for (index, event) in events.iter().enumerate() {
        let MatchEvent::TacticalPlanActivated(plan) = event.event() else {
            return Err("Unexpected stored event".into());
        };
        let clock = event.clock();
        match_prepared_plans::insert_activation(
            &mut tx,
            &MatchTacticalPlanActivationRow {
                match_id: id.to_string(),
                sequence_number: sequence + index as i64,
                team_id: plan.team_id.to_string(),
                plan_id: plan.plan_id.to_string(),
                plan_name: plan.plan_name.clone(),
                formation_id: plan.formation_id.to_string(),
                profile_id: plan.profile_id.to_string(),
                period: clock.period() as i32,
                seconds_in_period: clock.seconds_in_period(),
                total_elapsed_seconds: clock.total_elapsed_seconds(),
                assignments: plan.assignments.clone(),
            },
        )
        .await?;
    }
    tx.commit().await?;
    let plans = match_prepared_plans::list_plans_by_match_id(pool, id).await?;
    for team in [input.home(), input.away()] {
        for plan in team.prepared_plans() {
            let saved = plans
                .iter()
                .find(|row| row.plan_id == plan.id.to_string())
                .ok_or("Missing prepared snapshot")?;
            let profile = team
                .tactical_profiles()
                .find(|profile| profile.id() == plan.profile_id)
                .ok_or("Missing profile")?;
            if saved.plan != *plan || saved.profile != *profile {
                return Err(format!("Prepared snapshot differs: layout={}, profile={}; expected instructions {:?}, recovered {:?}",
                    saved.plan != *plan, saved.profile != *profile, profile.instructions(), saved.profile.instructions()).into());
            }
        }
    }
    let rows = match_prepared_plans::list_activations_by_match_id(pool, id).await?;
    let rows: Vec<_> = rows
        .iter()
        .filter(|row| row.sequence_number >= sequence)
        .collect();
    if rows.len() != events.len() {
        return Err("Unexpected stored activation count".into());
    }
    for (row, event) in rows.iter().zip(events) {
        let MatchEvent::TacticalPlanActivated(plan) = event.event() else {
            unreachable!()
        };
        if row.assignments != plan.assignments
            || row.plan_id != plan.plan_id.to_string()
            || row.total_elapsed_seconds != event.clock().total_elapsed_seconds()
        {
            return Err("Stored activation differs".into());
        }
    }
    let timeline =
        arlo_controller::controllers::r#match::match_timeline_controller::get_match_timeline(
            pool, id,
        )
        .await?;
    let count = timeline.iter().filter(|event| matches!(event,
        arlo_controller::dto::r#match::MatchTimelineEventDto::TacticalPlan(row) if row.sequence_number >= sequence as u64)).count();
    if count != events.len() {
        return Err("Prepared activation absent from timeline".into());
    }
    sqlx::query(
        "DELETE FROM match_tactical_plan_activations WHERE match_id = ? AND sequence_number >= ?",
    )
    .bind(id.to_string())
    .bind(sequence)
    .execute(pool)
    .await?;
    for plan in plans {
        if input
            .home()
            .prepared_plans()
            .iter()
            .chain(input.away().prepared_plans())
            .any(|original| original.id.to_string() == plan.plan_id)
        {
            sqlx::query("DELETE FROM match_prepared_plans WHERE match_id = ? AND team_id = ? AND plan_id = ?")
                .bind(id.to_string()).bind(plan.team_id).bind(plan.plan_id).execute(pool).await?;
        }
    }
    println!("Prepared plan snapshots, activation persistence and controller timeline verified");
    Ok(())
}
