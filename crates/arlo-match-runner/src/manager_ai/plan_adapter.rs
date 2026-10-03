use super::context::SlotContext;
use super::plan_context::{PlanContext, PlannedAssignment, TacticalEffect};
use arlo_domain::{Formation, PositionLine};
use arlo_engine::{preview_prepared_plan, MatchInput, MatchState, TeamInput, TeamState};
use arlo_tactics::{position_proficiency, static_role_fit, TeamInstructions};

pub(super) fn build(
    input: &MatchInput,
    state: &MatchState,
    team: &TeamInput,
    current: &TeamState,
    slots: &[SlotContext],
) -> Vec<PlanContext> {
    let mut contexts = Vec::new();
    for plan in team.prepared_plans() {
        if current.active_plan_id() == Some(plan.id) {
            continue;
        }
        let Ok(layout) = preview_prepared_plan(input, state, team.team_id(), plan.id) else {
            continue;
        };
        let Some(profile) = team
            .tactical_profiles()
            .find(|profile| profile.id() == plan.profile_id)
        else {
            continue;
        };
        let mut assignments = Vec::new();
        let mut changed_roles = 0;
        for target in layout.lineup.assignments() {
            let player_id = current.slot_player_id(target.player_id());
            let Some(current_index) = slots.iter().position(|slot| slot.player_id == player_id)
            else {
                continue;
            };
            let Some(player) = team.roster().iter().find(|player| player.id() == player_id) else {
                continue;
            };
            let slot = &layout.formation.slots()[target.formation_slot_index()];
            let attribute = |key| {
                input
                    .player_attribute_definitions()
                    .iter()
                    .find(|definition| definition.key() == key)
                    .and_then(|definition| {
                        player
                            .attributes()
                            .iter()
                            .find(|value| value.attribute_definition_id() == definition.id())
                    })
                    .map_or(10.0, |value| f64::from(value.value()))
            };
            let destination = SlotContext {
                player_id,
                offensive_position: slot.offensive_position(),
                defensive_position: slot.defensive_position(),
                role: target.slot_role(),
            };
            if destination.offensive_position != slots[current_index].offensive_position
                || destination.defensive_position != slots[current_index].defensive_position
                || destination.role != slots[current_index].role
            {
                changed_roles += 1;
            }
            assignments.push(PlannedAssignment {
                destination,
                current_index,
                quality: (static_role_fit(
                    player,
                    slot.offensive_position(),
                    target.slot_role(),
                    profile.instructions(),
                    &attribute,
                ) + static_role_fit(
                    player,
                    slot.defensive_position(),
                    target.slot_role(),
                    profile.instructions(),
                    &attribute,
                )) * 0.5,
                proficiency: position_proficiency(player, slot.offensive_position()),
            });
        }
        contexts.push(PlanContext {
            id: plan.id,
            effect: effect(profile.instructions(), &layout.formation),
            assignments,
            changed_roles,
            formation_changed: layout.formation.id() != current.formation(team).id(),
        });
    }
    contexts
}

pub(super) fn effect(instructions: &TeamInstructions, formation: &Formation) -> TacticalEffect {
    let attack = instructions.in_possession();
    let defense = instructions.out_of_possession();
    let mentality = (attack.mentality().value() + 1.0) * 0.5;
    let directness = (attack.directness().value() + 1.0) * 0.5;
    let structure = (attack.structure().value() + 1.0) * 0.5;
    let tempo = (attack.tempo().value() + 1.0) * 0.5;
    let shape = formation
        .slots()
        .iter()
        .map(|slot| match slot.offensive_position().line() {
            PositionLine::OffensiveLine => 1.0,
            PositionLine::DefenseLine => -1.0,
            _ => 0.0,
        })
        .sum::<f64>()
        / 14.0;
    let control =
        0.4 * (1.0 - directness) + 0.35 * attack.scoring_patience().value() + 0.25 * structure;
    TacticalEffect {
        attack: 0.5 * mentality
            + 0.2 * tempo
            + 0.2 * (shape + 1.0) * 0.5
            + 0.1 * instructions.transition().counter_attack_intensity().value(),
        defense: 0.4 * defense.compactness().value()
            + 0.35 * defense.pressing_intensity().value()
            + 0.25 * (1.0 - shape) * 0.5,
        control,
        security: 0.55 * control + 0.25 * defense.compactness().value() + 0.2 * (1.0 - tempo),
        load: 0.4 * tempo
            + 0.3 * defense.pressing_intensity().value()
            + 0.15 * attack.physicality().value()
            + 0.15 * instructions.transition().counter_press_intensity().value(),
        discipline: 0.6 * (1.0 - defense.aggression().value())
            + 0.4 * (1.0 - attack.physicality().value()),
    }
}
