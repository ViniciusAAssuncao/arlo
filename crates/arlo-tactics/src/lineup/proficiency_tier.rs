use arlo_domain::sport_constants::{MAX_POSITION_PROFICIENCY, MIN_POSITION_PROFICIENCY};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProficiencyTier {
    Unfamiliar,
    Makeshift,
    Competent,
    Accomplished,
    Natural,
}

pub fn derive_tier(proficiency: i32) -> ProficiencyTier {
    let clamped = proficiency.clamp(MIN_POSITION_PROFICIENCY, MAX_POSITION_PROFICIENCY);
    let span = (MAX_POSITION_PROFICIENCY - MIN_POSITION_PROFICIENCY) as f64;
    let offset = (clamped - MIN_POSITION_PROFICIENCY) as f64;
    let fraction = offset / span;
    let index = (fraction * 5.0).floor() as usize;

    match index {
        0 => ProficiencyTier::Unfamiliar,
        1 => ProficiencyTier::Makeshift,
        2 => ProficiencyTier::Competent,
        3 => ProficiencyTier::Accomplished,
        _ => ProficiencyTier::Natural,
    }
}
