use crate::error::PersistenceResult;
use crate::models::MatchTacticalRealignmentRow;
use crate::repositories::match_tactical_realignments;
use arlo_events::MatchEvent;
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_tactical_realignments(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    result: &MatchRunResult,
) -> PersistenceResult<()> {
    for envelope in result.raw_sink.events() {
        let MatchEvent::TacticalRealignmentMade(event) = envelope.event() else {
            continue;
        };
        let clock = envelope.clock();
        let row = MatchTacticalRealignmentRow {
            match_id: match_id.to_string(),
            sequence_number: envelope.sequence_number() as i64,
            team_id: event.team_id().to_string(),
            period: clock.period() as i32,
            seconds_in_period: clock.seconds_in_period(),
            total_elapsed_seconds: clock.total_elapsed_seconds(),
            assignments: event.assignments().clone(),
        };
        match_tactical_realignments::insert(tx, &row).await?;
    }
    Ok(())
}
