use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::play_calling::adaptive_blending::apply_adaptive_blending;
use crate::manager_ai::play_calling::counter_play_bias::apply_counter_bias;
use arlo_math::stats::{sample_categorical, softmax_weights, BetaBelief};
use arlo_tactics::PlayCall;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PlayCallDecisionEngine;

impl PlayCallDecisionEngine {
    pub fn select_next<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        playbook: &[PlayCall],
        ranked: &[(usize, f64)],
        efficacy_snapshot: &HashMap<Uuid, BetaBelief>,
        last_play_call_id: Option<Uuid>,
        last_play_failed: bool,
        rng: &mut R,
    ) -> Option<PlayCall> {
        if playbook.is_empty() || ranked.is_empty() {
            return None;
        }

        let mut candidate_ranked = ranked.to_vec();

        apply_counter_bias(
            &mut candidate_ranked,
            playbook,
            last_play_call_id,
            last_play_failed,
            context.manager_snapshot.in_game_adjustments,
        );

        apply_adaptive_blending(
            &mut candidate_ranked,
            playbook,
            efficacy_snapshot,
            context.manager_snapshot.adaptability,
        );

        let norm_tk = (context.manager_snapshot.tactical_knowledge.clamp(0.0, 20.0)) / 20.0;
        let norm_adapt = (context.manager_snapshot.adaptability.clamp(0.0, 20.0)) / 20.0;
        let manager_skill = norm_tk * 0.6 + norm_adapt * 0.4;
        let steepness = 1.0 + manager_skill * 4.0;

        let utilities: Vec<f64> = candidate_ranked.iter().map(|(_, score)| *score).collect();
        let weights = softmax_weights(&utilities, steepness);

        let selected_idx = sample_categorical(&weights, rng)?;
        let (playbook_idx, _) = candidate_ranked[selected_idx];

        playbook.get(playbook_idx).cloned()
    }
}
