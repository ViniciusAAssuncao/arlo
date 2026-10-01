use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{Formation, Player, Position};
use arlo_tactics::{PlayerInstructions, SlotAssignment, TacticalLineup};
use std::collections::HashSet;
use uuid::Uuid;

pub fn select_lineup(
    team_id: Uuid,
    players: &[Player],
    formations: &[Formation],
) -> ControllerResult<(TacticalLineup, Formation)> {
    let formation = formations
        .first()
        .ok_or_else(|| ControllerError::NotFound("No formations available in database".into()))?;
    if players.len() < formation.slots().len() {
        return Err(ControllerError::InvalidData(format!(
            "Team {team_id} has too few available players for a lineup"
        )));
    }

    let mut slots: Vec<_> = (0..formation.slots().len()).collect();
    slots.sort_by_key(|&index| {
        let position = formation.slots()[index].position();
        players
            .iter()
            .filter(|player| {
                player
                    .positions()
                    .iter()
                    .any(|proficiency| proficiency.position() == position)
            })
            .count()
    });

    let mut assigned = HashSet::new();
    let mut assignments = Vec::with_capacity(slots.len());
    for index in slots {
        let slot = &formation.slots()[index];
        let player = players
            .iter()
            .filter(|player| !assigned.contains(&player.id()))
            .max_by_key(|player| position_score(player, slot.position()))
            .ok_or_else(|| ControllerError::InvalidData("Unable to fill lineup".into()))?;
        assigned.insert(player.id());
        assignments.push(SlotAssignment::new(
            index,
            slot.position(),
            player.id(),
            slot.role(),
            PlayerInstructions::default(),
        ));
    }
    assignments.sort_by_key(SlotAssignment::formation_slot_index);
    let lineup = TacticalLineup::new(
        Uuid::new_v4(),
        team_id,
        formation.id(),
        format!("Automatic {}", formation.name()),
        assignments,
    );
    Ok((lineup, formation.clone()))
}

fn position_score(player: &Player, position: Position) -> i32 {
    let proficiency = player
        .positions()
        .iter()
        .filter(|entry| entry.position() == position)
        .map(|entry| entry.proficiency())
        .max()
        .unwrap_or(0);
    let same_line = player
        .positions()
        .iter()
        .any(|entry| entry.position().line() == position.line());
    let attribute_total: i32 = player.attributes().iter().map(|entry| entry.value()).sum();
    let attribute_mean = attribute_total / player.attributes().len().max(1) as i32;
    proficiency * 100 + i32::from(same_line) * 10 + attribute_mean
}
