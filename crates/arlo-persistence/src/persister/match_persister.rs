use crate::error::PersistenceResult;
use crate::persister::core_persister::persist_match_core;
use crate::persister::incident_events_persister::persist_incident_events;
use crate::persister::manager_and_referee_persister::{
    persist_manager_stats, persist_referee_stats,
};
use crate::persister::match_persistence_context::MatchPersistenceContext;
use crate::persister::player_action_stats_persister::persist_player_action_stats;
use crate::persister::player_condition_stats_persister::persist_player_condition_stats;
use crate::persister::squad_selection_persister::persist_squad_selections;
use crate::persister::team_stats_persister::persist_team_stats;
use arlo_engine::{MatchInput, MatchState};
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub struct MatchPersister;

impl MatchPersister {
    pub async fn persist_completed_match(
        tx: &mut Transaction<'_, Sqlite>,
        input: &MatchInput,
        state: &MatchState,
        run_result: &MatchRunResult,
        context: &MatchPersistenceContext,
    ) -> PersistenceResult<Uuid> {
        if input.match_id() != state.match_id()
            || input.home().team_id() != state.home().team_id()
            || input.away().team_id() != state.away().team_id()
        {
            return Err(crate::error::PersistenceError::InvalidData(
                "match input and state differ".into(),
            ));
        }
        crate::persister::assigned_referees::assigned_referees(input)?;
        let match_id = state.match_id();

        if let Some(fixture_row) = &context.completed_fixture {
            crate::repositories::fixtures::update(tx, fixture_row).await?;
        }

        persist_match_core(tx, match_id, input, state, context).await?;
        persist_squad_selections(tx, match_id, input, run_result).await?;
        persist_player_action_stats(tx, match_id, &run_result.aggregators).await?;
        persist_player_condition_stats(tx, match_id, &run_result.aggregators).await?;
        persist_team_stats(tx, match_id, &run_result.aggregators).await?;
        persist_manager_stats(tx, match_id, &run_result.aggregators).await?;
        persist_referee_stats(tx, match_id, input, &run_result.aggregators).await?;
        persist_incident_events(tx, match_id, run_result).await?;

        Ok(match_id)
    }
}
