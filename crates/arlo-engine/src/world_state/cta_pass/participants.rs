use crate::error::{EngineError, EngineResult};
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PhaseParticipants<'a> {
    pub passer: &'a Player,
    pub artrine: &'a Player,
    pub pass_rusher: &'a Player,
    pub goalguard: &'a Player,
    pub pass_blockers: Vec<(&'a Player, DomainPosition)>,
    pub pass_rushers: Vec<(&'a Player, DomainPosition)>,
    pub attacker_ids: Vec<Uuid>,
    pub defender_ids: Vec<Uuid>,
}

pub fn find_player_by_position<'a>(
    players: &[&'a Player],
    target: DomainPosition,
) -> EngineResult<&'a Player> {
    players
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == target && pos.proficiency() > 0)
        })
        .ok_or_else(|| EngineError::MissingRequiredPosition(format!("{target:?}")))
}

pub fn extract_participants<'a>(
    offense_players: &[&'a Player],
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defense_players: &[&'a Player],
) -> EngineResult<PhaseParticipants<'a>> {
    let passer = find_player_by_position(offense_players, DomainPosition::Passer)?;
    let artrine = find_player_by_position(offense_players, DomainPosition::Artrine)?;
    let pass_rusher = find_player_by_position(defense_players, DomainPosition::PassRusher)?;
    let goalguard = find_player_by_position(defense_players, DomainPosition::Goalguard)?;

    let mut pass_blockers = vec![
        (passer, DomainPosition::Passer),
        (artrine, DomainPosition::Artrine),
    ];
    let mut attacker_ids = vec![passer.id(), artrine.id()];

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
    let defender_ids = vec![pass_rusher.id()];

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
