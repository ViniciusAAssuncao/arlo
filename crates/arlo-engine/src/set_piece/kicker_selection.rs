use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use arlo_domain::{AttributeKey, Player, SlotRole};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn select_kicker<'a, R: Rng + ?Sized>(
    players: &[&'a Player],
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    rng: &mut R,
) -> Option<&'a Player> {
    if players.is_empty() {
        return None;
    }
    if players.len() == 1 {
        return Some(players[0]);
    }

    let mut weights: SmallVec<[f64; 16]> = SmallVec::with_capacity(players.len());
    for p in players {
        let table = tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let goal_kicking = table.get(AttributeKey::GoalKicking);
        let finishing = table.get(AttributeKey::Finishing);
        let composure = table.get(AttributeKey::Composure);
        let technique = table.get(AttributeKey::Technique);

        let base_score = 0.5 + goal_kicking * 0.40 + finishing * 0.35 + composure * 0.15 + technique * 0.10;
        let role_bonus = if role_index.and_then(|r| r.get(&p.id())) == Some(&SlotRole::Kicker) {
            15.0
        } else {
            0.0
        };
        weights.push((base_score + role_bonus).max(0.1));
    }

    let chosen_idx = sample_categorical(&weights, rng).unwrap_or(0);
    players.get(chosen_idx).copied()
}