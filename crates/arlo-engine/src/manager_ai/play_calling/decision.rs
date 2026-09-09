use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::play_calling::counter_play_bias::apply_counter_bias;
use crate::manager_ai::play_calling::situational_selection::rank_playbook;
use arlo_math::stats::{sample_categorical, softmax_weights};
use arlo_tactics::{PlayCall, PlayCallCategory, SituationalContext};
use rand::Rng;
use uuid::Uuid;

pub struct PlayCallDecisionEngine;

impl PlayCallDecisionEngine {
    pub fn select_next<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        playbook: &[PlayCall],
        situational_context: &SituationalContext,
        category: PlayCallCategory,
        last_play_call_id: Option<Uuid>,
        last_play_failed: bool,
        rng: &mut R,
    ) -> Option<PlayCall> {
        if playbook.is_empty() {
            return None;
        }

        let mut ranked = rank_playbook(playbook, situational_context, category);
        if ranked.is_empty() {
            return None;
        }

        apply_counter_bias(
            &mut ranked,
            playbook,
            last_play_call_id,
            last_play_failed,
            context.manager_snapshot.in_game_adjustments,
        );

        let norm_tk = (context.manager_snapshot.tactical_knowledge.clamp(0.0, 20.0)) / 20.0;
        let norm_adapt = (context.manager_snapshot.adaptability.clamp(0.0, 20.0)) / 20.0;
        let manager_skill = norm_tk * 0.6 + norm_adapt * 0.4;
        let steepness = 1.0 + manager_skill * 4.0;

        let utilities: Vec<f64> = ranked.iter().map(|(_, score)| *score).collect();
        let weights = softmax_weights(&utilities, steepness);

        let selected_idx = sample_categorical(&weights, rng)?;
        let (playbook_idx, _) = ranked[selected_idx];

        playbook.get(playbook_idx).cloned()
    }
}