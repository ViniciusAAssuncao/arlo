use crate::ai::cognitive::decision_threshold::action_probability;
use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::Lineup;
use crate::manager_ai::cognition::derive_manager_decision_noise;
use crate::manager_ai::context::squad_fatigue_summary::SquadFatigueSummary;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::substitutions::fatigue_trigger::urgency_for_player_with_load_management;
use crate::manager_ai::substitutions::replacement_selection::{best_replacement, best_replacement_from_tables};
use crate::manager_ai::substitutions::tactical_trigger::tactical_urgency;
use crate::physical::FatigueState;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::substitution::{
    SUBSTITUTION_FATIGUE_URGENCY_ROTATION_WEIGHT, SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT,
};
use arlo_domain::{AttributeKey, RotationPolicy};
use arlo_events::SubstitutionReason;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubstitutionPlan {
    pub outgoing_id: Uuid,
    pub incoming_id: Uuid,
    pub reason: SubstitutionReason,
}

pub struct SubstitutionDecisionEngine;

impl SubstitutionDecisionEngine {
    pub fn evaluate_plans_from_tables<F, R>(
        context: &ManagerDecisionContext,
        _squad_fatigue_summary: &SquadFatigueSummary,
        lineup: &Lineup,
        bench: &MatchdaySquad,
        attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
        fatigue_lookup: F,
        rng: &mut R,
    ) -> Vec<SubstitutionPlan>
    where
        F: Fn(&Uuid) -> FatigueState,
        R: Rng + ?Sized,
    {
        let rotation_policy = context
            .manager_snapshot
            .tactical_profile
            .as_ref()
            .map(|p| p.rotation_policy())
            .unwrap_or(RotationPolicy::Situational);
        let load_management = context.manager_snapshot.load_management;
        let tac_urg = tactical_urgency(context);
        let noise_params = derive_manager_decision_noise(context.manager_snapshot.discipline);

        let mut used_candidates = HashSet::new();
        let mut plans = Vec::new();

        for assignment in lineup.assignments() {
            let pid = assignment.player().id();
            let p_fatigue = fatigue_lookup(&pid);
            let fat_urg = urgency_for_player_with_load_management(
                &p_fatigue,
                rotation_policy,
                load_management,
            );

            let noise = noise_params.sample(rng);
            let combined_urgency = ((fat_urg * SUBSTITUTION_FATIGUE_URGENCY_ROTATION_WEIGHT
                + tac_urg * SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT)
                + noise)
                .clamp(0.0, 1.0);

            let prob = action_probability(
                combined_urgency,
                context.manager_snapshot.man_management,
                SIGNAL_DETECTION_BASE_SENSITIVITY,
                SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
                DECISION_THRESHOLD_LOGIT_STEEPNESS,
            );

            if prob.sample(rng) {
                let available_candidates: Vec<_> = bench
                    .available_replacements()
                    .filter(|p| !used_candidates.contains(&p.id()))
                    .cloned()
                    .collect();

                if !available_candidates.is_empty() {
                    let target_pos = assignment.slot().position();
                    if let Some(replacement) =
                        best_replacement_from_tables(target_pos, &available_candidates, attribute_tables)
                    {
                        used_candidates.insert(replacement.id());
                        let reason = if fat_urg >= tac_urg && fat_urg > 0.0 {
                            SubstitutionReason::Fatigue
                        } else {
                            SubstitutionReason::Tactical
                        };
                        plans.push(SubstitutionPlan {
                            outgoing_id: pid,
                            incoming_id: replacement.id(),
                            reason,
                        });
                    }
                }
            }
        }

        plans
    }

    pub fn evaluate_plans<F, R>(
        context: &ManagerDecisionContext,
        _squad_fatigue_summary: &SquadFatigueSummary,
        lineup: &Lineup,
        bench: &MatchdaySquad,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        fatigue_lookup: F,
        rng: &mut R,
    ) -> Vec<SubstitutionPlan>
    where
        F: Fn(&Uuid) -> FatigueState,
        R: Rng + ?Sized,
    {
        let rotation_policy = context
            .manager_snapshot
            .tactical_profile
            .as_ref()
            .map(|p| p.rotation_policy())
            .unwrap_or(RotationPolicy::Situational);
        let load_management = context.manager_snapshot.load_management;
        let tac_urg = tactical_urgency(context);
        let noise_params = derive_manager_decision_noise(context.manager_snapshot.discipline);

        let mut used_candidates = HashSet::new();
        let mut plans = Vec::new();

        for assignment in lineup.assignments() {
            let pid = assignment.player().id();
            let p_fatigue = fatigue_lookup(&pid);
            let fat_urg = urgency_for_player_with_load_management(
                &p_fatigue,
                rotation_policy,
                load_management,
            );

            let noise = noise_params.sample(rng);
            let combined_urgency = ((fat_urg * SUBSTITUTION_FATIGUE_URGENCY_ROTATION_WEIGHT
                + tac_urg * SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT)
                + noise)
                .clamp(0.0, 1.0);

            let prob = action_probability(
                combined_urgency,
                context.manager_snapshot.man_management,
                SIGNAL_DETECTION_BASE_SENSITIVITY,
                SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
                DECISION_THRESHOLD_LOGIT_STEEPNESS,
            );

            if prob.sample(rng) {
                let available_candidates: Vec<_> = bench
                    .available_replacements()
                    .filter(|p| !used_candidates.contains(&p.id()))
                    .cloned()
                    .collect();

                if !available_candidates.is_empty() {
                    let target_pos = assignment.slot().position();
                    if let Some(replacement) =
                        best_replacement(target_pos, &available_candidates, attribute_keys)
                    {
                        used_candidates.insert(replacement.id());
                        let reason = if fat_urg >= tac_urg && fat_urg > 0.0 {
                            SubstitutionReason::Fatigue
                        } else {
                            SubstitutionReason::Tactical
                        };
                        plans.push(SubstitutionPlan {
                            outgoing_id: pid,
                            incoming_id: replacement.id(),
                            reason,
                        });
                    }
                }
            }
        }

        plans
    }

    pub fn evaluate<F, R>(
        context: &ManagerDecisionContext,
        squad_fatigue_summary: &SquadFatigueSummary,
        lineup: &Lineup,
        bench: &MatchdaySquad,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        fatigue_lookup: F,
        rng: &mut R,
    ) -> Vec<(Uuid, Uuid)>
    where
        F: Fn(&Uuid) -> FatigueState,
        R: Rng + ?Sized,
    {
        Self::evaluate_plans(
            context,
            squad_fatigue_summary,
            lineup,
            bench,
            attribute_keys,
            fatigue_lookup,
            rng,
        )
        .into_iter()
        .map(|plan| (plan.outgoing_id, plan.incoming_id))
        .collect()
    }
}