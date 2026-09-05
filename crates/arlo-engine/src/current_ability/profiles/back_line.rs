use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::Position;

fn w(key: &str, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn artrine_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Artrine,
        vec![
            w("passing", 4.5),
            w("vision", 4.5),
            w("technique", 4.0),
            w("decisions", 4.0),
            w("spatial_awareness", 4.0),
            w("composure", 3.5),
            w("ball_control", 3.5),
            w("first_touch", 3.5),
            w("anticipation", 3.5),
            w("teamwork", 3.0),
            w("tactical_awareness", 3.0),
            w("long_shots", 2.5),
            w("balance", 2.5),
            w("stamina", 2.5),
            w("flair", 2.0),
            w("shooting", 2.0),
            w("work_rate", 2.0),
            w("concentration", 2.0),
        ],
    )
}

pub fn passer_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Passer,
        vec![
            w("passing", 5.0),
            w("vision", 4.5),
            w("decisions", 4.5),
            w("composure", 4.0),
            w("technique", 4.0),
            w("anticipation", 3.5),
            w("tactical_awareness", 3.5),
            w("concentration", 3.0),
            w("teamwork", 3.0),
            w("ball_control", 3.0),
            w("first_touch", 3.0),
            w("spatial_awareness", 3.0),
            w("strength", 2.5),
            w("balance", 2.5),
            w("stamina", 2.0),
            w("leadership", 2.0),
            w("positioning", 2.0),
            w("determination", 2.0),
        ],
    )
}

pub fn pass_rusher_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::PassRusher,
        vec![
            w("acceleration", 4.5),
            w("pace", 4.5),
            w("strength", 4.0),
            w("tackling", 4.0),
            w("aggression", 4.0),
            w("work_rate", 3.5),
            w("bravery", 3.5),
            w("agility", 3.5),
            w("anticipation", 3.0),
            w("stamina", 3.0),
            w("determination", 3.0),
            w("balance", 2.5),
            w("jumping", 2.5),
            w("tactical_awareness", 2.0),
            w("decisions", 2.0),
            w("positioning", 2.0),
            w("concentration", 1.5),
            w("composure", 1.5),
        ],
    )
}

pub fn wide_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WideEnd,
        vec![
            w("pace", 4.5),
            w("acceleration", 4.5),
            w("catching", 4.0),
            w("agility", 4.0),
            w("jumping", 3.5),
            w("balance", 3.5),
            w("anticipation", 3.0),
            w("stamina", 3.0),
            w("ball_control", 3.0),
            w("first_touch", 3.0),
            w("flair", 2.5),
            w("positioning", 2.5),
            w("composure", 2.5),
            w("technique", 2.5),
            w("crossing", 2.0),
            w("work_rate", 2.0),
            w("bravery", 2.0),
            w("determination", 1.5),
        ],
    )
}

pub fn running_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::RunningEnd,
        vec![
            w("acceleration", 4.5),
            w("pace", 4.5),
            w("balance", 4.0),
            w("agility", 4.0),
            w("strength", 3.5),
            w("stamina", 3.5),
            w("ball_control", 3.5),
            w("bravery", 3.0),
            w("work_rate", 3.0),
            w("anticipation", 3.0),
            w("first_touch", 2.5),
            w("composure", 2.5),
            w("decisions", 2.5),
            w("determination", 2.5),
            w("dribbling", 2.0),
            w("catching", 2.0),
            w("positioning", 1.5),
            w("flair", 1.5),
        ],
    )
}

pub fn lineback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Lineback,
        vec![
            w("tackling", 4.5),
            w("positioning", 4.0),
            w("strength", 4.0),
            w("anticipation", 4.0),
            w("decisions", 3.5),
            w("bravery", 3.5),
            w("stamina", 3.5),
            w("work_rate", 3.5),
            w("tactical_awareness", 3.0),
            w("concentration", 3.0),
            w("aggression", 3.0),
            w("balance", 2.5),
            w("jumping", 2.5),
            w("marking", 2.5),
            w("acceleration", 2.5),
            w("teamwork", 2.0),
            w("determination", 2.0),
            w("composure", 1.5),
        ],
    )
}

pub fn fullback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Fullback,
        vec![
            w("stamina", 4.5),
            w("pace", 4.0),
            w("acceleration", 4.0),
            w("tackling", 4.0),
            w("positioning", 3.5),
            w("marking", 3.5),
            w("work_rate", 3.5),
            w("crossing", 3.0),
            w("passing", 3.0),
            w("teamwork", 3.0),
            w("anticipation", 3.0),
            w("balance", 2.5),
            w("tactical_awareness", 2.5),
            w("decisions", 2.5),
            w("agility", 2.0),
            w("concentration", 2.0),
            w("bravery", 2.0),
            w("determination", 1.5),
        ],
    )
}