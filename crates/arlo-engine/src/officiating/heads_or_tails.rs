use arlo_math::Probability;
use rand::Rng;

pub fn flip_officiating_coin<R: Rng + ?Sized>(rng: &mut R) -> bool {
    Probability::new_clamped(0.5).sample(rng)
}
