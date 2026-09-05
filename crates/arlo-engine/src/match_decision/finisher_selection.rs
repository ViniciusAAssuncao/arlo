use crate::spatial::DynamicSpatialMap;
use arlo_domain::{Pitch, Player, Position};
use arlo_math::stats::sample_categorical;
use rand::Rng;
use uuid::Uuid;

pub fn position_finishing_bias(position: Position) -> f64 {
    match position {
        Position::CenterOffense => 5.0,
        Position::WingOffense => 3.5,
        Position::Corridor => 3.0,
        Position::Midcenter => 2.5,
        Position::WideEnd => 2.5,
        Position::RunningEnd => 2.5,
        Position::TightWing => 2.0,
        Position::Artrine => 2.0,
        Position::CenterTight => 1.5,
        Position::Passer => 1.0,
        Position::Lineback => 1.0,
        Position::Fullback => 1.0,
        Position::PassRusher => 0.8,
        Position::Centerback
        | Position::DefensiveEnd
        | Position::Rougieback
        | Position::DefensiveBlocker
        | Position::WideBlocker
        | Position::OutsideZonerback
        | Position::MiddleZonerback => 0.5,
        Position::Goalguard => 0.1,
    }
}

pub fn player_base_finishing_weight(player: &Player) -> f64 {
    let mut best_weight = 0.5;
    for pos in player.positions() {
        let weight = position_finishing_bias(pos.position());
        let prof_factor = (pos.proficiency() as f64) / 10.0;
        let weighted = weight * (0.5 + 0.5 * prof_factor);
        if weighted > best_weight {
            best_weight = weighted;
        }
    }
    best_weight
}

pub fn calculate_player_finishing_weight(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let base_weight = player_base_finishing_weight(player);
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

pub fn select_finisher<R: Rng + ?Sized>(
    candidates: &[&Player],
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attacking_positive_x: bool,
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
        .map(|p| calculate_player_finishing_weight(p, spatial_map, pitch, attacking_positive_x))
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}