use crate::ai::epv::DynamicEpvModel;
use crate::artrine::decision_profiles::get_artrine_decision_profile;
use crate::resolution::group_rating::calculate_player_duel_rating;
use arlo_domain::sport_constants::{
    FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkovDecisionEvaluator;

impl MarkovDecisionEvaluator {
    pub fn evaluate_action_utilities(
        artrine: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        available_kinds: &[ArtrineDecisionKind],
        normalized_proximity: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
        pass_protection_net_advantage: f64,
        best_available_target_weight: f64,
        pitch_control_ahead: f64,
        distance_to_next_artro_mirim: f64,
        pitch_length_mirim: f64,
        offensive_gravity: f64,
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        let epv_model = DynamicEpvModel::new(offensive_gravity);
        let current_epv = epv_model.calculate_epa(
            normalized_proximity,
            down,
            remaining_advance_mirim,
            drives_in_series,
        );
        let mut results = Vec::with_capacity(available_kinds.len());

        let target_quality = ((best_available_target_weight - 8.0) / 10.0).clamp(-0.5, 1.0);
        let pc = pitch_control_ahead.clamp(0.05, 0.95);

        for &kind in available_kinds {
            let profile = get_artrine_decision_profile(kind);
            let intrinsic_rating = calculate_player_duel_rating(
                artrine,
                Position::Artrine,
                attribute_keys,
                &profile,
            );
            let skill_mult = (intrinsic_rating / 10.0).clamp(0.5, 1.5);

            let expected_future_epv = match kind {
                ArtrineDecisionKind::SelfCarry => {
                    let adv_mirim = (4.0 + 4.0 * pc) * skill_mult;
                    let crosses_artro = adv_mirim >= distance_to_next_artro_mirim.max(0.5);
                    let new_drives = if crosses_artro {
                        drives_in_series + 1
                    } else {
                        drives_in_series
                    };
                    let (new_down, new_rem) = if adv_mirim >= remaining_advance_mirim {
                        (1, 10.0)
                    } else {
                        (
                            down.saturating_add(1).min(4),
                            (remaining_advance_mirim - adv_mirim).max(0.0),
                        )
                    };
                    let new_norm_x = (normalized_proximity
                        + adv_mirim / pitch_length_mirim.max(1.0))
                    .min(1.0);
                    let epv_success =
                        epv_model.calculate_epa(new_norm_x, new_down, new_rem, new_drives);
                    let epv_fail = if down >= 4 && remaining_advance_mirim > 0.0 {
                        -epv_model.opponent_epa(normalized_proximity)
                    } else {
                        epv_model.calculate_epa(
                            normalized_proximity,
                            down.saturating_add(1).min(4),
                            remaining_advance_mirim,
                            drives_in_series,
                        )
                    };
                    let p_succ = (pc * 0.75 + 0.15 * skill_mult).clamp(0.10, 0.95);
                    let p_to = ((1.0 - pc) * 0.15).clamp(0.02, 0.30);
                    let p_fail = (1.0 - p_succ - p_to).max(0.0);
                    p_succ * epv_success + p_fail * epv_fail
                        - p_to * epv_model.opponent_epa(normalized_proximity)
                }
                ArtrineDecisionKind::ShortPass => {
                    let adv_mirim = (6.0 + 3.0 * target_quality) * skill_mult;
                    let (new_down, new_rem) = if adv_mirim >= remaining_advance_mirim {
                        (1, 10.0)
                    } else {
                        (
                            down.saturating_add(1).min(4),
                            (remaining_advance_mirim - adv_mirim).max(0.0),
                        )
                    };
                    let new_norm_x = (normalized_proximity
                        + adv_mirim / pitch_length_mirim.max(1.0))
                    .min(1.0);
                    let epv_success =
                        epv_model.calculate_epa(new_norm_x, new_down, new_rem, drives_in_series);
                    let epv_fail = if down >= 4 && remaining_advance_mirim > 0.0 {
                        -epv_model.opponent_epa(normalized_proximity)
                    } else {
                        epv_model.calculate_epa(
                            normalized_proximity,
                            down.saturating_add(1).min(4),
                            remaining_advance_mirim,
                            drives_in_series,
                        )
                    };
                    let p_succ = (0.50
                        + 0.04 * pass_protection_net_advantage
                        + 0.20 * target_quality
                        + 0.10 * skill_mult)
                        .clamp(0.15, 0.95);
                    let p_to = ((1.0 - p_succ) * 0.20).clamp(0.02, 0.35);
                    let p_fail = (1.0 - p_succ - p_to).max(0.0);
                    p_succ * epv_success + p_fail * epv_fail
                        - p_to * epv_model.opponent_epa(normalized_proximity)
                }
                ArtrineDecisionKind::LongLaunch => {
                    let adv_mirim = (14.0 + 6.0 * target_quality) * skill_mult;
                    let (new_down, new_rem) = if adv_mirim >= remaining_advance_mirim {
                        (1, 10.0)
                    } else {
                        (
                            down.saturating_add(1).min(4),
                            (remaining_advance_mirim - adv_mirim).max(0.0),
                        )
                    };
                    let new_norm_x = (normalized_proximity
                        + adv_mirim / pitch_length_mirim.max(1.0))
                    .min(1.0);
                    let epv_success =
                        epv_model.calculate_epa(new_norm_x, new_down, new_rem, drives_in_series);
                    let epv_fail = if down >= 4 && remaining_advance_mirim > 0.0 {
                        -epv_model.opponent_epa(normalized_proximity)
                    } else {
                        epv_model.calculate_epa(
                            normalized_proximity,
                            down.saturating_add(1).min(4),
                            remaining_advance_mirim,
                            drives_in_series,
                        )
                    };
                    let p_succ = (0.35
                        + 0.03 * pass_protection_net_advantage
                        + 0.25 * target_quality
                        + 0.10 * skill_mult)
                        .clamp(0.10, 0.85);
                    let p_to = ((1.0 - p_succ) * 0.35).clamp(0.05, 0.50);
                    let p_fail = (1.0 - p_succ - p_to).max(0.0);
                    p_succ * epv_success + p_fail * epv_fail
                        - p_to * epv_model.opponent_epa(normalized_proximity)
                }
                ArtrineDecisionKind::Cross => {
                    let value = if drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
                        GOAL_POINT_VALUE as f64
                    } else if drives_in_series >= 1 {
                        FIELD_POINT_VALUE as f64
                    } else {
                        1.5
                    };
                    let p_goal = (0.30
                        + 0.40 * normalized_proximity
                        + 0.15 * target_quality
                        + 0.10 * skill_mult)
                        .clamp(0.10, 0.90)
                        * (0.6 + 0.4 * offensive_gravity.min(2.0));
                    let v_opp = epv_model.opponent_epa(normalized_proximity);
                    p_goal * value - (1.0 - p_goal) * v_opp
                }
                ArtrineDecisionKind::SelfFinish => {
                    let value = if drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
                        GOAL_POINT_VALUE as f64
                    } else if drives_in_series >= 1 {
                        FIELD_POINT_VALUE as f64
                    } else {
                        1.5
                    };
                    let p_goal = (0.25 + 0.50 * normalized_proximity + 0.15 * skill_mult)
                        .clamp(0.05, 0.90);
                    let v_opp = epv_model.opponent_epa(normalized_proximity);
                    p_goal * value - (1.0 - p_goal) * v_opp
                }
            };

            let delta_epv = expected_future_epv - current_epv;
            let total_utility = delta_epv * 3.5 + (intrinsic_rating * 0.2);
            results.push((kind, total_utility));
        }

        results
    }
}