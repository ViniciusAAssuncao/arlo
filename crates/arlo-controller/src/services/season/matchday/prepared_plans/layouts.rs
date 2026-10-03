use super::repertoire::{self, PreparationPolicy};
use arlo_domain::{AttributeKey, Formation, Player, PositionLine, SlotRole};
use arlo_engine::TeamInput;
use arlo_tactics::{
    adapt_layout, max_concurrent_count, position_proficiency, static_role_fit, Mentality,
    PlayerInstructions, SlotAssignment, TacticalLayout, TacticalLineup, TeamInstructions,
    TeamTacticalProfile,
};
use std::collections::HashMap;
use uuid::Uuid;

pub(super) fn prepare_layout(
    team: &TeamInput,
    formation: &Formation,
    attributes: &HashMap<Uuid, AttributeKey>,
    policy: &PreparationPolicy,
) -> Option<(TeamTacticalProfile, TacticalLayout)> {
    let preferred = team
        .manager()
        .tactical_profile()
        .and_then(|profile| {
            profile
                .preferred_formation_ids()
                .iter()
                .position(|id| *id == formation.id())
        })
        .map_or(0.0, |rank| 0.08 / (rank as f64 + 1.0));
    let change = shape(formation) - shape(team.formation());
    let familiarity_cost = change.abs() * (1.0 - policy.flexibility) * 0.10;
    let opportunity = change * policy.relative_strength * policy.flexibility * 0.25;
    let base = team.tactics().instructions();
    let instructions = TeamInstructions::builder(base.in_possession().mentality())
        .with_in_possession(*base.in_possession())
        .with_out_of_possession(*base.out_of_possession())
        .with_transition(*base.transition())
        .with_mentality(Mentality::new_clamped(
            base.in_possession().mentality().value() + change * policy.flexibility * 0.5,
        ))
        .build();
    let name = format!("{}: {}", team.tactics().name(), formation.name());
    let profile = repertoire::profile(team, name, instructions, formation.id().as_u128());
    let quality = |player: &Player, formation: &Formation, index: usize| {
        let slot = &formation.slots()[index];
        let attribute = |key| {
            player
                .attributes()
                .iter()
                .find(|value| attributes.get(&value.attribute_definition_id()) == Some(&key))
                .map_or(10.0, |value| f64::from(value.value()))
        };
        (static_role_fit(
            player,
            slot.offensive_position(),
            slot.role(),
            profile.instructions(),
            &attribute,
        ) + static_role_fit(
            player,
            slot.defensive_position(),
            slot.role(),
            profile.instructions(),
            &attribute,
        )) * 0.5
    };
    let assignments = formation
        .slots()
        .iter()
        .enumerate()
        .map(|(index, slot)| {
            SlotAssignment::new(
                index,
                slot.offensive_position(),
                team.lineup().assignments()[index].player_id(),
                slot.role(),
                PlayerInstructions::default(),
            )
        })
        .collect();
    let target = TacticalLayout {
        formation: formation.clone(),
        lineup: TacticalLineup::new(
            profile.id(),
            team.team_id(),
            formation.id(),
            profile.name(),
            assignments,
        ),
    };
    let initial = TacticalLayout {
        formation: team.formation().clone(),
        lineup: team.lineup().clone(),
    };
    let layout = adapt_layout(
        team.lineup(),
        &initial,
        &target,
        |_| true,
        |anchor, index| {
            let player = team
                .roster()
                .iter()
                .find(|player| player.id() == anchor)
                .expect("validated starter");
            let position = formation.slots()[index].offensive_position();
            let old = team
                .lineup()
                .assignment_for_player(anchor)
                .expect("validated starter");
            if position != old.position() && position_proficiency(player, position) < 0.4 {
                return -1.0;
            }
            quality(player, formation, index)
        },
    )?;
    for role in [
        SlotRole::FalseArtrine,
        SlotRole::Launcher,
        SlotRole::Safeguard,
        SlotRole::Kicker,
        SlotRole::Blocker,
    ] {
        if max_concurrent_count(role).is_some_and(|limit| {
            layout
                .lineup
                .assignments()
                .iter()
                .filter(|slot| slot.slot_role() == role)
                .count()
                > limit as usize
        }) {
            return None;
        }
    }
    let fit = |layout: &TacticalLayout| {
        layout
            .lineup
            .assignments()
            .iter()
            .map(|slot| {
                let player = team
                    .roster()
                    .iter()
                    .find(|player| player.id() == slot.player_id())
                    .expect("validated starter");
                quality(player, &layout.formation, slot.formation_slot_index())
            })
            .sum::<f64>()
            / 14.0
    };
    let gain = fit(&layout) - fit(&initial);
    let role_changes = layout
        .lineup
        .assignments()
        .iter()
        .filter(|slot| {
            let old = team
                .lineup()
                .assignment_for_player(slot.player_id())
                .expect("validated starter");
            let original = &team.formation().slots()[old.formation_slot_index()];
            let target = &layout.formation.slots()[slot.formation_slot_index()];
            old.position() != slot.position()
                || old.slot_role() != slot.slot_role()
                || original.defensive_position() != target.defensive_position()
        })
        .count() as f64
        / 14.0;
    let cost = familiarity_cost + role_changes * (1.0 - policy.flexibility) * 0.04;
    let coverage = (change.abs() + role_changes * 0.25).min(1.0)
        * policy.flexibility
        * (0.04 + policy.knowledge * 0.10);
    if role_changes == 0.0
        || gain < -(0.025 + policy.flexibility * 0.035)
        || gain + preferred + opportunity + coverage - cost
            < 0.006 + (1.0 - policy.knowledge) * 0.008 - policy.flexibility * 0.010
    {
        return None;
    }
    Some((profile, layout))
}

fn shape(formation: &Formation) -> f64 {
    formation
        .slots()
        .iter()
        .map(|slot| match slot.offensive_position().line() {
            PositionLine::OffensiveLine => 1.0,
            PositionLine::DefenseLine => -1.0,
            _ => 0.0,
        })
        .sum::<f64>()
        / 14.0
}
