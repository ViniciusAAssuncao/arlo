use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::resolution::AttributedDuelOutcome;
use crate::spatial::SpatialTrajectory;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::event_dispatcher::{
    emit_physical_strain, emit_recovery_processed,
};
use arlo_events::EventSink;
use std::collections::HashMap;
use uuid::Uuid;

pub fn apply_duel_strain(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    play_duels: &[AttributedDuelOutcome],
) {
    for duel in play_duels {
        let duel_kind = duel.outcome().kind();
        let mult = calculate_duel_intensity_multiplier(duel_kind);
        for attacker_id in duel.attacker_ids() {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*attacker_id, 1.0, mult);
            emit_physical_strain(state, sink, *attacker_id, energy, w_bal, 0.0);
        }
        for defender_id in duel.defender_ids() {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*defender_id, 1.0, mult);
            emit_physical_strain(state, sink, *defender_id, energy, w_bal, 0.0);
        }
    }
}

pub fn apply_kinematic_movement_strain(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    trajectories: &HashMap<Uuid, SpatialTrajectory>,
) {
    for (pid, traj) in trajectories {
        let dist_mirim = traj.total_distance_mirim();
        if dist_mirim > 0.0 {
            let (energy, w_bal) = state.record_distance(*pid, dist_mirim);
            emit_physical_strain(state, sink, *pid, energy, w_bal, dist_mirim);
        }
        let supra_time = traj.supramaximal_time_seconds();
        if supra_time > 0.0 {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*pid, supra_time, 1.0);
            emit_physical_strain(state, sink, *pid, energy, w_bal, 0.0);
        }
    }
}

pub fn apply_dead_ball_recovery(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    dead_ball_seconds: f64,
) {
    if dead_ball_seconds <= 0.0 {
        return;
    }
    let recoveries = state.apply_dead_ball_recovery(dead_ball_seconds);
    for (pid, recovery_amount, new_w_bal) in recoveries {
        if recovery_amount > 0.0 {
            emit_recovery_processed(state, sink, pid, recovery_amount, dead_ball_seconds, new_w_bal);
        }
    }
}