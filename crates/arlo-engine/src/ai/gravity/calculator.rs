use crate::ai::gravity::model::OffensiveGravity;
use crate::attributes::profiles::{
    gravity_creation_threat_profile, gravity_finishing_threat_profile,
};
use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use crate::physical::PhysicalState;
use arlo_domain::{Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_zone_factor(assigned_position: Position) -> f64 {
    match assigned_position {
        Position::CenterOffense => 1.25,
        Position::WingOffense | Position::WideEnd => 1.15,
        Position::TightWing | Position::RunningEnd | Position::Corridor => 1.05,
        _ => 1.00,
    }
}

pub fn calculate_player_offensive_gravity(
    player: &Player,
    table: &PlayerAttributeTable,
    assigned_position: Position,
    zone_factor: f64,
    state: &PhysicalState,
) -> OffensiveGravity {
    let finishing_profile = gravity_finishing_threat_profile();
    let creation_profile = gravity_creation_threat_profile();

    let deg_ctx = DegradationContext::new(state);

    let finishing_avg = finishing_profile.evaluate_saturated_average(|key| {
        extract_effective_attribute_value(table, key, &deg_ctx)
    });

    let creation_avg = creation_profile.evaluate_saturated_average(|key| {
        extract_effective_attribute_value(table, key, &deg_ctx)
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

pub fn calculate_team_max_finishing_gravity_with_fatigue_from_tables<F>(
    players: &[&Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    position_index: &HashMap<Uuid, Position>,
    _pitch: &arlo_domain::pitch::Pitch,
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

        let zone_factor = calculate_zone_factor(assigned_pos);
        let state = fatigue_for(&player.id());
        let table = attribute_tables
            .get(&player.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

        let grav = calculate_player_offensive_gravity(
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

pub fn calculate_team_max_finishing_gravity(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    pitch: &arlo_domain::pitch::Pitch,
    attribute_keys: &HashMap<Uuid, arlo_domain::AttributeKey>,
    attacking_positive_x: bool,
) -> OffensiveGravity {
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
        &|_| PhysicalState::initial(),
    )
}