use super::TeamState;
use uuid::Uuid;

impl TeamState {
    pub fn last_tactical_realignment_at(&self) -> Option<f64> {
        self.last_tactical_realignment_at
    }

    pub(crate) fn realign_slots(&mut self, first_slot: Uuid, second_slot: Uuid, elapsed: f64) {
        let first_player = self.slot_player_id(first_slot);
        let second_player = self.slot_player_id(second_slot);
        self.slot_replacements.insert(first_slot, second_player);
        self.slot_replacements.insert(second_slot, first_player);
        self.last_tactical_realignment_at = Some(elapsed);
    }
}
