use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::manager_match_context::ManagerMatchContext;
use arlo_domain::{ArtrineDependency, AttributeKey, Formation, Manager, OffensiveApproach, Player, Position, PositionLine, RotationPolicy};
use arlo_recovery::PlayerCondition;
use arlo_tactics::{PlayerInstructions, SlotAssignment, TacticalLineup};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn choose_lineup(
    team_id: Uuid,
    players: &[Player],
    formations: &[Formation],
    manager: &Manager,
    attributes: &HashMap<Uuid, AttributeKey>,
    conditions: &HashMap<Uuid, PlayerCondition>,
    context: &ManagerMatchContext,
    previous_formation_id: Option<Uuid>,
) -> ControllerResult<(TacticalLineup, Formation)> {
    let mut best: Option<(f64, TacticalLineup, Formation)> = None;
    for formation in formations.iter().filter(|formation| formation.slots().len() == 14) {
        if players.len() < formation.slots().len() {
            continue;
        }
        let mut indices: Vec<_> = (0..formation.slots().len()).collect();
        indices.sort_by_key(|&index| players.iter().filter(|player| player.positions().iter()
            .any(|entry| entry.position() == formation.slots()[index].position())).count());
        let mut selected = HashSet::new();
        let mut assignments = Vec::with_capacity(14);
        let mut score = 0.0;
        for index in indices {
            let slot = &formation.slots()[index];
            let (player, value) = players.iter()
                .filter(|player| !selected.contains(&player.id()))
                .map(|player| (player, player_score(player, slot.position(), manager, attributes, conditions, context)))
                .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.id().cmp(&b.0.id())))
                .ok_or_else(|| ControllerError::InvalidData("Unable to fill manager lineup".into()))?;
            selected.insert(player.id());
            score += value;
            assignments.push(SlotAssignment::new(index, slot.position(), player.id(), slot.role(), PlayerInstructions::default()));
        }
        assignments.sort_by_key(SlotAssignment::formation_slot_index);
        if let Some(style) = manager.tactical_profile() {
            if let Some(rank) = style.preferred_formation_ids().iter().position(|id| *id == formation.id()) {
                score += 45.0 / (rank as f64 + 1.0);
            }
        }
        let attack_slots = formation.slots().iter()
            .filter(|slot| slot.position().line() == PositionLine::OffensiveLine).count() as f64;
        let defense_slots = formation.slots().iter()
            .filter(|slot| slot.position().line() == PositionLine::DefenseLine).count() as f64;
        let flexibility = manager.tactical_profile().map_or_else(
            || (manager_attribute(manager, AttributeKey::Adaptability, attributes) / 20.0)
                .clamp(0.1, 0.95),
            |style| style.flexibility_tendency());
        if previous_formation_id == Some(formation.id()) {
            score += (1.0 - flexibility) * 55.0;
        }
        let tactical_need = context.relative_strength * flexibility;
        score += (attack_slots - defense_slots) * tactical_need * 13.0;
        let variation = (formation.id().as_u128() ^ context.selection_seed).rotate_left(29);
        let judgement_skill = manager_attribute(manager, AttributeKey::JudgingAbility, attributes);
        let judgement = (variation as u64 as f64 / u64::MAX as f64 - 0.5)
            * (1.0 + flexibility) * (18.0 - judgement_skill * 0.5);
        score += judgement;
        let lineup = TacticalLineup::new(Uuid::new_v4(), team_id, formation.id(),
            format!("Manager {}", formation.name()), assignments);
        if best.as_ref().is_none_or(|(value, _, _)| score > *value) {
            best = Some((score, lineup, formation.clone()));
        }
    }
    best.map(|(_, lineup, formation)| (lineup, formation))
        .ok_or_else(|| ControllerError::InvalidData(format!("Team {team_id} has no valid formation and available roster")))
}

