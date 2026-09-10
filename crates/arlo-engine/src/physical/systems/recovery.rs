use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::physical::state::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_recovery_tau(stamina: f64, natural_fitness: f64) -> f64 {
    let norm_fitness =
        (natural_fitness.clamp(0.0, 20.0) * 0.6 + stamina.clamp(0.0, 20.0) * 0.4) / 20.0;
    let tau = 180.0 - (130.0 * norm_fitness);
    tau.clamp(40.0, 240.0)
}

pub fn calculate_player_recovery_tau_from_table(table: &PlayerAttributeTable) -> f64 {
    let stamina = extract_attribute_value(table, AttributeKey::Stamina);
    let natural_fitness = extract_attribute_value(table, AttributeKey::NaturalFitness);
    calculate_recovery_tau(stamina, natural_fitness)
}

pub fn calculate_player_recovery_tau(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_recovery_tau_from_table(&table)
}

pub fn recover_w_prime(
    current_w_prime_balance: f64,
    dead_ball_seconds: f64,
    stamina: f64,
    natural_fitness: f64,
) -> f64 {
    if dead_ball_seconds <= 0.0 || current_w_prime_balance >= 1.0 {
        return current_w_prime_balance.clamp(0.0, 1.0);
    }
    let tau = calculate_recovery_tau(stamina, natural_fitness);
    let deficit = (1.0 - current_w_prime_balance).max(0.0);
    let recovered_deficit = deficit * (-dead_ball_seconds / tau).exp();
    (1.0 - recovered_deficit).clamp(0.0, 1.0)
}

pub fn recover_player_physical_state_from_table(
    state: &mut PhysicalState,
    table: &PlayerAttributeTable,
    dead_ball_seconds: f64,
) {
    let stamina = extract_attribute_value(table, AttributeKey::Stamina);
    let natural_fitness = extract_attribute_value(table, AttributeKey::NaturalFitness);
    let new_w_prime = recover_w_prime(
        state.w_prime_balance(),
        dead_ball_seconds,
        stamina,
        natural_fitness,
    );
    state.set_w_prime_balance(new_w_prime);
}

pub fn recover_player_physical_state(
    state: &mut PhysicalState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    dead_ball_seconds: f64,
) {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    recover_player_physical_state_from_table(state, &table, dead_ball_seconds);
}

pub fn recover_team_physical_states_from_tables(
    states: &mut HashMap<Uuid, PhysicalState>,
    players: &[&Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    dead_ball_seconds: f64,
) {
    if dead_ball_seconds <= 0.0 {
        return;
    }
    for player in players {
        let state = states.entry(player.id()).or_default();
        let table = attribute_tables
            .get(&player.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        recover_player_physical_state_from_table(state, table, dead_ball_seconds);
    }
}

pub fn recover_team_physical_states(
    states: &mut HashMap<Uuid, PhysicalState>,
    players: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    dead_ball_seconds: f64,
) {
    if dead_ball_seconds <= 0.0 {
        return;
    }
    for player in players {
        let state = states.entry(player.id()).or_default();
        recover_player_physical_state(state, player, attribute_keys, dead_ball_seconds);
    }
}

pub fn apply_intra_match_recovery_from_tables(
    home_fatigue: &mut HashMap<Uuid, PhysicalState>,
    away_fatigue: &mut HashMap<Uuid, PhysicalState>,
    home_players: &[&Player],
    away_players: &[&Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    dead_ball_seconds: f64,
) {
    recover_team_physical_states_from_tables(
        home_fatigue,
        home_players,
        attribute_tables,
        dead_ball_seconds,
    );
    recover_team_physical_states_from_tables(
        away_fatigue,
        away_players,
        attribute_tables,
        dead_ball_seconds,
    );
}

pub fn apply_intra_match_recovery(
    home_fatigue: &mut HashMap<Uuid, PhysicalState>,
    away_fatigue: &mut HashMap<Uuid, PhysicalState>,
    home_players: &[&Player],
    away_players: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    dead_ball_seconds: f64,
) {
    recover_team_physical_states(
        home_fatigue,
        home_players,
        attribute_keys,
        dead_ball_seconds,
    );
    recover_team_physical_states(
        away_fatigue,
        away_players,
        attribute_keys,
        dead_ball_seconds,
    );
}
