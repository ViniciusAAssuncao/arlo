use crate::officiating::punishment::reversal::{apply_play_reversal, PlayReversalSnapshot};
use crate::world_state::match_state::availability::AvailabilityState;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::PunishmentKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PunishmentLedgerEntry {
    pub kind: PunishmentKind,
    pub magnitude: Option<i32>,
    pub offending_player_id: Uuid,
    pub offending_team_id: Uuid,
    pub undo_snapshot: Option<PlayReversalSnapshot>,
    pub availability_before: Option<AvailabilityState>,
}

impl PunishmentLedgerEntry {
    pub fn new(
        kind: PunishmentKind,
        magnitude: Option<i32>,
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        undo_snapshot: Option<PlayReversalSnapshot>,
        availability_before: Option<AvailabilityState>,
    ) -> Self {
        Self {
            kind,
            magnitude,
            offending_player_id,
            offending_team_id,
            undo_snapshot,
            availability_before,
        }
    }
}

pub fn reverse_punishment(state: &mut MatchState, entry: &PunishmentLedgerEntry) {
    if let Some(snapshot) = &entry.undo_snapshot {
        apply_play_reversal(state, snapshot);
    }
    if let Some(avail) = entry.availability_before {
        state.restore_player_availability(entry.offending_player_id, avail);
    }
}