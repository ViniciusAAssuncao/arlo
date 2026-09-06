use crate::ai::epv::DynamicEpvModel;
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
        (x / total_len).max(0.0).min(1.0)
    } else {
        ((total_len - x) / total_len).max(0.0).min(1.0)
    }
}

pub fn drive_scarcity_term(drives_in_series: u32) -> f64 {
    if drives_in_series < 3 {
        ((3 - drives_in_series) as f64).exp()
    } else {
        0.0
    }
}

pub fn total_context_utility(
    decision: ArtrineDecisionKind,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    _pass_protection_net_advantage: f64,
    is_last_down: bool,
    territory_advance_mirim: f64,
    _best_available_target_weight: f64,
    _artrine_pos: Position,
    _next_artro_pos: Position,
    _spatial_resistance: f64,
) -> f64 {
    let down = if is_last_down {
        4
    } else {
        (4u8).saturating_sub(remaining_downs).max(1)
    };
    let rem_adv = (10.0 - territory_advance_mirim).max(0.0);
    let epv_model = DynamicEpvModel::default();
    let base_epv =
        epv_model.calculate_epa(normalized_proximity, down, rem_adv, drives_in_current_series);

    match decision {
        ArtrineDecisionKind::SelfCarry => {
            let next_epv = epv_model.calculate_epa(
                (normalized_proximity + 0.05).min(1.0),
                down,
                (rem_adv - 5.0).max(0.0),
                drives_in_current_series + 1,
            );
            (next_epv - base_epv) * 2.0
        }
        ArtrineDecisionKind::ShortPass => {
            let next_epv = epv_model.calculate_epa(
                (normalized_proximity + 0.06).min(1.0),
                down,
                (rem_adv - 6.0).max(0.0),
                drives_in_current_series,
            );
            (next_epv - base_epv) * 2.0
        }
        ArtrineDecisionKind::LongLaunch => {
            let next_epv = epv_model.calculate_epa(
                (normalized_proximity + 0.15).min(1.0),
                down,
                (rem_adv - 15.0).max(0.0),
                drives_in_current_series,
            );
            (next_epv - base_epv) * 2.0
        }
        ArtrineDecisionKind::Cross | ArtrineDecisionKind::SelfFinish => {
            let v_score = DynamicEpvModel::score_value(drives_in_current_series);
            normalized_proximity * v_score
        }
    }
}