use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::Position;

fn w(key: &str, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn goalguard_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Goalguard,
        vec![
            w("reflexes", 5.0),
            w("shot_stopping", 5.0),
            w("handling", 4.5),
            w("aerial_reach", 4.5),
            w("positioning", 4.5),
            w("one_on_ones", 4.0),
            w("command_of_area", 4.0),
            w("rushing_out", 3.5),
            w("communication", 3.5),
            w("concentration", 3.5),
            w("agility", 3.5),
            w("jumping", 3.5),
            w("composure", 3.0),
            w("anticipation", 3.0),
            w("bravery", 3.0),
            w("distribution", 2.5),
            w("kicking", 2.5),
            w("throwing", 2.5),
            w("punching", 2.0),
            w("decisions", 2.0),
        ],
    )
}