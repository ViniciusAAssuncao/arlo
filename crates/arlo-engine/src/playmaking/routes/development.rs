use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::physical::systems::degradation::calculate_effective_player_speed_from_table;
use crate::physical::FatigueState;
use crate::playmaking::routes::geometry::resolve_route_waypoints;
use crate::spatial::decision_vector::derive_velocity_towards_target;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{ATTRIBUTE_MAX, MAN_COVERAGE_OPENNESS_PENALTY};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::units::{Duration, Position as VectorPosition, Velocity};
use arlo_tactics::{MarkingAssignment, PlayerInstructions, RouteAssignment};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn simulate_route_development_from_tables<F, R>(
    pitch: &Pitch,
    attacking_positive_x: bool,
    offense_route_runners: &[&Player],
    route_index: &HashMap<Uuid, RouteAssignment>,
    offense_position_index: &HashMap<Uuid, Position>,
    defenders: &[&Player],
    _defense_position_index: &HashMap<Uuid, Position>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    spatial_map: &mut DynamicSpatialMap,
    fatigue_for: &F,
    available_duration: Duration,
    _rng: &mut R,
) -> HashMap<Uuid, f64>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    for runner in offense_route_runners {
        if let Some(route) = route_index.get(&runner.id()) {
            let start_pos = spatial_map
                .get_position(&runner.id())
                .unwrap_or_else(VectorPosition::zero);
            let waypoints = resolve_route_waypoints(start_pos, route, pitch, attacking_positive_x);
            let d1 = (waypoints.stem_point.raw() - start_pos.raw()).magnitude();
            let d2 = (waypoints.break_point.raw() - waypoints.stem_point.raw()).magnitude();

            let fatigue = fatigue_for(&runner.id());
            let table = attribute_tables
                .get(&runner.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let effective_speed =
                calculate_effective_player_speed_from_table(runner, table, &fatigue);
            let distance_traveled = effective_speed.value() * available_duration.value();

            let (final_pos, vel) = if distance_traveled <= 0.0 {
                (start_pos, Velocity::zero())
            } else if distance_traveled < d1 {
                let t = if d1 > 1e-6 {
                    distance_traveled / d1
                } else {
                    1.0
                };
                let final_pos_raw =
                    start_pos.raw() + (waypoints.stem_point.raw() - start_pos.raw()) * t;
                let final_pos = VectorPosition::from_raw(final_pos_raw);
                let vel = derive_velocity_towards_target(
                    final_pos,
                    waypoints.stem_point,
                    effective_speed,
                );
                (final_pos, vel)
            } else if distance_traveled < d1 + d2 {
                let remaining_d = distance_traveled - d1;
                let t = if d2 > 1e-6 { remaining_d / d2 } else { 1.0 };
                let final_pos_raw = waypoints.stem_point.raw()
                    + (waypoints.break_point.raw() - waypoints.stem_point.raw()) * t;
                let final_pos = VectorPosition::from_raw(final_pos_raw);
                let vel = derive_velocity_towards_target(
                    final_pos,
                    waypoints.break_point,
                    effective_speed,
                );
                (final_pos, vel)
            } else {
                let final_pos = waypoints.break_point;
                let vel = derive_velocity_towards_target(
                    waypoints.stem_point,
                    waypoints.break_point,
                    effective_speed,
                );
                (final_pos, vel)
            };

            spatial_map.set_position(runner.id(), final_pos);
            spatial_map.set_velocity(runner.id(), vel);
        }
    }

    let mut openness_map = HashMap::with_capacity(offense_route_runners.len());
    for runner in offense_route_runners {
        if let Some(route) = route_index.get(&runner.id()) {
            let runner_table = attribute_tables
                .get(&runner.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let runner_pace = runner_table.get(AttributeKey::Pace) / ATTRIBUTE_MAX;
            let runner_acc = runner_table.get(AttributeKey::Acceleration) / ATTRIBUTE_MAX;
            let runner_agility = runner_table.get(AttributeKey::Agility) / ATTRIBUTE_MAX;

            let runner_score = runner_pace * 0.40 + runner_acc * 0.35 + runner_agility * 0.25;

            let runner_formational_pos = offense_position_index.get(&runner.id());
            let is_man_marked = if let Some(&pos) = runner_formational_pos {
                defenders.iter().any(|def| {
                    defense_instructions_index
                        .get(&def.id())
                        .and_then(|inst| inst.out_of_possession().marking())
                        .map(|m| m == MarkingAssignment::Man(pos))
                        .unwrap_or(false)
                })
            } else {
                false
            };

            let base_control = (0.35 + runner_score * 0.45).clamp(0.10, 0.95);
            let penalized_control = if is_man_marked {
                (base_control - MAN_COVERAGE_OPENNESS_PENALTY).max(0.05)
            } else {
                base_control
            };

            let read_priority_val = route.read_priority().value();
            let final_openness = penalized_control * (1.0 + read_priority_val);
            openness_map.insert(runner.id(), final_openness);
        }
    }

    openness_map
}

pub fn simulate_route_development<F, R>(
    pitch: &Pitch,
    attacking_positive_x: bool,
    offense_route_runners: &[&Player],
    route_index: &HashMap<Uuid, RouteAssignment>,
    offense_position_index: &HashMap<Uuid, Position>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, Position>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    spatial_map: &mut DynamicSpatialMap,
    fatigue_for: &F,
    available_duration: Duration,
    rng: &mut R,
) -> HashMap<Uuid, f64>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let mut attribute_tables = HashMap::new();
    for r in offense_route_runners {
        attribute_tables.insert(r.id(), PlayerAttributeTable::from_player(r, attribute_keys));
    }
    for d in defenders {
        attribute_tables.insert(d.id(), PlayerAttributeTable::from_player(d, attribute_keys));
    }

    simulate_route_development_from_tables(
        pitch,
        attacking_positive_x,
        offense_route_runners,
        route_index,
        offense_position_index,
        defenders,
        defense_position_index,
        defense_instructions_index,
        &attribute_tables,
        spatial_map,
        fatigue_for,
        available_duration,
        rng,
    )
}