fn player_score(
    player: &Player,
    position: Position,
    manager: &Manager,
    attributes: &HashMap<Uuid, AttributeKey>,
    conditions: &HashMap<Uuid, PlayerCondition>,
    context: &ManagerMatchContext,
) -> f64 {
    let proficiency = player.positions().iter().filter(|entry| entry.position() == position)
        .map(|entry| entry.proficiency()).max().unwrap_or(0) as f64;
    let same_line = player.positions().iter().any(|entry| entry.position().line() == position.line());
    let keys: &[AttributeKey] = match position {
        Position::Artrine => &[AttributeKey::ArloControl, AttributeKey::Decisions, AttributeKey::Passing, AttributeKey::DriveTechnique],
        Position::Passer => &[AttributeKey::Passing, AttributeKey::Vision, AttributeKey::Decisions, AttributeKey::HandsReception],
        Position::Goalguard => &[AttributeKey::Reflexes, AttributeKey::Handling, AttributeKey::OneOnOne, AttributeKey::AreaCommand],
        Position::CenterOffense => &[AttributeKey::Finishing, AttributeKey::Positioning, AttributeKey::Anticipation, AttributeKey::Strength],
        _ => match position.line() {
            PositionLine::OffensiveLine => &[AttributeKey::Finishing, AttributeKey::HandsReception, AttributeKey::Pace, AttributeKey::Positioning],
            PositionLine::BackLine => &[AttributeKey::Passing, AttributeKey::ArloControl, AttributeKey::Teamwork, AttributeKey::WorkRate],
            PositionLine::DefenseLine => &[AttributeKey::DefensiveContainment, AttributeKey::PasserPressure, AttributeKey::Anticipation, AttributeKey::Strength],
            PositionLine::Goalguard => &[AttributeKey::Reflexes, AttributeKey::Handling],
        },
    };
    let skill = keys.iter().map(|key| attribute(player, *key, attributes)).sum::<f64>() / keys.len() as f64;
    let style_bonus = manager.tactical_profile().map_or(0.0, |style| match style.offensive_approach() {
        OffensiveApproach::Positional => 0.3 * attribute(player, AttributeKey::Teamwork, attributes)
            + 0.2 * attribute(player, AttributeKey::Vision, attributes),
        OffensiveApproach::Direct => 0.3 * attribute(player, AttributeKey::Pace, attributes)
            + 0.2 * attribute(player, AttributeKey::Strength, attributes),
        _ => 0.25 * attribute(player, AttributeKey::Decisions, attributes),
    });
    let artrine_fit = if position == Position::Artrine
        && manager.tactical_profile().is_some_and(|style| style.artrine_dependency() == ArtrineDependency::ArtrineCentric) {
        attribute(player, AttributeKey::DriveTechnique, attributes) * 0.55
            + attribute(player, AttributeKey::Leadership, attributes) * 0.25
    } else { 0.0 };
    let energy = conditions.get(&player.id()).map_or(1.0, |condition| condition.fatigue().energy());
    let morale = conditions.get(&player.id()).map_or(100.0, |condition| condition.morale().current());
    let rotation = manager.tactical_profile().map_or_else(
        || (manager_attribute(manager, AttributeKey::LoadManagement, attributes) / 20.0)
            .clamp(0.2, 0.9),
        |style| match style.rotation_policy() {
        RotationPolicy::StrictCore => 0.25,
        RotationPolicy::Situational => 0.5,
        RotationPolicy::HighRotation => 0.85,
    });
    let readiness = (1.0 - rotation * (1.0 - energy) * 0.55) * (0.9 + morale.clamp(0.0, 120.0) * 0.001);
    let rotation_seed = (player.id().as_u128() ^ context.selection_seed).rotate_left(13);
    let rotation_bias = (rotation_seed as u64 as f64 / u64::MAX as f64 - 0.5)
        * context.rotation_opportunity * rotation * 24.0;
    let recent_load = f64::from(*context.recent_starts.get(&player.id()).unwrap_or(&0));
    let rest_priority = recent_load * context.rotation_opportunity * rotation * 6.0;
    (proficiency * 5.0 + if same_line { 8.0 } else { 0.0 } + skill * 1.8 + style_bonus + artrine_fit)
        * readiness + rotation_bias - rest_priority
}

fn attribute(player: &Player, key: AttributeKey, definitions: &HashMap<Uuid, AttributeKey>) -> f64 {
    player.attributes().iter().find(|entry| definitions.get(&entry.attribute_definition_id()) == Some(&key))
        .map_or(10.0, |entry| f64::from(entry.value()))
}

fn manager_attribute(manager: &Manager, key: AttributeKey, definitions: &HashMap<Uuid, AttributeKey>) -> f64 {
    manager.attributes().iter()
        .find(|entry| definitions.get(&entry.attribute_definition_id()) == Some(&key))
        .map_or(10.0, |entry| f64::from(entry.value()))
}
