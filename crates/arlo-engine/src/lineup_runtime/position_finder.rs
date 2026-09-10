use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::position_similarity::position_similarity;
use arlo_domain::{Player, Position};

pub fn find_player_by_position<'a>(
    players: &[&'a Player],
    target: Position,
) -> EngineResult<&'a Player> {
    players
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == target && pos.proficiency() > 0)
        })
        .or_else(|| {
            players
                .iter()
                .copied()
                .find(|p| p.positions().iter().any(|pos| pos.position() == target))
        })
        .or_else(|| {
            players.iter().copied().max_by(|a, b| {
                let prof_a = a
                    .positions()
                    .iter()
                    .map(|pp| {
                        position_similarity(pp.position(), target) * (pp.proficiency() as f64)
                    })
                    .fold(0.0_f64, f64::max);
                let prof_b = b
                    .positions()
                    .iter()
                    .map(|pp| {
                        position_similarity(pp.position(), target) * (pp.proficiency() as f64)
                    })
                    .fold(0.0_f64, f64::max);
                prof_a
                    .partial_cmp(&prof_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        })
        .or_else(|| players.first().copied())
        .ok_or_else(|| EngineError::MissingRequiredPosition(format!("{target:?}")))
}

pub fn find_goalguard<'a>(defenders: &[&'a Player]) -> EngineResult<&'a Player> {
    find_player_by_position(defenders, Position::Goalguard)
}
