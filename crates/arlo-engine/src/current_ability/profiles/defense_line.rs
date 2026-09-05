use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::Position;

fn w(key: &str, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn centerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Centerback,
        vec![
            w("tackling", 4.5),
            w("marking", 4.5),
            w("positioning", 4.5),
            w("strength", 4.0),
            w("heading", 4.0),
            w("jumping", 3.5),
            w("anticipation", 3.5),
            w("bravery", 3.5),
            w("composure", 3.0),
            w("decisions", 3.0),
            w("concentration", 3.0),
            w("tactical_awareness", 3.0),
            w("balance", 2.5),
            w("aggression", 2.5),
            w("pace", 2.0),
            w("acceleration", 2.0),
            w("stamina", 2.0),
            w("teamwork", 2.0),
        ],
    )
}

pub fn defensive_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::DefensiveEnd,
        vec![
            w("tackling", 4.5),
            w("strength", 4.5),
            w("acceleration", 4.0),
            w("pace", 4.0),
            w("aggression", 4.0),
            w("bravery", 3.5),
            w("work_rate", 3.5),
            w("stamina", 3.5),
            w("anticipation", 3.0),
            w("agility", 3.0),
            w("positioning", 2.5),
            w("balance", 2.5),
            w("jumping", 2.5),
            w("decisions", 2.0),
            w("determination", 2.0),
            w("marking", 2.0),
            w("concentration", 2.0),
            w("tactical_awareness", 1.5),
        ],
    )
}

pub fn rougieback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Rougieback,
        vec![
            w("tackling", 4.5),
            w("positioning", 4.0),
            w("pace", 4.0),
            w("acceleration", 4.0),
            w("agility", 3.5),
            w("anticipation", 3.5),
            w("marking", 3.5),
            w("stamina", 3.5),
            w("bravery", 3.0),
            w("concentration", 3.0),
            w("work_rate", 3.0),
            w("balance", 2.5),
            w("decisions", 2.5),
            w("tactical_awareness", 2.5),
            w("strength", 2.0),
            w("teamwork", 2.0),
            w("interceptions", 2.0),
            w("determination", 1.5),
        ],
    )
}

pub fn defensive_blocker_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::DefensiveBlocker,
        vec![
            w("strength", 5.0),
            w("blocking", 4.5),
            w("tackling", 4.0),
            w("positioning", 4.0),
            w("bravery", 4.0),
            w("balance", 3.5),
            w("jumping", 3.5),
            w("concentration", 3.0),
            w("composure", 3.0),
            w("decisions", 3.0),
            w("anticipation", 2.5),
            w("tactical_awareness", 2.5),
            w("aggression", 2.5),
            w("stamina", 2.0),
            w("work_rate", 2.0),
            w("marking", 2.0),
            w("teamwork", 1.5),
            w("determination", 1.5),
        ],
    )
}

pub fn wide_blocker_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WideBlocker,
        vec![
            w("pace", 4.5),
            w("acceleration", 4.0),
            w("tackling", 4.0),
            w("blocking", 4.0),
            w("agility", 3.5),
            w("stamina", 3.5),
            w("marking", 3.5),
            w("positioning", 3.5),
            w("work_rate", 3.0),
            w("balance", 3.0),
            w("anticipation", 3.0),
            w("bravery", 2.5),
            w("strength", 2.5),
            w("decisions", 2.0),
            w("concentration", 2.0),
            w("tactical_awareness", 2.0),
            w("teamwork", 1.5),
            w("determination", 1.5),
        ],
    )
}

pub fn outside_zonerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::OutsideZonerback,
        vec![
            w("positioning", 4.5),
            w("tactical_awareness", 4.5),
            w("pace", 4.0),
            w("acceleration", 4.0),
            w("anticipation", 4.0),
            w("interceptions", 3.5),
            w("marking", 3.5),
            w("agility", 3.0),
            w("stamina", 3.0),
            w("decisions", 3.0),
            w("concentration", 2.5),
            w("composure", 2.5),
            w("tackling", 2.5),
            w("balance", 2.0),
            w("work_rate", 2.0),
            w("teamwork", 2.0),
            w("spatial_awareness", 2.0),
            w("determination", 1.5),
        ],
    )
}

pub fn middle_zonerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::MiddleZonerback,
        vec![
            w("positioning", 4.5),
            w("tactical_awareness", 4.5),
            w("anticipation", 4.5),
            w("interceptions", 4.0),
            w("tackling", 3.5),
            w("decisions", 3.5),
            w("composure", 3.5),
            w("teamwork", 3.0),
            w("strength", 3.0),
            w("stamina", 3.0),
            w("concentration", 3.0),
            w("marking", 2.5),
            w("spatial_awareness", 2.5),
            w("balance", 2.0),
            w("bravery", 2.0),
            w("acceleration", 2.0),
            w("pace", 2.0),
            w("leadership", 1.5),
        ],
    )
}