use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::Position;

fn w(key: &str, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn center_offense_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::CenterOffense,
        vec![
            w("finishing", 4.5),
            w("shooting", 4.0),
            w("composure", 4.0),
            w("anticipation", 3.5),
            w("first_touch", 3.5),
            w("ball_control", 3.5),
            w("heading", 3.0),
            w("strength", 3.0),
            w("balance", 3.0),
            w("acceleration", 3.0),
            w("decisions", 2.5),
            w("positioning", 2.5),
            w("technique", 2.5),
            w("pace", 2.5),
            w("jumping", 2.0),
            w("stamina", 2.0),
            w("work_rate", 2.0),
            w("determination", 2.0),
        ],
    )
}

pub fn wing_offense_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WingOffense,
        vec![
            w("pace", 4.5),
            w("acceleration", 4.5),
            w("dribbling", 4.0),
            w("crossing", 4.0),
            w("agility", 4.0),
            w("ball_control", 3.5),
            w("first_touch", 3.5),
            w("technique", 3.5),
            w("flair", 3.0),
            w("finishing", 3.0),
            w("stamina", 3.0),
            w("composure", 2.5),
            w("passing", 2.5),
            w("vision", 2.5),
            w("anticipation", 2.5),
            w("balance", 2.5),
            w("work_rate", 2.0),
            w("decisions", 2.0),
        ],
    )
}

pub fn midcenter_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Midcenter,
        vec![
            w("passing", 4.5),
            w("vision", 4.5),
            w("technique", 4.0),
            w("ball_control", 4.0),
            w("first_touch", 4.0),
            w("decisions", 4.0),
            w("composure", 3.5),
            w("teamwork", 3.5),
            w("tactical_awareness", 3.5),
            w("anticipation", 3.5),
            w("stamina", 3.0),
            w("positioning", 3.0),
            w("work_rate", 3.0),
            w("balance", 2.5),
            w("agility", 2.5),
            w("shooting", 2.5),
            w("long_shots", 2.5),
            w("tackling", 2.0),
        ],
    )
}

pub fn tight_wing_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::TightWing,
        vec![
            w("dribbling", 4.5),
            w("agility", 4.0),
            w("acceleration", 4.0),
            w("ball_control", 4.0),
            w("crossing", 3.5),
            w("first_touch", 3.5),
            w("technique", 3.5),
            w("pace", 3.5),
            w("balance", 3.0),
            w("passing", 3.0),
            w("stamina", 3.0),
            w("work_rate", 3.0),
            w("anticipation", 2.5),
            w("composure", 2.5),
            w("decisions", 2.5),
            w("tactical_awareness", 2.0),
            w("tackling", 2.0),
            w("finishing", 2.0),
        ],
    )
}

pub fn center_tight_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::CenterTight,
        vec![
            w("ball_control", 4.0),
            w("passing", 4.0),
            w("strength", 4.0),
            w("balance", 3.5),
            w("tactical_awareness", 3.5),
            w("decisions", 3.5),
            w("composure", 3.5),
            w("first_touch", 3.5),
            w("technique", 3.5),
            w("positioning", 3.0),
            w("teamwork", 3.0),
            w("work_rate", 3.0),
            w("stamina", 3.0),
            w("anticipation", 2.5),
            w("tackling", 2.5),
            w("heading", 2.0),
            w("shooting", 2.0),
            w("vision", 2.0),
        ],
    )
}

pub fn corridor_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Corridor,
        vec![
            w("stamina", 4.5),
            w("work_rate", 4.5),
            w("pace", 4.0),
            w("acceleration", 4.0),
            w("teamwork", 3.5),
            w("crossing", 3.5),
            w("passing", 3.5),
            w("positioning", 3.0),
            w("tackling", 3.0),
            w("anticipation", 3.0),
            w("tactical_awareness", 3.0),
            w("ball_control", 2.5),
            w("first_touch", 2.5),
            w("balance", 2.5),
            w("agility", 2.5),
            w("decisions", 2.5),
            w("determination", 2.0),
            w("marking", 2.0),
        ],
    )
}