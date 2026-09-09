use crate::instructions::axes::PassingRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PassRangeTier {
    VeryShort,
    Short,
    Mixed,
    Long,
    VeryLong,
}

pub fn derive_pass_range_tier(passing_range: PassingRange) -> PassRangeTier {
    let value = passing_range.value().clamp(-1.0, 1.0);
    let normalized = (value + 1.0) / 2.0;
    let index = (normalized * 5.0).floor() as usize;

    match index {
        0 => PassRangeTier::VeryShort,
        1 => PassRangeTier::Short,
        2 => PassRangeTier::Mixed,
        3 => PassRangeTier::Long,
        _ => PassRangeTier::VeryLong,
    }
}
