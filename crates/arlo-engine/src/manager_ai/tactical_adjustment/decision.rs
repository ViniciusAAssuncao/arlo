use crate::manager_ai::cognition::derive_manager_decision_noise;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::tactical_adjustment::fit_scoring::score_candidate;
use arlo_domain::sport_constants::tactical_adaptation::{
    PROFILE_SWITCH_FIT_MARGIN_BASE, PROFILE_SWITCH_FLEXIBILITY_SCALE,
    PROFILE_SWITCH_TACTICAL_KNOWLEDGE_NOISE_SCALE,
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
        let norm_tk = (context.manager_snapshot.tactical_knowledge.clamp(0.0, 20.0)) / 20.0;
        let noise_scale = (1.0 - norm_tk) * PROFILE_SWITCH_TACTICAL_KNOWLEDGE_NOISE_SCALE;
        let discipline_noise_params =
            derive_manager_decision_noise(context.manager_snapshot.discipline);

        let calculate_perceived_score = |profile: &TeamTacticalProfile, rng: &mut R| -> f64 {
            let raw_score = score_candidate(profile, situational_context, mtp);
            let id_hash = (profile.id().as_u128() & 0xFFFF) as f64 / 65535.0;
            let id_noise = (id_hash * 2.0 - 1.0) * noise_scale;
            let discipline_noise = discipline_noise_params.sample(rng);
            (raw_score + id_noise + discipline_noise).clamp(0.0, 1.0)
        };

        let active_profile = available_profiles.iter().find(|p| p.id() == active_profile_id);
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

        best_candidate_id
    }
}