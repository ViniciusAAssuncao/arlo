use crate::snapshot::player_match_snapshot::PlayerMatchSnapshot;
use std::collections::HashMap;
use uuid::Uuid;

pub trait IntoSnapshot {
    type Snapshot;
    fn into_snapshot(&self) -> Self::Snapshot;
}

pub trait IntoPlayerMatchSnapshot {
    fn into_player_match_snapshot(&self, player_id: &Uuid) -> PlayerMatchSnapshot;
}

pub trait IntoPlayerSnapshots {
    fn player_snapshots(&self) -> HashMap<Uuid, PlayerMatchSnapshot>;
}