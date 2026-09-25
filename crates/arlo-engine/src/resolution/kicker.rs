use super::ratings::RatingIndex;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{AttributeKey, SlotRole};
use arlo_tactics::PlayCall;
use uuid::Uuid;

pub(super) fn select_kicker(
    ratings: &RatingIndex,
    team: &TeamInput,
    play_call: Option<&PlayCall>,
) -> EngineResult<Uuid> {
    if let Some(assignment) = team.lineup().assignments().iter().find(|assignment| {
        if !ratings.is_active_slot(team, assignment.player_id()) { return false; }
        let role = play_call
            .and_then(|call| {
                call.role_overrides()
                    .iter()
                    .rev()
                    .find(|(slot, _)| *slot == assignment.formation_slot_index())
                    .map(|(_, role)| *role)
            })
            .unwrap_or_else(|| assignment.slot_role());
        role == SlotRole::Kicker
    }) {
        return Ok(ratings.slot_player_id(team, assignment.player_id()));
    }
    let mut best = None;
    for assignment in team.lineup().assignments() {
        let player_id = ratings.slot_player_id(team, assignment.player_id());
        if !ratings.is_active(team, player_id) { continue; }
        let finishing = ratings.player_value(team, player_id, AttributeKey::Finishing)?;
        let technique = ratings.player_value(team, player_id, AttributeKey::Technique)?;
        let score = finishing + technique;
        if best.is_none_or(|(_, best_score)| score > best_score) {
            best = Some((player_id, score));
        }
    }
    best.map(|(player_id, _)| player_id)
        .ok_or_else(|| EngineError::InvalidInput("lineup has no available kick taker".into()))
}
