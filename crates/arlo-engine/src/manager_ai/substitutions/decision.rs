use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::Lineup;
use crate::manager_ai::context::squad_fatigue_summary::SquadFatigueSummary;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::substitutions::budget_policy::SubstitutionBudgetPolicy;
use crate::manager_ai::substitutions::plan_builder::build_substitution_plans;
use crate::physical::FatigueState;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use crate::world_state::AvailabilityState;
use arlo_events::SubstitutionReason;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubstitutionPlan {
    pub outgoing_id: Uuid,
    pub incoming_id: Uuid,
    pub reason: SubstitutionReason,
}

pub struct SubstitutionDecisionEngine;

impl SubstitutionDecisionEngine {
    pub fn evaluate_plans<F, R>(
        context: &ManagerDecisionContext,
        _squad_fatigue_summary: &SquadFatigueSummary,
        lineup: &Lineup,
        bench: &MatchdaySquad,
        attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
        fatigue_lookup: F,
        availability_lookup: &dyn Fn(&Uuid) -> AvailabilityState,
        rng: &mut R,
    ) -> Vec<SubstitutionPlan>
    where
        F: Fn(&Uuid) -> FatigueState,
        R: Rng + ?Sized,
    {
        let budget_policy = SubstitutionBudgetPolicy::default();
        build_substitution_plans(
            context,
            lineup,
            bench,
            attribute_tables,
            fatigue_lookup,
            availability_lookup,
            &budget_policy,
            rng,
        )
    }

    pub fn evaluate<F, R>(
        context: &ManagerDecisionContext,
        squad_fatigue_summary: &SquadFatigueSummary,
        lineup: &Lineup,
        bench: &MatchdaySquad,
        attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
        fatigue_lookup: F,
        availability_lookup: &dyn Fn(&Uuid) -> AvailabilityState,
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
            attribute_tables,
            fatigue_lookup,
            availability_lookup,
            rng,
        )
        .into_iter()
        .map(|plan| (plan.outgoing_id, plan.incoming_id))
        .collect()
    }
}
