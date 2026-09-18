use crate::ai::gravity::model::OffensiveGravity;
use crate::attributes::profiles::{
    gravity_creation_threat_profile, gravity_finishing_threat_profile,
};
use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIRST_ZONE_DEPTH_MIRIM,
};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_offensive_gravity_with_state_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
    assigned_position: Position,
    zone_factor: f64,
    state: &PhysicalState,
) -> OffensiveGravity {
    let finishing_profile = gravity_finishing_threat_profile();
    let creation_profile = gravity_creation_threat_profile();

    let finishing_avg = finishing_profile.evaluate_saturated_average(|key| {
        extract_effective_attribute_value(table, key, state)
    });

    let creation_avg = creation_profile.evaluate_saturated_average(|key| {
        extract_effective_attribute_value(table, key, state)
    });

    let fit = calculate_fit_for_position(player, assigned_position);
    let fit_mult = fit.efficiency_multiplier();

    let finishing_threat = (finishing_avg / 10.0).powf(1.4) * fit_mult;
    let creation_threat = (creation_avg / 10.0) * fit_mult;

    let position_role_weight = match assigned_position {
        Position::CenterOffense => 0.85,
        Position::WingOffense | Position::WideEnd => 0.65,
        Position::TightWing | Position::RunningEnd | Position::Corridor => 0.50,
        _ => 0.30,
    };

    let composite_threat = (finishing_threat * position_role_weight)
        + (creation_threat * (1.0 - position_role_weight));

    let raw_curve = 0.50 + 1.50 / (1.0 + (-2.2 * (composite_threat - 1.15)).exp());
    let multiplier = (raw_curve * zone_factor).clamp(0.5, 2.0);

    OffensiveGravity::new(multiplier, finishing_threat, creation_threat, zone_factor)
}

pub fn calculate_player_offensive_gravity_with_state(
    player: &Player,
    assigned_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    zone_factor: f64,
    state: &PhysicalState,
) -> OffensiveGravity {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_offensive_gravity_with_state_from_table(
        player,
        &table,
        assigned_position,
        zone_factor,
        state,
    )
}

pub fn calculate_player_offensive_gravity(
    player: &Player,
    assigned_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    zone_factor: f64,
) -> OffensiveGravity {
    calculate_player_offensive_gravity_with_state(
        player,
        assigned_position,
        attribute_keys,
        zone_factor,
        &PhysicalState::initial(),
    )
}

pub fn calculate_zone_factor(
    player_pos: VectorPosition,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let pitch_length_m = pitch.length().value();
    let player_x_m = player_pos.raw().0;

    let dist_to_goal_m = if attacking_positive_x {
        (pitch_length_m - player_x_m).max(0.0)
    } else {
        player_x_m.max(0.0)
    };

    let first_zone_limit_m = FIRST_ZONE_DEPTH_MIRIM * MIRIM_TO_METERS;
    let second_zone_limit_m =
        (FIRST_ZONE_DEPTH_MIRIM + AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM) * MIRIM_TO_METERS;

    if dist_to_goal_m <= first_zone_limit_m {
        1.25
    } else if dist_to_goal_m <= second_zone_limit_m {
        1.15
    } else if dist_to_goal_m <= second_zone_limit_m + (10.0 * MIRIM_TO_METERS) {
        1.05
    } else {
        1.00
    }
}

pub fn calculate_team_max_finishing_gravity_with_fatigue_from_tables<F>(
    players: &[&Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    position_index: &HashMap<Uuid, Position>,
    _pitch: &Pitch,
    _attacking_positive_x: bool,
    fatigue_for: &F,
) -> OffensiveGravity
where
    F: Fn(&Uuid) -> PhysicalState,
{
    if players.is_empty() {
        return OffensiveGravity::default();
    }

    let mut best_gravity = OffensiveGravity::default();

    for &player in players {
        let assigned_pos = position_index
            .get(&player.id())
            .copied()
            .unwrap_or_else(|| {
                player
                    .positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::CenterOffense)
            });

        let zone_factor = match assigned_pos {
            Position::CenterOffense => 1.25,
            Position::WingOffense | Position::WideEnd => 1.15,
            Position::TightWing | Position::RunningEnd | Position::Corridor => 1.05,
            _ => 1.00,
        };

        let state = fatigue_for(&player.id());
        let table = attribute_tables
            .get(&player.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

        let grav = calculate_player_offensive_gravity_with_state_from_table(
            player,
            table,
            assigned_pos,
            zone_factor,
            &state,
        );

        if grav.multiplier() > best_gravity.multiplier() {
            best_gravity = grav;
        }
    }

    best_gravity
}

pub fn calculate_team_max_finishing_gravity_with_fatigue<F>(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    fatigue_for: &F,
) -> OffensiveGravity
where
    F: Fn(&Uuid) -> PhysicalState,
{
    let mut attribute_tables = HashMap::with_capacity(players.len());
    for p in players {
        attribute_tables.insert(p.id(), PlayerAttributeTable::from_player(p, attribute_keys));
    }
    calculate_team_max_finishing_gravity_with_fatigue_from_tables(
        players,
        &attribute_tables,
        position_index,
        pitch,
        attacking_positive_x,
        fatigue_for,
    )
}

pub fn calculate_team_max_finishing_gravity(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
) -> OffensiveGravity {
    calculate_team_max_finishing_gravity_with_fatigue(
        players,
        position_index,
        pitch,
        attribute_keys,
        attacking_positive_x,
        &|_| PhysicalState::initial(),
    )
}
