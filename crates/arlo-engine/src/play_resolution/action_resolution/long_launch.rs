use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::psychology::state::ImpulseState;
use crate::team_identity::passing_style::long_launch_advance_multiplier;
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::AttributeKey;
use arlo_math::Probability;
use arlo_tactics::PassingRange;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongLaunchActionResult {
    pub completed: bool,
    pub is_aerial: bool,
    pub advance_mirim: f64,
    pub turnover: bool,
    pub interception: bool,
    pub target_player_id: Option<Uuid>,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
}

pub fn resolve_long_launch_action<R: Rng + ?Sized>(
    passer_table: &PlayerAttributeTable,
    passer_fatigue: &PhysicalState,
    passer_impulse: &ImpulseState,
    receiver_id: Uuid,
    receiver_table: &PlayerAttributeTable,
    space_rating: &TeamSpaceRating,
    pitch_state: &PitchState,
    passing_range: PassingRange,
    pitch_length_mirim: f64,
    rng: &mut R,
) -> LongLaunchActionResult {
    let passing = passer_table.get(AttributeKey::Passing) / ATTRIBUTE_MAX;
    let vision = passer_table.get(AttributeKey::Vision) / ATTRIBUTE_MAX;
    let flair = passer_table.get(AttributeKey::Flair) / ATTRIBUTE_MAX;
    let technique = passer_table.get(AttributeKey::Technique) / ATTRIBUTE_MAX;

    let passer_fatigue_penalty = (1.0 - passer_fatigue.energy()) * 0.25;
    let passer_impulse_mod = ((passer_impulse.accumulator() - 50.0) / 50.0) * 0.12;
    let passer_skill = (passing * 0.35 + vision * 0.35 + flair * 0.15 + technique * 0.15
        + passer_impulse_mod
        - passer_fatigue_penalty)
        .clamp(0.1, 1.5);

    let jumping = receiver_table.get(AttributeKey::JumpingReach) / ATTRIBUTE_MAX;
    let hands = receiver_table.get(AttributeKey::HandsReception) / ATTRIBUTE_MAX;
    let pace = receiver_table.get(AttributeKey::Pace) / ATTRIBUTE_MAX;
    let ant = receiver_table.get(AttributeKey::Anticipation) / ATTRIBUTE_MAX;
    let receiver_skill =
        (jumping * 0.35 + hands * 0.30 + pace * 0.20 + ant * 0.15).clamp(0.1, 1.5);

    let attack_strength = passer_skill * 0.50 + receiver_skill * 0.50;
    let defense_strength = (space_rating.pressure_intensity() * 1.15).max(0.1);
    let total_strength = attack_strength + defense_strength;
    let base_prob = attack_strength / total_strength;
    let space_modifier = (space_rating.space_index() - 0.5) * 0.12;
    let win_prob_val = (base_prob + space_modifier).clamp(0.08, 0.92);
    let win_probability = Probability::new_clamped(win_prob_val);
    let completed = win_probability.sample(rng);

    let net_advantage = (attack_strength - defense_strength) * 4.0;

    let pass_mult = long_launch_advance_multiplier(passing_range);
    let (turnover, interception, actual_advance) = if completed {
        let advance = (16.0 * pass_mult + 5.0 * receiver_skill + 4.0 * passer_skill).clamp(15.0, 35.0);
        (false, false, advance)
    } else {
        let int_p =
            (0.18 * space_rating.pressure_intensity() * (1.0 - vision)).clamp(0.05, 0.35);
        let is_int = Probability::new_clamped(int_p).sample(rng);
        (is_int, is_int, 0.0)
    };

    let next_pitch_state = pitch_state.with_advance(actual_advance, pitch_length_mirim);

    LongLaunchActionResult {
        completed,
        is_aerial: true,
        advance_mirim: actual_advance,
        turnover,
        interception,
        target_player_id: Some(receiver_id),
        net_advantage,
        win_probability,
        next_pitch_state,
    }
}
