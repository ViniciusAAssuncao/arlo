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
use arlo_engine::MatchState;
use arlo_match_runner::MatchRunResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct MatchPersister;

impl MatchPersister {
    pub async fn persist_completed_match(
        pool: &SqlitePool,
        state: &MatchState,
        run_result: &MatchRunResult,
        context: &MatchPersistenceContext,
    ) -> PersistenceResult<Uuid> {
        let match_id = Uuid::new_v4();
        let mut tx = pool.begin().await?;

        persist_match_core(&mut tx, match_id, state, context).await?;
        persist_squad_selections(&mut tx, match_id, state, run_result).await?;
        persist_player_action_stats(&mut tx, match_id, &run_result.aggregators).await?;
        persist_player_condition_stats(&mut tx, match_id, &run_result.aggregators).await?;
        persist_team_stats(&mut tx, match_id, &run_result.aggregators).await?;
        persist_manager_stats(&mut tx, match_id, &run_result.aggregators).await?;
        persist_referee_stats(&mut tx, match_id, state, &run_result.aggregators).await?;
        persist_incident_events(&mut tx, match_id, run_result).await?;

        tx.commit().await?;

        Ok(match_id)
    }
}
