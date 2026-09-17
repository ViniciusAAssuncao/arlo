use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::{ArtroPlacement, AttributeKey, PitchZone};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossActionResult {
    pub completed: bool,
    pub target_player_id: Option<Uuid>,
    pub target_zone: PitchZone,
    pub turnover: bool,
    pub scoring_attempt_ready: bool,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
}

pub fn resolve_cross_action<R: Rng + ?Sized>(
    crosser_table: &PlayerAttributeTable,
    crosser_fatigue: &PhysicalState,
    target_finisher_id: Uuid,
    target_finisher_table: &PlayerAttributeTable,
    space_rating: &TeamSpaceRating,
    pitch_state: &PitchState,
    pitch_length_mirim: f64,
    rng: &mut R,
) -> CrossActionResult {
    let crossing = crosser_table.get(AttributeKey::Crossing) / ATTRIBUTE_MAX;
    let vision = crosser_table.get(AttributeKey::Vision) / ATTRIBUTE_MAX;
    let flair = crosser_table.get(AttributeKey::Flair) / ATTRIBUTE_MAX;
    let tech = crosser_table.get(AttributeKey::Technique) / ATTRIBUTE_MAX;
    let fatigue_penalty = (1.0 - crosser_fatigue.energy()) * 0.20;

    let crosser_skill =
        (crossing * 0.45 + vision * 0.25 + flair * 0.15 + tech * 0.15 - fatigue_penalty)
            .clamp(0.1, 1.5);

    let finishing = target_finisher_table.get(AttributeKey::Finishing) / ATTRIBUTE_MAX;
    let ant = target_finisher_table.get(AttributeKey::Anticipation) / ATTRIBUTE_MAX;
    let jumping = target_finisher_table.get(AttributeKey::JumpingReach) / ATTRIBUTE_MAX;
    let receiver_skill = (finishing * 0.40 + ant * 0.35 + jumping * 0.25).clamp(0.1, 1.5);

    let channel_bonus = match pitch_state.channel() {
        ArtroPlacement::LeftLateral | ArtroPlacement::RightLateral => 0.35,
        ArtroPlacement::Central => -0.15,
    };

    let cross_logit = (crosser_skill - 0.5) * 3.5
        + (receiver_skill - 0.5) * 2.5
        + (space_rating.flank_openness() - 0.5) * 3.5
        + channel_bonus;
    let win_prob_val = logistic(cross_logit).clamp(0.10, 0.96);
    let win_probability = Probability::new_clamped(win_prob_val);
    let completed = win_probability.sample(rng);

    let net_advantage =
        (crosser_skill + receiver_skill - 2.0 * space_rating.pressure_intensity()) * 3.5;

    let target_zone = PitchZone::FirstZone;
    let (turnover, scoring_attempt_ready) = if completed {
        (false, true)
    } else {
        let to_p = (0.25 * space_rating.pressure_intensity()).clamp(0.05, 0.50);
        (Probability::new_clamped(to_p).sample(rng), false)
    };

    let next_pitch_state = pitch_state
        .with_advance(8.0, pitch_length_mirim)
        .with_channel(ArtroPlacement::Central);

    CrossActionResult {
        completed,
        target_player_id: Some(target_finisher_id),
        target_zone,
        turnover,
        scoring_attempt_ready,
        net_advantage,
        win_probability,
        next_pitch_state,
    }
}