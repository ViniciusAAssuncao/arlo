use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::find_player_by_position;
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PhaseParticipants<'a> {
    pub passer: &'a Player,
    pub artrine: &'a Player,
    pub pass_rusher: &'a Player,
    pub goalguard: &'a Player,
    pub pass_blockers: Vec<(&'a Player, DomainPosition)>,
    pub pass_rushers: Vec<(&'a Player, DomainPosition)>,
    pub attacker_ids: SmallVec<[Uuid; 4]>,
    pub defender_ids: SmallVec<[Uuid; 4]>,
}

pub fn extract_participants<'a>(
    offense_players: &[&'a Player],
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defense_players: &[&'a Player],
    defense_pos_index: &HashMap<Uuid, DomainPosition>,
) -> EngineResult<PhaseParticipants<'a>> {
    if offense_players.is_empty() {
        return Err(EngineError::MissingRequiredPosition(format!("{:?}", DomainPosition::Passer)));
    }
    if defense_players.is_empty() {
        return Err(EngineError::MissingRequiredPosition(format!("{:?}", DomainPosition::Goalguard)));
    }

    let passer = offense_players
        .iter()
        .copied()
        .find(|p| offense_pos_index.get(&p.id()) == Some(&DomainPosition::Passer))
        .or_else(|| {
            offense_players.iter().copied().find(|p| {
                p.positions()
                    .iter()
                    .any(|pos| pos.position() == DomainPosition::Passer && pos.proficiency() > 0)
            })
        })
        .or_else(|| find_player_by_position(offense_players, DomainPosition::Passer).ok())
        .unwrap_or(offense_players[0]);

    let artrine = offense_players
        .iter()
        .copied()
        .filter(|p| p.id() != passer.id())
        .find(|p| offense_pos_index.get(&p.id()) == Some(&DomainPosition::Artrine))
        .or_else(|| {
            offense_players
                .iter()
                .copied()
                .filter(|p| p.id() != passer.id())
                .find(|p| {
                    p.positions()
                        .iter()
                        .any(|pos| pos.position() == DomainPosition::Artrine && pos.proficiency() > 0)
                })
        })
        .or_else(|| {
            let candidates: Vec<&Player> = offense_players
                .iter()
                .copied()
                .filter(|p| p.id() != passer.id())
                .collect();
            find_player_by_position(&candidates, DomainPosition::Artrine).ok()
        })
        .or_else(|| {
            offense_players
                .iter()
                .copied()
                .find(|p| p.id() != passer.id())
        })
        .unwrap_or(passer);

    let goalguard = defense_players
        .iter()
        .copied()
        .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::Goalguard))
        .or_else(|| {
            defense_players.iter().copied().find(|p| {
                p.positions()
                    .iter()
                    .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
            })
        })
        .or_else(|| find_player_by_position(defense_players, DomainPosition::Goalguard).ok())
        .unwrap_or(defense_players[0]);

    let outfield_defenders: Vec<&Player> = defense_players
        .iter()
        .copied()
        .filter(|p| p.id() != goalguard.id())
        .collect();

    let pass_rusher = if !outfield_defenders.is_empty() {
        outfield_defenders
            .iter()
            .copied()
            .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::PassRusher))
            .or_else(|| {
                outfield_defenders.iter().copied().find(|p| {
                    p.positions()
                        .iter()
                        .any(|pos| pos.position() == DomainPosition::PassRusher && pos.proficiency() > 0)
                })
            })
            .or_else(|| find_player_by_position(&outfield_defenders, DomainPosition::PassRusher).ok())
            .unwrap_or(outfield_defenders[0])
    } else {
        goalguard
    };

    let mut pass_blockers = vec![
        (passer, DomainPosition::Passer),
        (artrine, DomainPosition::Artrine),
    ];
    let mut attacker_ids: SmallVec<[Uuid; 4]> = smallvec![passer.id(), artrine.id()];

    for &player in offense_players {
        if offense_role_index.get(&player.id()) == Some(&SlotRole::Safeguard)
            && !attacker_ids.contains(&player.id())
        {
            let pos = offense_pos_index
                .get(&player.id())
                .copied()
                .unwrap_or_else(|| {
                    player
                        .positions()
                        .first()
                        .map(|pp| pp.position())
                        .unwrap_or(DomainPosition::Fullback)
                });
            pass_blockers.push((player, pos));
            attacker_ids.push(player.id());
        }
    }

    let pass_rushers = vec![(pass_rusher, DomainPosition::PassRusher)];
    let defender_ids: SmallVec<[Uuid; 4]> = smallvec![pass_rusher.id()];

    Ok(PhaseParticipants {
        passer,
        artrine,
        pass_rusher,
        goalguard,
        pass_blockers,
        pass_rushers,
        attacker_ids,
        defender_ids,
    })
}
