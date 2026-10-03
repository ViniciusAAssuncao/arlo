use super::AuditResult;
use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, Manager,
    ManagerAttributeValue, Player, PlayerPosition, Position,
};
use arlo_engine::{MatchInput, TeamInput};
use arlo_tactics::{PreparedTacticalPlan, SlotAssignment, TacticalLayout, TacticalLineup};
use std::collections::HashMap;
use std::sync::Arc;

pub fn rebuild(input: &MatchInput, home: TeamInput) -> AuditResult<MatchInput> {
    rebuild_seed(input, home, input.seed())
}

pub fn rebuild_seed(input: &MatchInput, home: TeamInput, seed: u64) -> AuditResult<MatchInput> {
    let energy = home
        .roster()
        .iter()
        .chain(input.away().roster())
        .map(|player| (player.id(), input.player_start_energy(player.id())))
        .collect();
    let morale = home
        .roster()
        .iter()
        .chain(input.away().roster())
        .map(|player| (player.id(), input.player_start_morale(player.id())))
        .collect();
    Ok(MatchInput::new(
        input.match_id(),
        home,
        input.away().clone(),
        input.format(),
        input.pitch(),
        input.referees().to_vec(),
        Arc::new(input.referee_attribute_keys().clone()),
        Arc::new(input.fault_catalog().clone()),
        Arc::new(input.injury_catalog().clone()),
        input.player_attribute_definitions().to_vec(),
        seed,
    )?
    .with_manager_decision_context(Arc::new(input.manager_attribute_keys().clone()), energy)?
    .with_player_start_morale(morale)?)
}

pub fn manager(input: &MatchInput, value: i32) -> AuditResult<MatchInput> {
    alter_manager(input, |_, _| value)
}

pub fn attribute(input: &MatchInput, target: AttributeKey, value: i32) -> AuditResult<MatchInput> {
    alter_manager(input, |key, old| if key == target { value } else { old })
}

fn alter_manager(
    input: &MatchInput,
    value: impl Fn(AttributeKey, i32) -> i32,
) -> AuditResult<MatchInput> {
    let team = input.home();
    let attributes = team
        .manager()
        .attributes()
        .iter()
        .map(|attribute| {
            let id = attribute.attribute_definition_id();
            let key = *input
                .manager_attribute_keys()
                .get(&id)
                .expect("manager attribute key");
            let definition = AttributeDefinition::new(
                id,
                key,
                "Audit attribute",
                AttributeCategory::Managerial,
                AttributeTarget::Manager,
            )?;
            ManagerAttributeValue::new(&definition, value(key, attribute.value()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let manager = Manager::new(
        team.manager().person().clone(),
        Some(team.team_id()),
        team.manager().control_mode(),
        attributes,
        team.manager().tactical_profile().cloned(),
    )?;
    let team = TeamInput::new(
        team.team_id(),
        team.formation().clone(),
        team.lineup().clone(),
        team.roster().to_vec(),
        manager,
        team.tactics().clone(),
    )?
    .with_alternative_tactics(team.tactical_profiles().skip(1).cloned().collect())?
    .with_prepared_plans(team.prepared_plans().to_vec())?;
    rebuild(input, team)
}

pub fn misaligned(input: &MatchInput) -> AuditResult<MatchInput> {
    let input = manager(input, 20)?;
    let team = input.home();
    let slots = team.lineup().assignments();
    let (a, b) = slots
        .iter()
        .enumerate()
        .flat_map(|(i, a)| slots.iter().skip(i + 1).map(move |b| (a, b)))
        .find(|(a, b)| {
            let sa = &team.formation().slots()[a.formation_slot_index()];
            let sb = &team.formation().slots()[b.formation_slot_index()];
            !mandatory(a.position())
                && !mandatory(b.position())
                && a.position() != b.position()
                && [sa.offensive_position(), sa.defensive_position()]
                    .iter()
                    .all(|position| {
                        ![sb.offensive_position(), sb.defensive_position()].contains(position)
                    })
        })
        .ok_or("No independent nonmandatory roles for misalignment scenario")?;
    let mut roster = team.roster().to_vec();
    for (own, other) in [(a, b), (b, a)] {
        let player = roster
            .iter_mut()
            .find(|player| player.id() == own.player_id())
            .ok_or("Missing scenario player")?;
        let own_slot = &team.formation().slots()[own.formation_slot_index()];
        let other_slot = &team.formation().slots()[other.formation_slot_index()];
        let mut positions: HashMap<_, _> = player
            .positions()
            .iter()
            .map(|position| (position.position(), position.proficiency()))
            .collect();
        for position in [
            other_slot.offensive_position(),
            other_slot.defensive_position(),
        ] {
            positions.insert(position, 4);
        }
        for position in [own_slot.offensive_position(), own_slot.defensive_position()] {
            positions.insert(position, 10);
        }
        let mut positions = positions
            .into_iter()
            .map(|(position, value)| PlayerPosition::new(position, value))
            .collect::<Result<Vec<_>, _>>()?;
        positions.sort_by_key(|position| format!("{:?}", position.position()));
        *player = Player::builder(
            player.id(),
            player.name(),
            player.height_m(),
            player.birthdate_unix_seconds(),
            player.nationality_id(),
        )
        .with_team_id(player.team_id())
        .with_squad_number(player.squad_number())
        .with_captaincy_role(player.captaincy_role())
        .with_attributes(player.attributes().to_vec())
        .with_positions(positions)
        .build()?;
    }
    let assignments = slots
        .iter()
        .map(|slot| {
            SlotAssignment::new(
                slot.formation_slot_index(),
                slot.position(),
                if slot.player_id() == a.player_id() {
                    b.player_id()
                } else if slot.player_id() == b.player_id() {
                    a.player_id()
                } else {
                    slot.player_id()
                },
                slot.slot_role(),
                *slot.player_instructions(),
            )
        })
        .collect();
    let lineup = TacticalLineup::new(
        team.lineup().id(),
        team.team_id(),
        team.formation().id(),
        "Audited misalignment",
        assignments,
    );
    let primary = PreparedTacticalPlan {
        id: team.tactics().id(),
        name: team.tactics().name().to_owned(),
        profile_id: team.tactics().id(),
        layout: TacticalLayout {
            formation: team.formation().clone(),
            lineup: lineup.clone(),
        },
    };
    let team = TeamInput::new(
        team.team_id(),
        team.formation().clone(),
        lineup,
        roster,
        team.manager().clone(),
        team.tactics().clone(),
    )?
    .with_prepared_plans(vec![primary])?;
    rebuild(&input, team)
}

fn mandatory(position: Position) -> bool {
    matches!(
        position,
        Position::Artrine | Position::Passer | Position::Goalguard
    )
}
