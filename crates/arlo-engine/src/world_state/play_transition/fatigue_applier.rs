use crate::injury::exertion::{evaluate_and_resolve_exertion_injury, ExertionInjuryContext};
use crate::physical::models::aerobic::calculate_player_age;
use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::physical::models::metabolic_power::calculate_player_critical_speed_from_table;
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::spatial::SpatialTrajectory;
use crate::team_identity::intensity_multiplier_scale;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::sport_constants::SPATIAL_TICK_DURATION_SECONDS;
use arlo_events::EventSink;
use arlo_math::units::Position;
use std::collections::HashMap;
use uuid::Uuid;

pub fn apply_duel_strain(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    play_duels: &[AttributedDuelOutcome],
) {
    let pitch = *publisher.state().pitch();
    for duel in play_duels {
        let duel_kind = duel.outcome().kind();
        let base_mult = calculate_duel_intensity_multiplier(duel_kind);
        for attacker_id in duel.attacker_ids() {
            let (energy, w_bal) =
                publisher
                    .state_mut()
                    .apply_duel_anaerobic_cost(*attacker_id, 1.0, base_mult);
            let pos = publisher
                .state()
                .spatial_map()
                .get_position(attacker_id)
                .unwrap_or_else(Position::zero);
            let zone = pitch.zone_at_position(pos);
            publisher.emit_physical_strain(
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
                let team_id = if publisher
                    .state()
                    .home_offensive_position_index()
                    .contains_key(defender_id)
                {
                    publisher.state().home_team_id()
                } else {
                    publisher.state().away_team_id()
                };
                let aggression = publisher
                    .state()
                    .instructions_for_team(team_id)
                    .out_of_possession()
                    .aggression();
                base_mult * intensity_multiplier_scale(aggression)
            } else {
                base_mult
            };
            let (energy, w_bal) =
                publisher
                    .state_mut()
                    .apply_duel_anaerobic_cost(*defender_id, 1.0, mult);
            let pos = publisher
                .state()
                .spatial_map()
                .get_position(defender_id)
                .unwrap_or_else(Position::zero);
            let zone = pitch.zone_at_position(pos);
            publisher.emit_physical_strain(
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
    publisher: &mut EventPublisher<'_, impl EventSink>,
    trajectories: &HashMap<Uuid, SpatialTrajectory>,
) {
    for (pid, traj) in trajectories {
        let dist_mirim = traj.total_distance_mirim();
        let high_dist = traj.high_intensity_distance_mirim();
        let low_dist = traj.low_intensity_distance_mirim();
        let metabolic_joules = traj.metabolic_energy_joules();
        let peak_spd = traj.peak_speed_meters_per_sec();
        let zone = traj.primary_zone();

        let mut current_energy = publisher.state().fatigue_for(pid).energy();
        let mut current_w_bal = publisher.state().fatigue_for(pid).w_prime_balance();

        if dist_mirim > 0.0 {
            let (energy, w_bal) = publisher.state_mut().record_distance(*pid, dist_mirim);
            current_energy = energy;
            current_w_bal = w_bal;
        }

        let supra_time = traj.supramaximal_time_seconds();
        if supra_time > 0.0 {
            let (energy, w_bal) = publisher
                .state_mut()
                .apply_duel_anaerobic_cost(*pid, supra_time, 1.0);
            current_energy = energy;
            current_w_bal = w_bal;
        }

        if dist_mirim > 0.0 || supra_time > 0.0 || metabolic_joules > 0.0 {
            publisher.emit_physical_strain(
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

        let player_opt = publisher.state().teams.find_player(pid);
        let player_table = *publisher.state().attribute_table_for(pid);
        let player_fatigue = publisher.state().fatigue_for(pid);
        let injury_profile = publisher.state().player_injury_profile(pid);
        let team_id = if publisher.state().teams.is_home_player(pid) {
            publisher.state().home_team_id()
        } else {
            publisher.state().away_team_id()
        };

        let critical_speed = if let Some(p) = player_opt {
            calculate_player_critical_speed_from_table(p, &player_table, 0).value()
        } else {
            5.0
        };

        let age_years = if let Some(p) = player_opt {
            calculate_player_age(p, 0)
        } else {
            25.0
        };

        let exposure_duration_seconds =
            (traj.positions().len().saturating_sub(1) as f64) * SPATIAL_TICK_DURATION_SECONDS;

        if exposure_duration_seconds > 0.0 {
            let exertion_ctx = ExertionInjuryContext::new(
                *pid,
                team_id,
                &player_table,
                player_fatigue,
                injury_profile,
                peak_spd,
                critical_speed,
                high_dist,
                supra_time,
                exposure_duration_seconds,
                age_years,
            );

            let seq = publisher.state().event_sequence();
            let mut exertion_rng = publisher
                .state()
                .rng_provider()
                .indexed_rng_for(RngStream::DuelResolution, seq);

            let catalog = publisher.state().injury_catalog().clone();
            if let Some(injury_resolution) =
                evaluate_and_resolve_exertion_injury(&exertion_ctx, &catalog, &mut exertion_rng)
            {
                publisher.emit_injury_incident(&injury_resolution);
            }
        }
    }
}

pub fn apply_dead_ball_recovery(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    dead_ball_seconds: f64,
) {
    if dead_ball_seconds <= 0.0 {
        return;
    }
    let recoveries = publisher
        .state_mut()
        .apply_dead_ball_recovery(dead_ball_seconds);
    for (pid, recovery_amount, new_w_bal) in recoveries {
        if recovery_amount > 0.0 {
            publisher.emit_recovery_processed(pid, recovery_amount, dead_ball_seconds, new_w_bal);
        }
    }
}