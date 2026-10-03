use crate::error::{PersistenceError, PersistenceResult};
use arlo_engine::MatchInput;
use arlo_events::MatchEvent;
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use std::collections::HashSet;

pub async fn persist_activated_tactical_profiles(
    tx: &mut Transaction<'_, Sqlite>,
    input: &MatchInput,
    run_result: &MatchRunResult,
) -> PersistenceResult<()> {
    let mut inserted = HashSet::new();
    for envelope in run_result.raw_sink.events() {
        let (team_id, profile_id) = match envelope.event() {
            MatchEvent::TacticalProfileActivated(event) => (event.team_id(), event.profile_id()),
            MatchEvent::TacticalPlanActivated(event) => (event.team_id, event.profile_id),
            _ => continue,
        };
        let team = if team_id == input.home().team_id() {
            input.home()
        } else if team_id == input.away().team_id() {
            input.away()
        } else {
            return Err(PersistenceError::InvalidData(
                "unknown tactical team".into(),
            ));
        };
        if profile_id == team.tactics().id() || !inserted.insert(profile_id) {
            continue;
        }
        let profile = team
            .tactical_profiles()
            .find(|profile| profile.id() == profile_id)
            .ok_or_else(|| PersistenceError::InvalidData("unknown tactical profile".into()))?;
        arlo_tactics::team_instructions::insert_profile_in_transaction(tx, profile)
            .await
            .map_err(|error| PersistenceError::InvalidData(error.to_string()))?;
    }
    Ok(())
}
