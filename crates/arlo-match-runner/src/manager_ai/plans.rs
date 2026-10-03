use super::context::ManagerDecisionContext;
use super::diagnosis::TeamDiagnosis;
use arlo_domain::Position;
use arlo_manager_control::PreparedPlanIntent;

#[derive(Debug, Clone)]
pub struct PreparedPlanCandidate {
    pub intent: PreparedPlanIntent,
    pub utility: f64,
    pub collective_gain: f64,
    pub fit_gain: f64,
    pub change_cost: f64,
    pub changed_roles: usize,
    pub formation_changed: bool,
}

pub(super) fn generate(
    context: &ManagerDecisionContext,
    diagnosis: &TeamDiagnosis,
) -> Vec<PreparedPlanCandidate> {
    if !super::cooldowns::DecisionReadiness::from_context(context).plan {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for plan in &context.plans {
        let mut fit_gain = 0.0;
        let mut valid = plan.assignments.len() == context.slots.len();
        for assignment in &plan.assignments {
            let Some(player) = context.player(assignment.destination.player_id) else {
                valid = false;
                break;
            };
            let current = &context.slots[assignment.current_index];
            let changed = current.offensive_position != assignment.destination.offensive_position
                || current.defensive_position != assignment.destination.defensive_position
                || current.role != assignment.destination.role;
            let minimum = if matches!(
                assignment.destination.offensive_position,
                Position::Artrine | Position::Passer | Position::Goalguard
            ) {
                0.6
            } else {
                0.4
            };
            if changed
                && (player.injured
                    || assignment.proficiency < minimum
                    || player
                        .entered_at
                        .is_some_and(|last| context.elapsed - last < 600.0))
            {
                valid = false;
                break;
            }
            let physical = (0.58 + player.energy * 0.42)
                * (0.72 + player.morale.clamp(0.0, 100.0) * 0.0028)
                * player.settling;
            fit_gain +=
                (assignment.quality - player.fits[assignment.current_index].quality) * physical;
        }
        if !valid {
            continue;
        }
        fit_gain /= plan.assignments.len().max(1) as f64;
        if fit_gain < -0.06 {
            continue;
        }
        let delta = plan.effect;
        let current = context.current_effect;
        let skills = context.skills;
        let collective_gain =
            2.4 * (delta.attack - current.attack) * diagnosis.attack * (0.4 + 0.6 * skills.offense)
                + 2.2
                    * (delta.defense - current.defense)
                    * diagnosis.defense
                    * (0.4 + 0.6 * skills.defense)
                + 1.6
                    * (delta.control - current.control)
                    * diagnosis.control
                    * (0.4 + 0.6 * skills.artro)
                + 1.6 * (delta.security - current.security) * diagnosis.security
                + 1.2 * (current.load - delta.load) * diagnosis.energy * (0.4 + 0.6 * skills.load)
                + (delta.discipline - current.discipline) * diagnosis.discipline;
        let change_cost = 0.02
            + (1.0 - skills.flexibility) * 0.025
            + (1.0 - skills.adaptability) * 0.025
            + plan.changed_roles as f64 / context.slots.len().max(1) as f64 * 0.05
            + if plan.formation_changed {
                0.015 + (1.0 - skills.knowledge) * 0.025
            } else {
                0.0
            };
        candidates.push(PreparedPlanCandidate {
            intent: PreparedPlanIntent::new(plan.id),
            utility: collective_gain + fit_gain * 1.5 - change_cost,
            collective_gain,
            fit_gain,
            change_cost,
            changed_roles: plan.changed_roles,
            formation_changed: plan.formation_changed,
        });
    }
    candidates.sort_by(|a, b| {
        b.utility
            .total_cmp(&a.utility)
            .then_with(|| a.intent.plan_id().cmp(&b.intent.plan_id()))
    });
    candidates
}
