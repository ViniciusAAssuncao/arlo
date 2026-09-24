use crate::error::PersistenceResult;
use crate::models::MatchSquadSelectionRow;
use crate::repositories;
use arlo_engine::{MatchInput, TeamInput};
use arlo_events::{AvailabilityStatus, MatchEvent};
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

fn status_name(status: AvailabilityStatus) -> &'static str {
    match status {
        AvailabilityStatus::Active => "Active",
        AvailabilityStatus::Suspended => "Suspended",
        AvailabilityStatus::Expelled => "Expelled",
        AvailabilityStatus::Injured => "Injured",
    }
}

fn build_team_squad_selections(
    match_id: Uuid,
    team: &TeamInput,
    used_player_ids: &HashSet<Uuid>,
    final_statuses: &HashMap<Uuid, AvailabilityStatus>,
) -> Vec<MatchSquadSelectionRow> {
    let starters: HashSet<Uuid> = team
        .lineup()
        .assignments()
        .iter()
        .map(|assignment| assignment.player_id())
        .collect();
    let mut rows = Vec::with_capacity(team.roster().len());
    for assignment in team.lineup().assignments() {
        let player_id = assignment.player_id();
        let status = final_statuses
            .get(&player_id)
            .copied()
            .unwrap_or(AvailabilityStatus::Active);
        rows.push(MatchSquadSelectionRow::new(
            Uuid::new_v4(),
            match_id,
            team.team_id(),
            player_id,
            true,
            Some(assignment.formation_slot_index() as i32),
            Some(format!("{:?}", assignment.slot_role())),
            true,
            status_name(status),
            None,
        ));
    }
    for player in team.roster() {
        let player_id = player.id();
        if starters.contains(&player_id) {
            continue;
        }
        let status = final_statuses
            .get(&player_id)
            .copied()
            .unwrap_or(AvailabilityStatus::Active);
        rows.push(MatchSquadSelectionRow::new(
            Uuid::new_v4(),
            match_id,
            team.team_id(),
            player_id,
            false,
            None,
            None,
            used_player_ids.contains(&player_id),
            status_name(status),
            None,
        ));
    }
    rows
}

pub async fn persist_squad_selections(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    input: &MatchInput,
    run_result: &MatchRunResult,
) -> PersistenceResult<()> {
    let mut used_player_ids = HashSet::new();
    let mut final_statuses = HashMap::new();
    for envelope in run_result.raw_sink.events() {
        match envelope.event() {
            MatchEvent::SubstitutionMade(event) => {
                used_player_ids.insert(event.player_out());
                used_player_ids.insert(event.player_in());
            }
            MatchEvent::PlayerAvailabilityChanged(event) => {
                final_statuses.insert(event.player_id(), event.new_status());
            }
            _ => {}
        }
    }
    let mut rows =
        build_team_squad_selections(match_id, input.home(), &used_player_ids, &final_statuses);
    rows.extend(build_team_squad_selections(
        match_id,
        input.away(),
        &used_player_ids,
        &final_statuses,
    ));
    repositories::match_squad_selection::insert_batch(tx, &rows).await?;
    Ok(())
}
