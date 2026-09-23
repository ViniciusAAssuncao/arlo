use crate::error::PersistenceResult;
use crate::models::MatchSquadSelectionRow;
use crate::repositories;
use arlo_engine::{AvailabilityState, MatchState};
use arlo_events::MatchEvent;
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use std::collections::HashSet;
use uuid::Uuid;

fn map_availability_status(state: &MatchState, player_id: &Uuid) -> (&'static str, Option<f64>) {
    match state.availability_for(player_id) {
        AvailabilityState::Active => ("Active", None),
        AvailabilityState::Suspended { remaining_seconds } => {
            ("Suspended", Some(remaining_seconds))
        }
        AvailabilityState::Expelled => ("Expelled", None),
        AvailabilityState::Injured => ("Injured", None),
    }
}

fn build_team_squad_selections(
    match_id: Uuid,
    team_id: Uuid,
    is_home: bool,
    state: &MatchState,
    starters_from_subs: &HashSet<Uuid>,
    ever_subbed_in: &HashSet<Uuid>,
    ever_subbed_out: &HashSet<Uuid>,
) -> Vec<MatchSquadSelectionRow> {
    let mut rows = Vec::new();
    let role_index = state.role_index_for_team(team_id);
    let lineup = if is_home {
        state.home_lineup()
    } else {
        state.away_lineup()
    };
    let squad = if is_home {
        state.home_squad()
    } else {
        state.away_squad()
    };

    for (idx, assignment) in lineup.assignments().iter().enumerate() {
        let pid = assignment.player().id();
        let was_starter = !ever_subbed_in.contains(&pid) || starters_from_subs.contains(&pid);
        let slot_role = role_index.get(&pid).map(|r| format!("{:?}", r));
        let (status_str, rem_sec) = map_availability_status(state, &pid);
        rows.push(MatchSquadSelectionRow::new(
            Uuid::new_v4(),
            match_id,
            team_id,
            pid,
            was_starter,
            Some(idx as i32),
            slot_role,
            true,
            status_str,
            rem_sec,
        ));
    }

    for bench_p in squad.bench() {
        let pid = bench_p.id();
        let was_used = ever_subbed_in.contains(&pid) || ever_subbed_out.contains(&pid);
        let was_starter = starters_from_subs.contains(&pid);
        let (status_str, rem_sec) = map_availability_status(state, &pid);
        rows.push(MatchSquadSelectionRow::new(
            Uuid::new_v4(),
            match_id,
            team_id,
            pid,
            was_starter,
            None,
            None,
            was_used,
            status_str,
            rem_sec,
        ));
    }

    rows
}

pub async fn persist_squad_selections(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    state: &MatchState,
    run_result: &MatchRunResult,
) -> PersistenceResult<()> {
    let mut starters_from_subs = HashSet::new();
    let mut ever_subbed_in = HashSet::new();
    let mut ever_subbed_out = HashSet::new();

    for env in run_result.raw_sink.events() {
        if let MatchEvent::SubstitutionMade(e) = env.event() {
            if !ever_subbed_in.contains(&e.player_out()) {
                starters_from_subs.insert(e.player_out());
            }
            ever_subbed_out.insert(e.player_out());
            ever_subbed_in.insert(e.player_in());
        }
    }

    let mut rows = build_team_squad_selections(
        match_id,
        state.home_team_id(),
        true,
        state,
        &starters_from_subs,
        &ever_subbed_in,
        &ever_subbed_out,
    );

    let away_rows = build_team_squad_selections(
        match_id,
        state.away_team_id(),
        false,
        state,
        &starters_from_subs,
        &ever_subbed_in,
        &ever_subbed_out,
    );

    rows.extend(away_rows);
    repositories::match_squad_selection::insert_batch(tx, &rows).await?;

    Ok(())
}
