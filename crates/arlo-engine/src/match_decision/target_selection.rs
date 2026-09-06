use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::DynamicSpatialMap;
use crate::tactics::calculate_fit_for_position;
use arlo_domain::{AttributeKey, Pitch, Player, Position};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptionRole {
    Finisher,
    OpenPlayReceiver,
}

pub fn player_base_reception_weight(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    role: ReceptionRole,
) -> f64 {
    match role {
        ReceptionRole::OpenPlayReceiver => {
            let hands = extract_attribute_value(player, attribute_keys, AttributeKey::HandsReception);
            let ant = extract_attribute_value(player, attribute_keys, AttributeKey::Anticipation);
            let pos = extract_attribute_value(player, attribute_keys, AttributeKey::Positioning);
            (hands * 0.45 + ant * 0.35 + pos * 0.20).max(0.1)
        }
        ReceptionRole::Finisher => {
            let finishing = extract_attribute_value(player, attribute_keys, AttributeKey::Finishing);
            let technique = extract_attribute_value(player, attribute_keys, AttributeKey::Technique);
            let composure = extract_attribute_value(player, attribute_keys, AttributeKey::Composure);
            (finishing * 0.50 + technique * 0.30 + composure * 0.20).max(0.1)
        }
    }
}

pub fn calculate_player_target_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
) -> f64 {
    let base_weight = player_base_reception_weight(player, attribute_keys, role);
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
    base_weight * proximity_factor * fit_mult
}

pub fn select_target<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    rng: &mut R,
) -> Option<Uuid> {
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return Some(candidates[0].id());
    }

    let weights: Vec<f64> = candidates
        .iter()
        .map(|p| {
            calculate_player_target_weight(
                p,
                spatial_map,
                pitch,
                position_index,
                attribute_keys,
                attacking_positive_x,
                role,
            )
        })
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}