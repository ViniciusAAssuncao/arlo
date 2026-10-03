use crate::error::PersistenceResult;
use crate::models::{MatchPreparedPlanRow, MatchTacticalPlanActivationRow};
use crate::repositories::match_prepared_plans;
use arlo_engine::MatchInput;
use arlo_events::MatchEvent;
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_prepared_plans(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    input: &MatchInput,
    result: &MatchRunResult,
) -> PersistenceResult<()> {
    for team in [input.home(), input.away()] {
        for plan in team.prepared_plans() {
            let profile = team
                .tactical_profiles()
                .find(|profile| profile.id() == plan.profile_id)
                .expect("validated prepared profile");
            match_prepared_plans::insert_plan(
                tx,
                &MatchPreparedPlanRow {
                    match_id: match_id.to_string(),
                    team_id: team.team_id().to_string(),
                    plan_id: plan.id.to_string(),
                    plan: plan.clone(),
                    profile: profile.clone(),
                },
            )
            .await?;
        }
    }
    for envelope in result.raw_sink.events() {
        let MatchEvent::TacticalPlanActivated(event) = envelope.event() else {
            continue;
        };
        let clock = envelope.clock();
        match_prepared_plans::insert_activation(
            tx,
            &MatchTacticalPlanActivationRow {
                match_id: match_id.to_string(),
                sequence_number: envelope.sequence_number() as i64,
                team_id: event.team_id.to_string(),
                plan_id: event.plan_id.to_string(),
                plan_name: event.plan_name.clone(),
                formation_id: event.formation_id.to_string(),
                profile_id: event.profile_id.to_string(),
                period: clock.period() as i32,
                seconds_in_period: clock.seconds_in_period(),
                total_elapsed_seconds: clock.total_elapsed_seconds(),
                assignments: event.assignments.clone(),
            },
        )
        .await?;
    }
    Ok(())
}
