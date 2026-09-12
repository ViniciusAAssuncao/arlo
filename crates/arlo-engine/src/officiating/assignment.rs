use crate::error::{EngineError, EngineResult};
use crate::rng::{MatchSeed, RngProvider, RngStream};
use arlo_domain::Referee;
use rand::Rng;

pub fn draw_match_referees(
    mut candidates: Vec<Referee>,
    seed: MatchSeed,
) -> EngineResult<(Referee, Referee)> {
    if candidates.len() < 2 {
        return Err(EngineError::InsufficientRefereeCandidates);
    }

    let provider = RngProvider::new(seed);
    let mut rng = provider.rng_for(RngStream::RefereeAssignment);

    let head_idx = rng.gen_range(0..candidates.len());
    let head_referee = candidates.remove(head_idx);

    let peace_idx = rng.gen_range(0..candidates.len());
    let peace_referee = candidates.remove(peace_idx);

    Ok((head_referee, peace_referee))
}