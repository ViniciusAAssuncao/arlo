use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::psychology::state::ImpulseState;
use crate::team_identity::passing_style::short_pass_advance_multiplier;
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::AttributeKey;
use arlo_math::stats::contrast::logistic;
use arlo_math::stats::noise::sample_gaussian_noise;
use arlo_math::Probability;
use arlo_tactics::PassingRange;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortPassActionResult {
    pub completed: bool,
    pub advance_mirim: f64,
    pub turnover: bool,
    pub interception: bool,
    pub target_player_id: Option<Uuid>,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
}

pub fn resolve_short_pass_action<R: Rng + ?Sized>(
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
) -> ShortPassActionResult {
    let passing = passer_table.get(AttributeKey::Passing) / ATTRIBUTE_MAX;
    let decisions = passer_table.get(AttributeKey::Decisions) / ATTRIBUTE_MAX;
    let composure = passer_table.get(AttributeKey::Composure) / ATTRIBUTE_MAX;
    let technique = passer_table.get(AttributeKey::Technique) / ATTRIBUTE_MAX;

    let passer_fatigue_penalty = (1.0 - passer_fatigue.energy()) * 0.20;
    let passer_impulse_mod = ((passer_impulse.accumulator() - 50.0) / 50.0) * 0.10;
    let passer_skill = (passing * 0.40 + decisions * 0.30 + composure * 0.15 + technique * 0.15
        + passer_impulse_mod
        - passer_fatigue_penalty)
        .clamp(0.1, 1.5);

    let hands = receiver_table.get(AttributeKey::HandsReception) / ATTRIBUTE_MAX;
    let ant = receiver_table.get(AttributeKey::Anticipation) / ATTRIBUTE_MAX;
    let agility = receiver_table.get(AttributeKey::Agility) / ATTRIBUTE_MAX;
    let receiver_skill = (hands * 0.45 + ant * 0.35 + agility * 0.20).clamp(0.1, 1.5);

    let pass_mult = short_pass_advance_multiplier(passing_range);
    let base_adv =
        (space_rating.expected_free_mirim() * 0.85 * pass_mult + 2.0 * receiver_skill) * passer_skill;
    let noise = sample_gaussian_noise(0.5, rng);
    let potential_advance = (base_adv + noise).clamp(1.0, 15.0);

    let completion_logit = (passer_skill - 0.5) * 3.5
        + (receiver_skill - 0.5) * 2.5
        + (space_rating.space_index() - 0.5) * 4.0;
    let win_prob_val = logistic(completion_logit).clamp(0.10, 0.98);
    let win_probability = Probability::new_clamped(win_prob_val);
    let completed = win_probability.sample(rng);

    let net_advantage =
        (passer_skill + receiver_skill - 2.0 * space_rating.pressure_intensity()) * 3.5;

    let (turnover, interception, actual_advance) = if completed {
        (false, false, potential_advance)
    } else {
        let int_p =
            (0.12 * space_rating.pressure_intensity() * (1.0 - decisions)).clamp(0.02, 0.30);
        let is_int = Probability::new_clamped(int_p).sample(rng);
        (is_int, is_int, 0.0)
    };

    let next_pitch_state = pitch_state.with_advance(actual_advance, pitch_length_mirim);

    ShortPassActionResult {
        completed,
        advance_mirim: actual_advance,
        turnover,
        interception,
        target_player_id: Some(receiver_id),
        net_advantage,
        win_probability,
        next_pitch_state,
    }
}
