use crate::attributes::PlayerAttributeTable;
use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::find_goalguard;
use arlo_domain::{AttributeKey, Player, Position, SlotRole};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct KickFoulParticipants<'a> {
    pub kicker: &'a Player,
    pub protectors: Vec<&'a Player>,
    pub rushers: Vec<&'a Player>,
    pub goalguard: &'a Player,
    pub target_candidates: Vec<&'a Player>,
}

impl<'a> KickFoulParticipants<'a> {
    pub fn new(
        kicker: &'a Player,
        protectors: Vec<&'a Player>,
        rushers: Vec<&'a Player>,
        goalguard: &'a Player,
        target_candidates: Vec<&'a Player>,
    ) -> Self {
        Self {
            kicker,
            protectors,
            rushers,
            goalguard,
            target_candidates,
        }
    }
}

pub fn select_kicker<'a, R: Rng + ?Sized>(
    offense_players: &[&'a Player],
    offense_role_index: &HashMap<Uuid, SlotRole>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    rng: &mut R,
) -> Option<&'a Player> {
    if let Some(designated_id) =
        crate::set_piece::kicker_selection::select_kicker(offense_players, Some(offense_role_index))
    {
        return offense_players
            .iter()
            .copied()
            .find(|p| p.id() == designated_id);
    }

    if offense_players.is_empty() {
        return None;
    }

    let weights: Vec<f64> = offense_players
        .iter()
        .map(|p| {
            let table = tables.get(&p.id());
            table
                .map(|t| t.get(AttributeKey::GoalKicking) + t.get(AttributeKey::Finishing))
                .unwrap_or(0.0)
        })
        .collect();

    let idx = sample_categorical(&weights, rng).unwrap_or(0);
    offense_players.get(idx).copied()
}

pub fn select_kick_foul_participants<'a, R: Rng + ?Sized>(
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    offense_role_index: &HashMap<Uuid, SlotRole>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    rng: &mut R,
) -> EngineResult<KickFoulParticipants<'a>> {
    let kicker = select_kicker(offense_players, offense_role_index, tables, rng)
        .or_else(|| offense_players.first().copied())
        .ok_or_else(|| EngineError::MissingRequiredPosition(format!("{:?}", Position::CenterOffense)))?;

    let goalguard = find_goalguard(defense_players)?;

    let protectors: Vec<&Player> = offense_players
        .iter()
        .copied()
        .filter(|p| p.id() != kicker.id())
        .collect();

    let rushers: Vec<&Player> = defense_players
        .iter()
        .copied()
        .filter(|p| p.id() != goalguard.id())
        .collect();

    let target_candidates: Vec<&Player> = protectors.clone();

    Ok(KickFoulParticipants::new(
        kicker,
        protectors,
        rushers,
        goalguard,
        target_candidates,
    ))
}