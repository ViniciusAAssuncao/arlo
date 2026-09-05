use crate::resolution::outcome::DuelOutcome;
use rand::Rng;

pub trait ProgressionResolutionStrategy {
    fn resolve_progression<R: Rng + ?Sized>(
        &self,
        duel_outcome: &DuelOutcome,
        rng: &mut R,
    ) -> f64;
}