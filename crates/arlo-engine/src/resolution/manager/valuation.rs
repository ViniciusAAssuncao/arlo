use crate::input::{MatchInput, TeamInput};
use crate::state::{MatchState, TeamState};
use arlo_domain::{AttributeKey, Manager, Player, Position, PositionLine};
use uuid::Uuid;

pub(super) fn team_pair<'a>(input: &'a MatchInput, state: &'a MatchState, team_id: Uuid)
    -> Option<(&'a TeamInput, &'a TeamState)> {
    if team_id == input.home().team_id() { Some((input.home(), state.home())) }
    else if team_id == input.away().team_id() { Some((input.away(), state.away())) }
    else { None }
}

pub(super) fn assigned_position(team: &TeamInput, state: &TeamState, player_id: Uuid) -> Option<Position> {
    team.lineup().assignments().iter()
        .find(|assignment| state.slot_player_id(assignment.player_id()) == player_id)
        .map(|assignment| assignment.position())
}

pub(super) fn manager_value(input: &MatchInput, manager: &Manager, key: AttributeKey) -> f64 {
    manager.attributes().iter()
        .find(|entry| input.manager_attribute_keys().get(&entry.attribute_definition_id()) == Some(&key))
        .map_or(10.0, |entry| f64::from(entry.value()))
}

pub(super) fn player_value(input: &MatchInput, player: &Player, key: AttributeKey) -> f64 {
    input.player_attribute_definitions().iter().find(|definition| definition.key() == key)
        .and_then(|definition| player.attributes().iter()
            .find(|entry| entry.attribute_definition_id() == definition.id()))
        .map_or(10.0, |entry| f64::from(entry.value()))
}

pub(super) fn tactical_fit(input: &MatchInput, team: &TeamInput, player_id: Uuid, position: Position, trailing: bool) -> f64 {
    let Some(player) = team.roster().iter().find(|player| player.id() == player_id) else { return 0.0 };
    let keys: &[AttributeKey] = if trailing && position.line() == PositionLine::OffensiveLine {
        &[AttributeKey::Finishing, AttributeKey::Pace, AttributeKey::Anticipation]
    } else if !trailing && position.line() == PositionLine::DefenseLine {
        &[AttributeKey::DefensiveContainment, AttributeKey::Positioning, AttributeKey::Concentration]
    } else { return 0.0 };
    keys.iter().map(|key| player_value(input, player, *key)).sum::<f64>() / keys.len() as f64 / 20.0
}

pub(super) fn player_quality(
    input: &MatchInput, state: &MatchState, team: &TeamInput,
    player_id: Uuid, position: Position, newcomer: bool,
) -> f64 {
    let Some(player) = team.roster().iter().find(|player| player.id() == player_id) else { return 0.0 };
    let proficiency = player.positions().iter().filter(|entry| entry.position() == position)
        .map(|entry| entry.proficiency()).max().unwrap_or(0) as f64 / 10.0;
    let keys: &[AttributeKey] = match position {
        Position::Artrine => &[AttributeKey::ArloControl, AttributeKey::Passing, AttributeKey::Decisions, AttributeKey::DriveTechnique],
        Position::Passer => &[AttributeKey::Passing, AttributeKey::Vision, AttributeKey::Decisions, AttributeKey::HandsReception],
        Position::Goalguard => &[AttributeKey::Reflexes, AttributeKey::Handling, AttributeKey::AreaCommand, AttributeKey::OneOnOne],
        Position::CenterOffense => &[AttributeKey::Finishing, AttributeKey::Positioning, AttributeKey::Anticipation, AttributeKey::Strength],
        _ => match position.line() {
            PositionLine::OffensiveLine => &[AttributeKey::Finishing, AttributeKey::Positioning, AttributeKey::HandsReception],
            PositionLine::BackLine => &[AttributeKey::Passing, AttributeKey::ArloControl, AttributeKey::WorkRate],
            PositionLine::DefenseLine => &[AttributeKey::DefensiveContainment, AttributeKey::Anticipation, AttributeKey::Strength],
            PositionLine::Goalguard => &[AttributeKey::Reflexes, AttributeKey::Handling],
        },
    };
    let skill = keys.iter().map(|key| player_value(input, player, *key)).sum::<f64>() / keys.len() as f64 / 20.0;
    let energy = state.player_energy(player_id);
    let morale = state.player_morale(player_id);
    let condition = (0.58 + 0.42 * energy) * (0.72 + morale.clamp(0.0, 100.0) * 0.0028);
    let injury = if state.home().injured_player_ids().contains(&player_id)
        || state.away().injured_player_ids().contains(&player_id) { 0.72 } else { 1.0 };
    (0.5 * proficiency + 0.5 * skill) * condition * injury
        * if newcomer { 0.92 } else { state.player_settling_factor(player_id) }
}
