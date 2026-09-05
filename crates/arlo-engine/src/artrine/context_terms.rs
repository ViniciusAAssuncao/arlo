use arlo_domain::sport_constants::{
    DOWN_PRESSURE_UTILITY_SCALE, DRIVES_NEEDED_UTILITY_SCALE,
    FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST, FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM,
    FIELD_POINT_REQUIRED_DRIVES, GOAL_POINT_REQUIRED_DRIVES,
    LAST_DOWN_DESPERATION_UTILITY_SCALE, MAX_CALL_TO_ACTIONS_PER_SERIES,
    PRESSURE_READ_UTILITY_SCALE, RANGE_UTILITY_SCALE,
};
use arlo_domain::{ArtrineDecisionKind, Pitch};
use arlo_math::units::Position;

pub fn calculate_normalized_proximity(
    artrine_pos: Position,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let total_len = pitch.length().value();
    if total_len <= 0.0 {
        return 0.0;
    }
    let x = artrine_pos.raw().0;
    if attacking_positive_x {
        (x / total_len).clamp(0.0, 1.0)
    } else {
        ((total_len - x) / total_len).clamp(0.0, 1.0)
    }
}

pub fn range_term(decision: ArtrineDecisionKind, normalized_proximity: f64) -> f64 {
    let prox = normalized_proximity.clamp(0.0, 1.0);
    match decision {
        ArtrineDecisionKind::SelfFinish => RANGE_UTILITY_SCALE * prox,
        ArtrineDecisionKind::Cross => RANGE_UTILITY_SCALE * prox * 0.8,
        ArtrineDecisionKind::SelfCarry
        | ArtrineDecisionKind::ShortPass
        | ArtrineDecisionKind::LongLaunch => 0.0,
    }
}

pub fn drives_needed_term(decision: ArtrineDecisionKind, drives_in_current_series: u32) -> f64 {
    if drives_in_current_series < GOAL_POINT_REQUIRED_DRIVES {
        let needed_ratio = (GOAL_POINT_REQUIRED_DRIVES - drives_in_current_series) as f64
            / (GOAL_POINT_REQUIRED_DRIVES as f64);
        match decision {
            ArtrineDecisionKind::SelfCarry => DRIVES_NEEDED_UTILITY_SCALE * needed_ratio,
            _ => 0.0,
        }
    } else {
        0.0
    }
}

pub fn down_pressure_term(decision: ArtrineDecisionKind, remaining_downs: u8) -> f64 {
    let max_downs = MAX_CALL_TO_ACTIONS_PER_SERIES as f64;
    let urgency =
        (max_downs - 1.0 - (remaining_downs as f64).min(max_downs - 1.0)) / (max_downs - 1.0);
    let urgency = urgency.clamp(0.0, 1.0);

    match decision {
        ArtrineDecisionKind::SelfCarry => DOWN_PRESSURE_UTILITY_SCALE * urgency * 0.5,
        ArtrineDecisionKind::LongLaunch => DOWN_PRESSURE_UTILITY_SCALE * urgency,
        ArtrineDecisionKind::ShortPass => -DOWN_PRESSURE_UTILITY_SCALE * urgency,
        ArtrineDecisionKind::Cross => DOWN_PRESSURE_UTILITY_SCALE * urgency * 0.5,
        ArtrineDecisionKind::SelfFinish => 0.0,
    }
}

pub fn pressure_read_term(
    decision: ArtrineDecisionKind,
    pass_protection_net_advantage: f64,
) -> f64 {
    let pressure_factor = (-pass_protection_net_advantage / 5.0).clamp(-2.0, 2.0);
    match decision {
        ArtrineDecisionKind::ShortPass => PRESSURE_READ_UTILITY_SCALE * pressure_factor,
        ArtrineDecisionKind::SelfCarry => -PRESSURE_READ_UTILITY_SCALE * pressure_factor,
        ArtrineDecisionKind::LongLaunch => -PRESSURE_READ_UTILITY_SCALE * pressure_factor,
        ArtrineDecisionKind::Cross => -PRESSURE_READ_UTILITY_SCALE * pressure_factor * 0.5,
        ArtrineDecisionKind::SelfFinish => 0.0,
    }
}

pub fn last_down_desperation_term(
    decision: ArtrineDecisionKind,
    is_last_down: bool,
    territory_advance_mirim: f64,
    drives_in_series: u32,
) -> f64 {
    if !is_last_down {
        return 0.0;
    }

    let goal_point_available = drives_in_series >= GOAL_POINT_REQUIRED_DRIVES;
    let field_point_available = drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
        && territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM;
    let field_goal_reachable =
        territory_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST;

    if !goal_point_available && !field_point_available && field_goal_reachable {
        match decision {
            ArtrineDecisionKind::SelfFinish => LAST_DOWN_DESPERATION_UTILITY_SCALE,
            ArtrineDecisionKind::Cross => LAST_DOWN_DESPERATION_UTILITY_SCALE * 0.8,
            _ => 0.0,
        }
    } else {
        0.0
    }
}

pub fn total_context_utility(
    decision: ArtrineDecisionKind,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
    territory_advance_mirim: f64,
) -> f64 {
    range_term(decision, normalized_proximity)
        + drives_needed_term(decision, drives_in_current_series)
        + down_pressure_term(decision, remaining_downs)
        + pressure_read_term(decision, pass_protection_net_advantage)
        + last_down_desperation_term(
            decision,
            is_last_down,
            territory_advance_mirim,
            drives_in_current_series,
        )
}