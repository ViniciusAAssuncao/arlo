use crate::ai::cognitive::evaluate_decision_gate;
use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::Lineup;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::substitutions::budget_policy::SubstitutionBudgetPolicy;
use crate::manager_ai::substitutions::decision::SubstitutionPlan;
use crate::manager_ai::substitutions::urgency_ranking::rank_substitution_urgency;
use crate::manager_ai::substitutions::value_of_substitution::evaluate_best_substitution_value;
use crate::physical::FatigueState;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use crate::world_state::AvailabilityState;
use arlo_domain::{Position, RotationPolicy};
use arlo_events::SubstitutionReason;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn build_substitution_plans<F, R>(
    context: &ManagerDecisionContext,
    lineup: &Lineup,
    bench: &MatchdaySquad,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_lookup: F,
    availability_lookup: &dyn Fn(&Uuid) -> AvailabilityState,
    budget_policy: &SubstitutionBudgetPolicy,
    rng: &mut R,
) -> Vec<SubstitutionPlan>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let rankings = rank_substitution_urgency(context, lineup, &fatigue_lookup, availability_lookup);
    let rotation_policy = context
        .manager_snapshot
        .tactical_profile
        .as_ref()
        .map(|p| p.rotation_policy())
        .unwrap_or(RotationPolicy::Situational);

    let mut used_candidates = HashSet::new();
    let mut plans = Vec::new();

    for ranked in rankings {
        if plans.len() >= budget_policy.max_subs_per_stoppage() {
            break;
        }

        let pid = ranked.player_id;
        let is_artrine = lineup
            .get_slot_for_player(&pid)
            .map(|s| s.offensive_position() == Position::Artrine || s.position() == Position::Artrine)
            .unwrap_or(false);
        let is_passer = lineup
            .get_slot_for_player(&pid)
            .map(|s| s.offensive_position() == Position::Passer || s.position() == Position::Passer)
            .unwrap_or(false);

        if (is_artrine || is_passer) && rotation_policy == RotationPolicy::StrictCore {
            if ranked.fatigue_urgency < 0.85 && ranked.disciplinary_urgency < 0.5 {
                continue;
            }
        }

        let gate_prob = evaluate_decision_gate(
            ranked.total_stimulus,
            context.manager_snapshot.man_management,
            context.manager_snapshot.discipline,
        );

        if !gate_prob.sample(rng) {
            continue;
        }

        let assignment = match lineup.get_assignment(&pid) {
            Some(a) => a,
            None => continue,
        };

        let available_candidates: Vec<_> = bench
            .available_replacements()
            .filter(|p| !used_candidates.contains(&p.id()))
            .filter(|p| availability_lookup(&p.id()).is_active())
            .cloned()
            .collect();

        if available_candidates.is_empty() {
            continue;
        }

        let slot = assignment.slot();
        let target_pos = if slot.defensive_position() == Position::Goalguard
            || slot.position() == Position::Goalguard
            || slot.offensive_position() == Position::Goalguard
        {
            Position::Goalguard
        } else {
            slot.position()
        };

        let out_fatigue = fatigue_lookup(&pid);

        if let Some(assessment) = evaluate_best_substitution_value(
            assignment.player(),
            target_pos,
            &out_fatigue,
            &available_candidates,
            attribute_tables,
            rotation_policy,
        ) {
            used_candidates.insert(assessment.replacement.id());
            let reason = if ranked.disciplinary_urgency > 0.0 {
                SubstitutionReason::Disciplinary
            } else if ranked.fatigue_urgency >= ranked.tactical_urgency && ranked.fatigue_urgency > 0.0 {
                SubstitutionReason::Fatigue
            } else {
                SubstitutionReason::Tactical
            };
            plans.push(SubstitutionPlan {
                outgoing_id: pid,
                incoming_id: assessment.replacement.id(),
                reason,
            });
        }
    }

    plans
}
