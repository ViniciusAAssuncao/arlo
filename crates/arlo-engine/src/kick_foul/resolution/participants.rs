use crate::attributes::PlayerAttributeTable;
use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::find_goalguard;
use arlo_domain::{AttributeKey, Player, Position, SlotRole};
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

pub fn select_kicker<'a>(
    offense_players: &[&'a Player],
    offense_role_index: &HashMap<Uuid, SlotRole>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
) -> Option<&'a Player> {
    if let Some(&kicker) = offense_players
        .iter()
        .find(|p| offense_role_index.get(&p.id()) == Some(&SlotRole::Kicker))
    {
        return Some(kicker);
    }

    offense_players.iter().copied().max_by(|a, b| {
        let table_a = tables.get(&a.id());
        let table_b = tables.get(&b.id());
        let score_a = table_a
            .map(|t| t.get(AttributeKey::GoalKicking) + t.get(AttributeKey::Finishing))
            .unwrap_or(0.0);
        let score_b = table_b
            .map(|t| t.get(AttributeKey::GoalKicking) + t.get(AttributeKey::Finishing))
            .unwrap_or(0.0);
        score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
    })
}

pub fn select_kick_foul_participants<'a>(
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    offense_role_index: &HashMap<Uuid, SlotRole>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
) -> EngineResult<KickFoulParticipants<'a>> {
    let kicker = select_kicker(offense_players, offense_role_index, tables)
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