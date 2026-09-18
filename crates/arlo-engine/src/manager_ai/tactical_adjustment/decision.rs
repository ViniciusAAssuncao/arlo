use crate::ai::cognitive::ManagerDecisionFactory;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::tactical_adjustment::fit_scoring::score_candidate;
use arlo_domain::sport_constants::{
    PROFILE_SWITCH_FIT_MARGIN_BASE, PROFILE_SWITCH_FLEXIBILITY_SCALE,
};
use arlo_tactics::{SituationalContext, TeamTacticalProfile};
use rand::Rng;
use uuid::Uuid;

pub fn calculate_tactical_adjustment_stimulus(
    available_profiles: &[TeamTacticalProfile],
    active_profile_id: Uuid,
    situational_context: &SituationalContext,
    context: &ManagerDecisionContext,
) -> Option<(Uuid, f64)> {
    if available_profiles.is_empty() {
        return None;
    }

    let mtp = context.manager_snapshot.tactical_profile.as_ref();
    let active_profile = available_profiles
        .iter()
        .find(|p| p.id() == active_profile_id);
    let active_score = match active_profile {
        Some(p) => score_candidate(p, situational_context, mtp),
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

        let score = score_candidate(profile, situational_context, mtp);
        if score > best_candidate_score {
            best_candidate_score = score;
            best_candidate_id = Some(profile.id());
        }
    }

    let candidate_id = best_candidate_id?;
    let stimulus = (best_candidate_score - threshold + 0.5).clamp(0.0, 1.0);
    Some((candidate_id, stimulus))
}

pub struct TacticalAdjustmentDecisionEngine;

impl TacticalAdjustmentDecisionEngine {
    pub fn evaluate<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        available_profiles: &[TeamTacticalProfile],
        active_profile_id: Uuid,
        situational_context: &SituationalContext,
        rng: &mut R,
    ) -> Option<Uuid> {
        let (candidate_id, stimulus) = calculate_tactical_adjustment_stimulus(
            available_profiles,
            active_profile_id,
            situational_context,
            context,
        )?;

        if ManagerDecisionFactory::decide(
            stimulus,
            context.manager_snapshot.adaptability,
            context.manager_snapshot.discipline,
            rng,
        ) {
            Some(candidate_id)
        } else {
            None
        }
    }
}