use crate::lineup_runtime::Lineup;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::substitutions::disciplinary_trigger::disciplinary_urgency;
use crate::manager_ai::substitutions::fatigue_trigger::urgency_for_player_with_load_management;
use crate::manager_ai::substitutions::tactical_trigger::tactical_urgency;
use crate::physical::FatigueState;
use crate::world_state::AvailabilityState;
use arlo_domain::sport_constants::{
    SUBSTITUTION_DISCIPLINARY_URGENCY_WEIGHT, SUBSTITUTION_FATIGUE_URGENCY_ROTATION_WEIGHT,
    SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT,
};
use arlo_domain::RotationPolicy;
use uuid::Uuid;

pub struct PlayerUrgency {
    pub player_id: Uuid,
    pub total_stimulus: f64,
    pub fatigue_urgency: f64,
    pub tactical_urgency: f64,
    pub disciplinary_urgency: f64,
}

pub fn rank_substitution_urgency<F>(
    context: &ManagerDecisionContext,
    lineup: &Lineup,
    fatigue_lookup: F,
    availability_lookup: &dyn Fn(&Uuid) -> AvailabilityState,
) -> Vec<PlayerUrgency>
where
    F: Fn(&Uuid) -> FatigueState,
{
    let rotation_policy = context
        .manager_snapshot
        .tactical_profile
        .as_ref()
        .map(|p| p.rotation_policy())
        .unwrap_or(RotationPolicy::Situational);
    let load_management = context.manager_snapshot.load_management;
    let tac_urg = tactical_urgency(context);

    let mut list = Vec::with_capacity(lineup.len());

    for assignment in lineup.assignments() {
        let pid = assignment.player().id();
        let p_avail = availability_lookup(&pid);
        if p_avail.is_expelled() {
            continue;
        }

        let p_fatigue = fatigue_lookup(&pid);
        let fat_urg = urgency_for_player_with_load_management(
            &p_fatigue,
            rotation_policy,
            load_management,
        );
        let disc_urg = disciplinary_urgency(p_avail);

        let total_stimulus = (fat_urg * SUBSTITUTION_FATIGUE_URGENCY_ROTATION_WEIGHT
            + tac_urg * SUBSTITUTION_TACTICAL_URGENCY_DEFICIT_WEIGHT
            + disc_urg * SUBSTITUTION_DISCIPLINARY_URGENCY_WEIGHT)
            .clamp(0.0, 1.0);

        list.push(PlayerUrgency {
            player_id: pid,
            total_stimulus,
            fatigue_urgency: fat_urg,
            tactical_urgency: tac_urg,
            disciplinary_urgency: disc_urg,
        });
    }

    list.sort_by(|a, b| {
        b.total_stimulus
            .partial_cmp(&a.total_stimulus)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    list
}
