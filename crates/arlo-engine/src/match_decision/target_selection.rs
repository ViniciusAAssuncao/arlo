use crate::spatial::DynamicSpatialMap;
use crate::tactics::position_profile_cache::get_position_profile;
use arlo_domain::{AttributeKey, Pitch, Player, Position};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptionRole {
    Finisher,
    OpenPlayReceiver,
}

pub fn position_reception_bias(position: Position, role: ReceptionRole) -> f64 {
    let profile = get_position_profile(position);
    let sum: f64 = profile
        .weights
        .iter()
        .filter(|w| match role {
            ReceptionRole::OpenPlayReceiver => matches!(
                w.key,
                AttributeKey::HandsReception
                    | AttributeKey::Anticipation
                    | AttributeKey::Positioning
            ),
            ReceptionRole::Finisher => matches!(
                w.key,
                AttributeKey::Finishing
                    | AttributeKey::Technique
                    | AttributeKey::Composure
            ),
        })
        .map(|w| w.weight)
        .sum();

    if sum > 0.0 {
        sum
    } else {
        0.1
    }
}

pub fn player_base_reception_weight(player: &Player, role: ReceptionRole) -> f64 {
    let mut best_weight = 0.1;
    for pos in player.positions() {
        let weight = position_reception_bias(pos.position(), role);
        let prof_factor = (pos.proficiency() as f64) / 10.0;
        let weighted = weight * (0.5 + 0.5 * prof_factor);
        if weighted > best_weight {
            best_weight = weighted;
        }
    }
    best_weight
}

pub fn calculate_player_target_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attacking_positive_x: bool,
    role: ReceptionRole,
) -> f64 {
    let base_weight = player_base_reception_weight(player, role);
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
    base_weight * proximity_factor
}

pub fn select_target<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
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
        .map(|p| calculate_player_target_weight(p, spatial_map, pitch, attacking_positive_x, role))
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}