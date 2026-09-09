use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::team_identity::marking::block_marking_role::BlockMarkingRole;
use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, BLOCK_MARKING_AGGRESSION_WEIGHT, BLOCK_MARKING_PROXIMITY_WEIGHT,
};
use arlo_domain::{AttributeKey, Manager, Player, Position, PositionLine};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{nearest_block_notation, PressBlockShape};
use std::collections::HashMap;
use uuid::Uuid;

pub fn eligible_block_marking_defenders<'a>(
    defenders: &[&'a Player],
    position_index: &HashMap<Uuid, Position>,
) -> Vec<&'a Player> {
    defenders
        .iter()
        .copied()
        .filter(|p| {
            let pos = position_index.get(&p.id()).copied().unwrap_or_else(|| {
                p.positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::Centerback)
            });
            matches!(
                pos.line(),
                PositionLine::DefenseLine | PositionLine::BackLine
            )
        })
        .collect()
}

pub fn extract_manager_attribute_value(
    manager: &Manager,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    target: AttributeKey,
) -> f64 {
    let target_id = attribute_keys
        .iter()
        .find_map(|(id, &key)| if key == target { Some(*id) } else { None });

    if let Some(target_id) = target_id {
        for attr in manager.attributes() {
            if attr.attribute_definition_id() == target_id {
                return attr.value() as f64;
            }
        }
    }
    10.0
}

pub fn extract_manager_artro_strategy_fidelity(
    manager: &Manager,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let raw = extract_manager_attribute_value(manager, attribute_keys, AttributeKey::ArtroStrategy);
    (raw / ATTRIBUTE_MAX).clamp(0.0, 1.0)
}

pub fn derive_block_marking_roles(
    eligible_defenders: &[&Player],
    reference_pos: VectorPosition,
    spatial_map: &DynamicSpatialMap,
    press_block_shape: PressBlockShape,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    execution_fidelity: f64,
) -> HashMap<Uuid, BlockMarkingRole> {
    if eligible_defenders.is_empty() {
        return HashMap::new();
    }

    let (bite_count, _) =
        nearest_block_notation(press_block_shape, eligible_defenders.len() as u32);
    let bite_count = (bite_count as usize).min(eligible_defenders.len());

    let mut scores = Vec::with_capacity(eligible_defenders.len());
    let mut total_raw_score = 0.0;

    for defender in eligible_defenders {
        let def_pos = spatial_map
            .get_position(&defender.id())
            .unwrap_or(reference_pos);
        let dist_mirim = calculate_distance_mirim(def_pos, reference_pos);
        let proximity_score = 1.0 / (1.0 + dist_mirim);
        let aggression_val =
            extract_attribute_value(defender, attribute_keys, AttributeKey::ControlledAggression);
        let norm_aggression = (aggression_val / ATTRIBUTE_MAX).clamp(0.0, 1.0);

        let raw_score = proximity_score * BLOCK_MARKING_PROXIMITY_WEIGHT
            + norm_aggression * BLOCK_MARKING_AGGRESSION_WEIGHT;

        total_raw_score += raw_score;
        scores.push((defender.id(), raw_score));
    }

    let mean_score = total_raw_score / (eligible_defenders.len() as f64);
    let fidelity = execution_fidelity.clamp(0.0, 1.0);

    let mut adjusted_scores: Vec<(Uuid, f64)> = scores
        .into_iter()
        .map(|(id, raw_score)| {
            let adjusted = raw_score * fidelity + mean_score * (1.0 - fidelity);
            (id, adjusted)
        })
        .collect();

    adjusted_scores.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut roles = HashMap::with_capacity(eligible_defenders.len());
    for (idx, (id, _)) in adjusted_scores.into_iter().enumerate() {
        let role = if idx < bite_count {
            BlockMarkingRole::Biter
        } else {
            BlockMarkingRole::Coverer
        };
        roles.insert(id, role);
    }

    roles
}
