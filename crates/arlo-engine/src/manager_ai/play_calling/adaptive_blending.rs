use arlo_domain::sport_constants::{BELIEF_PRIOR_STRENGTH_MAX, BELIEF_PRIOR_STRENGTH_MIN};
use arlo_math::stats::BetaBelief;
use arlo_tactics::PlayCall;
use std::collections::HashMap;
use uuid::Uuid;

pub fn blend_fit_with_efficacy(
    fit_score: f64,
    belief: &BetaBelief,
    prior_strength: f64,
    adaptability: f64,
) -> f64 {
    let obs = belief.observations() as f64;
    let denom = obs + prior_strength;
    let data_confidence = if denom > 0.0 { obs / denom } else { 0.0 };
    let norm_adapt = (adaptability.clamp(0.0, 20.0)) / 20.0;
    let effective_weight = norm_adapt * data_confidence;
    fit_score * (1.0 - effective_weight) + belief.mean() * effective_weight
}

pub fn apply_adaptive_blending(
    ranked: &mut Vec<(usize, f64)>,
    playbook: &[PlayCall],
    efficacy_snapshot: &HashMap<Uuid, BetaBelief>,
    adaptability: f64,
) {
    let norm_adapt = (adaptability.clamp(0.0, 20.0)) / 20.0;
    let prior_strength = BELIEF_PRIOR_STRENGTH_MAX
        - norm_adapt * (BELIEF_PRIOR_STRENGTH_MAX - BELIEF_PRIOR_STRENGTH_MIN);

    for (idx, score) in ranked.iter_mut() {
        if let Some(play_call) = playbook.get(*idx) {
            let default_belief = BetaBelief::from_prior_mean_and_strength(*score, prior_strength);
            let belief = efficacy_snapshot
                .get(&play_call.id())
                .unwrap_or(&default_belief);
            *score = blend_fit_with_efficacy(*score, belief, prior_strength, adaptability);
        }
    }
}