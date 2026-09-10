use crate::lineup_runtime::lineup::Lineup;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub const TOTAL_MATCH_SLOTS: usize = 28;
pub const SLOTS_PER_TEAM: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerSlot(u8);

impl PlayerSlot {
    #[inline]
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn is_home(self) -> bool {
        (self.0 as usize) < SLOTS_PER_TEAM
    }

    #[inline]
    pub const fn is_away(self) -> bool {
        let idx = self.0 as usize;
        idx >= SLOTS_PER_TEAM && idx < TOTAL_MATCH_SLOTS
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSlotRegistry {
    slots: [Uuid; TOTAL_MATCH_SLOTS],
    lookup: HashMap<Uuid, u8>,
}

impl PlayerSlotRegistry {
    pub fn new(slots: [Uuid; TOTAL_MATCH_SLOTS], lookup: HashMap<Uuid, u8>) -> Self {
        Self { slots, lookup }
    }

    pub fn from_lineups(home_lineup: &Lineup, away_lineup: &Lineup) -> Self {
        let mut slots = [Uuid::nil(); TOTAL_MATCH_SLOTS];
        let mut lookup = HashMap::with_capacity(TOTAL_MATCH_SLOTS);

        for (i, assignment) in home_lineup.assignments().iter().enumerate().take(SLOTS_PER_TEAM) {
            let pid = assignment.player().id();
            slots[i] = pid;
            lookup.insert(pid, i as u8);
        }

        for (i, assignment) in away_lineup.assignments().iter().enumerate().take(SLOTS_PER_TEAM) {
            let slot_idx = i + SLOTS_PER_TEAM;
            let pid = assignment.player().id();
            slots[slot_idx] = pid;
            lookup.insert(pid, slot_idx as u8);
        }

        Self { slots, lookup }
    }

    pub fn from_slices(home_players: &[Uuid], away_players: &[Uuid]) -> Self {
        let mut slots = [Uuid::nil(); TOTAL_MATCH_SLOTS];
        let mut lookup = HashMap::with_capacity(TOTAL_MATCH_SLOTS);

        for (i, &pid) in home_players.iter().enumerate().take(SLOTS_PER_TEAM) {
            slots[i] = pid;
            lookup.insert(pid, i as u8);
        }

        for (i, &pid) in away_players.iter().enumerate().take(SLOTS_PER_TEAM) {
            let slot_idx = i + SLOTS_PER_TEAM;
            slots[slot_idx] = pid;
            lookup.insert(pid, slot_idx as u8);
        }

        Self { slots, lookup }
    }

    #[inline]
    pub fn slot_for(&self, player_id: &Uuid) -> Option<PlayerSlot> {
        self.lookup.get(player_id).copied().map(PlayerSlot::new)
    }

    #[inline]
    pub fn player_at(&self, slot: PlayerSlot) -> Uuid {
        self.slots[slot.index()]
    }

    #[inline]
    pub fn slots(&self) -> &[Uuid; TOTAL_MATCH_SLOTS] {
        &self.slots
    }

    pub fn substitute(&mut self, outgoing: Uuid, incoming: Uuid) -> Option<PlayerSlot> {
        if let Some(&slot_idx) = self.lookup.get(&outgoing) {
            self.lookup.remove(&outgoing);
            self.lookup.insert(incoming, slot_idx);
            self.slots[slot_idx as usize] = incoming;
            Some(PlayerSlot::new(slot_idx))
        } else {
            None
        }
    }

    #[inline]
    pub fn is_home_slot(&self, slot: PlayerSlot) -> bool {
        slot.is_home()
    }

    #[inline]
    pub fn is_away_slot(&self, slot: PlayerSlot) -> bool {
        slot.is_away()
    }

    #[inline]
    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.slot_for(player_id).map(|s| s.is_home()).unwrap_or(false)
    }

    #[inline]
    pub fn is_away_player(&self, player_id: &Uuid) -> bool {
        self.slot_for(player_id).map(|s| s.is_away()).unwrap_or(false)
    }

    #[inline]
    pub fn len(&self) -> usize {
        TOTAL_MATCH_SLOTS
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl Default for PlayerSlotRegistry {
    fn default() -> Self {
        Self {
            slots: [Uuid::nil(); TOTAL_MATCH_SLOTS],
            lookup: HashMap::new(),
        }
    }
}