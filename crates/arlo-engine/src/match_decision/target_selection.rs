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

pub fn player_base_reception_weight_with_state(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    role: ReceptionRole,
    state: &PhysicalState,
) -> f64 {
    match role {
        ReceptionRole::OpenPlayReceiver => {
            let hands = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::HandsReception,
                state,
            );
            let ant = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Anticipation,
                state,
            );
            let pos = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Positioning,
                state,
            );
            (hands * 0.45 + ant * 0.35 + pos * 0.20).max(0.1)
        }
        ReceptionRole::Finisher => {
            let finishing = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Finishing,
                state,
            );
            let technique = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Technique,
                state,
            );
            let composure = extract_effective_attribute_value(
                player,
                attribute_keys,
                AttributeKey::Composure,
                state,
            );
            (finishing * 0.50 + technique * 0.30 + composure * 0.20).max(0.1)
        }
    }
}

pub fn player_base_reception_weight(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    role: ReceptionRole,
) -> f64 {
    player_base_reception_weight_with_state(player, attribute_keys, role, &PhysicalState::initial())
}

pub fn calculate_player_target_weight_with_state(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    state: &PhysicalState,
) -> f64 {
    let base_weight = player_base_reception_weight_with_state(player, attribute_keys, role, state);
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
    base_weight * proximity_factor * fit_mult * priority_mult
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
) -> f64 {
    calculate_player_target_weight_with_state(
        player,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        role,
        &PhysicalState::initial(),
    )
}

pub fn select_target_with_fatigue<F, R>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    fatigue_for: &F,
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

    let weights: Vec<f64> = candidates
        .iter()
        .map(|p| {
            let state = fatigue_for(&p.id());
            calculate_player_target_weight_with_state(
                p,
                spatial_map,
                pitch,
                position_index,
                instructions_index,
                attribute_keys,
                attacking_positive_x,
                role,
                &state,
            )
        })
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}

pub fn select_target<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    rng: &mut R,
) -> Option<Uuid> {
    select_target_with_fatigue(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        role,
        &|_| PhysicalState::initial(),
        rng,
    )
}
