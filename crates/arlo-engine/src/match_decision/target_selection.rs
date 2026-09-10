use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Pitch, Player, Position};
use arlo_math::stats::sample_categorical;
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptionRole {
    Finisher,
    OpenPlayReceiver,
}

pub fn player_base_reception_weight_from_table(
    table: &PlayerAttributeTable,
    role: ReceptionRole,
    state: Option<&PhysicalState>,
) -> f64 {
    let default_state = PhysicalState::initial();
    let effective_state = state.unwrap_or(&default_state);

    match role {
        ReceptionRole::OpenPlayReceiver => {
            let hands = extract_effective_attribute_value(
                table,
                AttributeKey::HandsReception,
                effective_state,
            );
            let ant = extract_effective_attribute_value(
                table,
                AttributeKey::Anticipation,
                effective_state,
            );
            let pos = extract_effective_attribute_value(
                table,
                AttributeKey::Positioning,
                effective_state,
            );
            (hands * 0.45 + ant * 0.35 + pos * 0.20).max(0.1)
        }
        ReceptionRole::Finisher => {
            let finishing = extract_effective_attribute_value(
                table,
                AttributeKey::Finishing,
                effective_state,
            );
            let technique = extract_effective_attribute_value(
                table,
                AttributeKey::Technique,
                effective_state,
            );
            let composure = extract_effective_attribute_value(
                table,
                AttributeKey::Composure,
                effective_state,
            );
            (finishing * 0.50 + technique * 0.30 + composure * 0.20).max(0.1)
        }
    }
}

pub fn player_base_reception_weight(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    role: ReceptionRole,
    state: Option<&PhysicalState>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    player_base_reception_weight_from_table(&table, role, state)
}

pub fn calculate_player_target_weight_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    state: Option<&PhysicalState>,
) -> f64 {
    let base_weight = player_base_reception_weight_from_table(table, role, state);
    let proximity_factor = match spatial_map.get_position(&player.id()) {
        Some(pos) => {
            let total_len = pitch.length().value();
            if total_len > 0.0 {
                let x = pos.raw().0;
                let normalized_x = if attacking_positive_x {
                    (x / total_len).clamp(0.0, 1.0)
                } else {
                    ((total_len - x) / total_len).clamp(0.0, 1.0)
                };
                0.5 + 1.5 * normalized_x
            } else {
                1.0
            }
        }
        None => 1.0,
    };
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
    let fit_mult = calculate_fit_for_position(player, assigned_pos).efficiency_multiplier();
    let priority_mult = 1.0
        + instructions_index
            .get(&player.id())
            .copied()
            .unwrap_or_default()
            .in_possession()
            .involvement_priority()
            .value();
    let openness = openness_by_player.get(&player.id()).copied().unwrap_or(1.0);
    base_weight * proximity_factor * fit_mult * priority_mult * openness
}

pub fn calculate_player_target_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    state: Option<&PhysicalState>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_target_weight_from_table(
        player,
        &table,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attacking_positive_x,
        role,
        openness_by_player,
        state,
    )
}

pub fn select_target_from_tables<F, R>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return Some(candidates[0].id());
    }

    let default_state = PhysicalState::initial();
    
    let weights: Vec<f64> = candidates
        .iter()
        .map(|p| {
            let state = match fatigue_for {
                Some(lookup) => lookup(&p.id()),
                None => default_state,
            };
            let table = attribute_tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            calculate_player_target_weight_from_table(
                p,
                table,
                spatial_map,
                pitch,
                position_index,
                instructions_index,
                attacking_positive_x,
                role,
                openness_by_player,
                Some(&state),
            )
        })
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}

pub fn select_target<F, R>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    let mut attribute_tables = HashMap::with_capacity(candidates.len());
    for p in candidates {
        attribute_tables.insert(p.id(), PlayerAttributeTable::from_player(p, attribute_keys));
    }
    select_target_from_tables(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        &attribute_tables,
        attacking_positive_x,
        role,
        openness_by_player,
        fatigue_for,
        rng,
    )
}
