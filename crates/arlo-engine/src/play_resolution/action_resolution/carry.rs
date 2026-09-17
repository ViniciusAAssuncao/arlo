use crate::ai::cognitive::RiskProfile;
use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::psychology::state::ImpulseState;
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::AttributeKey;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarryActionResult {
    pub mirins_advanced: f64,
    pub drives_crossed: u32,
    pub success: bool,
    pub turnover: bool,
    pub contact_severity: f64,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
}

pub fn resolve_carry_action<R: Rng + ?Sized>(
    carrier_table: &PlayerAttributeTable,
    carrier_fatigue: &PhysicalState,
    carrier_impulse: &ImpulseState,
    risk_profile: &RiskProfile,
    space_rating: &TeamSpaceRating,
    pitch_state: &PitchState,
    pitch_length_mirim: f64,
    is_true_artrine: bool,
    rng: &mut R,
) -> CarryActionResult {
    let drive_tech = carrier_table.get(AttributeKey::DriveTechnique) / ATTRIBUTE_MAX;
    let arlo_ctrl = carrier_table.get(AttributeKey::ArloControl) / ATTRIBUTE_MAX;
    let accel = carrier_table.get(AttributeKey::Acceleration) / ATTRIBUTE_MAX;
    let balance = carrier_table.get(AttributeKey::Balance) / ATTRIBUTE_MAX;
    let bravery = carrier_table.get(AttributeKey::Bravery) / ATTRIBUTE_MAX;

    let impulse_boost = ((carrier_impulse.accumulator() - 50.0) / 50.0) * 0.15;
    let fatigue_penalty = (1.0 - carrier_fatigue.energy()) * 0.25;

    let carrier_skill = (drive_tech * 0.35
        + arlo_ctrl * 0.25
        + accel * 0.20
        + balance * 0.10
        + bravery * 0.10
        + impulse_boost
        - fatigue_penalty)
        .clamp(0.1, 1.5);

    let attack_strength = carrier_skill;
    let defense_strength = space_rating.pressure_intensity().max(0.1);
    let total_strength = attack_strength + defense_strength;
    let base_prob = attack_strength / total_strength;
    let space_modifier = (space_rating.space_index() - 0.5) * 0.10;
    let win_prob_val = (base_prob + space_modifier).clamp(0.05, 0.95);
    let win_probability = Probability::new_clamped(win_prob_val);
    let success = win_probability.sample(rng);

    let net_advantage = (attack_strength - defense_strength) * 5.0;

    let actual_advance = if success {
        12.0 + carrier_skill * 3.0 + space_rating.space_index() * 3.0
    } else {
        (space_rating.expected_free_mirim() * 0.25).clamp(0.0, 3.0)
    };

    let drives_crossed = if is_true_artrine && actual_advance >= 3.0 {
        (actual_advance / 3.0).floor() as u32
    } else {
        0
    };

    let contact_severity = if success {
        (space_rating.pressure_intensity() * (1.0 - balance) * 0.30).clamp(0.0, 0.50)
    } else {
        (space_rating.pressure_intensity() * 0.60 + 0.20).clamp(0.10, 1.00)
    };

    let turnover_prob_val = if success {
        (0.02 * space_rating.pressure_intensity() / risk_profile.tolerance_index())
            .clamp(0.005, 0.12)
    } else {
        (0.18 * space_rating.pressure_intensity() / risk_profile.tolerance_index())
            .clamp(0.05, 0.45)
    };
    let turnover = Probability::new_clamped(turnover_prob_val).sample(rng);

    let next_pitch_state = pitch_state
        .with_advance(actual_advance, pitch_length_mirim)
        .with_drive_increment(drives_crossed);

    CarryActionResult {
        mirins_advanced: actual_advance,
        drives_crossed,
        success,
        turnover,
        contact_severity,
        net_advantage,
        win_probability,
        next_pitch_state,
    }
}
