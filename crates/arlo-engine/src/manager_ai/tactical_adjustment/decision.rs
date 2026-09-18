use crate::ai::cognitive::sample_manager_action;
use crate::manager_ai::cognition::sample_manager_decision_noise;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::tactical_adjustment::fit_scoring::score_candidate;
use arlo_domain::sport_constants::tactical_adaptation::{
    PROFILE_SWITCH_FIT_MARGIN_BASE, PROFILE_SWITCH_FLEXIBILITY_SCALE,
};
use arlo_tactics::{SituationalContext, TeamTacticalProfile};
use rand::Rng;
use uuid::Uuid;

pub struct TacticalAdjustmentDecisionEngine;

impl TacticalAdjustmentDecisionEngine {
    pub fn evaluate<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        available_profiles: &[TeamTacticalProfile],
        active_profile_id: Uuid,
        situational_context: &SituationalContext,
        rng: &mut R,
    ) -> Option<Uuid> {
        if available_profiles.is_empty() {
            return None;
        }

        let mtp = context.manager_snapshot.tactical_profile.as_ref();
        let discipline = context.manager_snapshot.discipline;

        let calculate_perceived_score = |profile: &TeamTacticalProfile, rng_val: &mut R| -> f64 {
            let raw_score = score_candidate(profile, situational_context, mtp);
            let noise = sample_manager_decision_noise(discipline, rng_val);
            (raw_score + noise).clamp(0.0, 1.0)
        };

        let active_profile = available_profiles
            .iter()
            .find(|p| p.id() == active_profile_id);
        let active_score = match active_profile {
            Some(p) => calculate_perceived_score(p, rng),
            None => 0.0,
        };

        let effective_flexibility = context.manager_snapshot.effective_flexibility;
        let required_margin = (PROFILE_SWITCH_FIT_MARGIN_BASE
            - (effective_flexibility * PROFILE_SWITCH_FLEXIBILITY_SCALE))
            .max(0.0);

        let threshold = active_score + required_margin;

        let mut best_candidate_id = None;
        let mut best_candidate_score = threshold;

        for profile in available_profiles {
            if profile.id() == active_profile_id {
                continue;
            }

            let perceived_score = calculate_perceived_score(profile, rng);
            if perceived_score > best_candidate_score {
                best_candidate_score = perceived_score;
                best_candidate_id = Some(profile.id());
            }
        }

        if let Some(candidate_id) = best_candidate_id {
            let stimulus = (best_candidate_score - threshold + 0.5).clamp(0.0, 1.0);
            if sample_manager_action(stimulus, context.manager_snapshot.adaptability, rng) {
                return Some(candidate_id);
            }
        }

        None
    }
}