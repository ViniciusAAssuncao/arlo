use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::resolution::AttributedDuelOutcome;
use crate::spatial::SpatialTrajectory;
use crate::team_identity::intensity_multiplier_scale;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::event_dispatcher::{
    emit_physical_strain, emit_recovery_processed,
};
use arlo_events::EventSink;
use arlo_math::units::Position;
use std::collections::HashMap;
use uuid::Uuid;

pub fn apply_duel_strain(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    play_duels: &[AttributedDuelOutcome],
) {
    let pitch = *state.pitch();
    for duel in play_duels {
        let duel_kind = duel.outcome().kind();
        let base_mult = calculate_duel_intensity_multiplier(duel_kind);
        for attacker_id in duel.attacker_ids() {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*attacker_id, 1.0, base_mult);
            let pos = state
                .spatial_map()
                .get_position(attacker_id)
                .unwrap_or_else(Position::zero);
            let zone = pitch.zone_at_position(pos);
            emit_physical_strain(
                state,
                sink,
                *attacker_id,
                energy,
                w_bal,
                0.0,
                0.0,
                0.0,
                0.0,
                zone,
                0.0,
            );
        }
        for defender_id in duel.defender_ids() {
            let mult = if duel_kind.is_contact_duel() {
                let team_id = if state.home_offensive_position_index().contains_key(defender_id) {
                    state.home_team_id()
                } else {
                    state.away_team_id()
                };
                let aggression = state.instructions_for_team(team_id).out_of_possession().aggression();
                base_mult * intensity_multiplier_scale(aggression)
            } else {
                base_mult
            };
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*defender_id, 1.0, mult);
            let pos = state
                .spatial_map()
                .get_position(defender_id)
                .unwrap_or_else(Position::zero);
            let zone = pitch.zone_at_position(pos);
            emit_physical_strain(
                state,
                sink,
                *defender_id,
                energy,
                w_bal,
                0.0,
                0.0,
                0.0,
                0.0,
                zone,
                0.0,
            );
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
        let high_dist = traj.high_intensity_distance_mirim();
        let low_dist = traj.low_intensity_distance_mirim();
        let metabolic_joules = traj.metabolic_energy_joules();
        let peak_spd = traj.peak_speed_meters_per_sec();
        let zone = traj.primary_zone();

        let mut current_energy = state.fatigue_for(pid).energy();
        let mut current_w_bal = state.fatigue_for(pid).w_prime_balance();

        if dist_mirim > 0.0 {
            let (energy, w_bal) = state.record_distance(*pid, dist_mirim);
            current_energy = energy;
            current_w_bal = w_bal;
        }

        let supra_time = traj.supramaximal_time_seconds();
        if supra_time > 0.0 {
            let (energy, w_bal) = state.apply_duel_anaerobic_cost(*pid, supra_time, 1.0);
            current_energy = energy;
            current_w_bal = w_bal;
        }

        if dist_mirim > 0.0 || supra_time > 0.0 || metabolic_joules > 0.0 {
            emit_physical_strain(
                state,
                sink,
                *pid,
                current_energy,
                current_w_bal,
                dist_mirim,
                high_dist,
                low_dist,
                metabolic_joules,
                zone,
                peak_spd,
            );
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
            emit_recovery_processed(
                state,
                sink,
                pid,
                recovery_amount,
                dead_ball_seconds,
                new_w_bal,
            );
        }
    }
}