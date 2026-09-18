use crate::injury::exertion::{evaluate_and_resolve_exertion_injury, ExertionInjuryContext};
use crate::physical::models::aerobic::calculate_player_age;
use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::physical::models::metabolic_power::calculate_player_critical_speed_from_table;
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::team_identity::intensity_multiplier_scale;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::{AttributeKey, PitchZone, Player};
use arlo_events::EventSink;

pub fn apply_duel_strain(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    play_duels: &[AttributedDuelOutcome],
) {
    for duel in play_duels {
        let duel_kind = duel.outcome().kind();
        let base_mult = calculate_duel_intensity_multiplier(duel_kind);
        for attacker_id in duel.attacker_ids() {
            let (energy, w_bal) =
                publisher
                    .state_mut()
                    .apply_duel_anaerobic_cost(*attacker_id, 1.0, base_mult);
            publisher.emit_physical_strain(
                *attacker_id,
                energy,
                w_bal,
                0.0,
                0.0,
                0.0,
                0.0,
                PitchZone::OpenField,
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
            publisher.emit_physical_strain(
                *defender_id,
                energy,
                w_bal,
                0.0,
                0.0,
                0.0,
                0.0,
                PitchZone::OpenField,
                0.0,
            );
        }
    }
}

pub fn apply_movement_strain(publisher: &mut EventPublisher<'_, impl EventSink>) {
    let home_lineup = publisher.state().home_lineup_arc();
    let away_lineup = publisher.state().away_lineup_arc();
    let all_players: Vec<(&Player, bool)> = home_lineup
        .assignments()
        .iter()
        .map(|a| (a.player(), true))
        .chain(away_lineup.assignments().iter().map(|a| (a.player(), false)))
        .collect();

    let is_home_offense = publisher
        .state()
        .possession()
        .role()
        .is_offense(publisher.state().home_team_id());

    for (player, is_home) in all_players {
        let pid = player.id();
        let is_offense = is_home == is_home_offense;
        let table = *publisher.state().attribute_table_for(&pid);
        let pace = table.get(AttributeKey::Pace) / 20.0;
        let stamina = table.get(AttributeKey::Stamina) / 20.0;

        let base_dist_mirim = if is_offense {
            4.0 + pace * 3.0
        } else {
            3.5 + pace * 2.5
        };

        let high_dist_mirim = if is_offense {
            (base_dist_mirim * 0.40 * (1.0 + pace * 0.2)).max(0.5)
        } else {
            (base_dist_mirim * 0.35 * (1.0 + pace * 0.2)).max(0.5)
        };
        let low_dist_mirim = (base_dist_mirim - high_dist_mirim).max(0.0);

        let (energy, w_bal) = publisher.state_mut().record_distance(pid, base_dist_mirim);

        let supra_time = (high_dist_mirim / 6.0).max(0.0);
        if supra_time > 0.0 {
            publisher
                .state_mut()
                .apply_duel_anaerobic_cost(pid, supra_time, 1.0);
        }

        let metabolic_joules = base_dist_mirim * 300.0 * (1.5 - stamina * 0.5);
        let peak_spd = 5.0 + pace * 4.0;
        let zone = PitchZone::OpenField;

        publisher.emit_physical_strain(
            pid,
            energy,
            w_bal,
            base_dist_mirim,
            high_dist_mirim,
            low_dist_mirim,
            metabolic_joules,
            zone,
            peak_spd,
        );

        let team_id = if is_home {
            publisher.state().home_team_id()
        } else {
            publisher.state().away_team_id()
        };

        let critical_speed =
            calculate_player_critical_speed_from_table(player, &table, 0).value();
        let age_years = calculate_player_age(player, 0);
        let exposure_duration_seconds = (base_dist_mirim / 4.0).clamp(1.0, 10.0);

        let injury_profile = publisher.state().player_injury_profile(&pid);
        let player_fatigue = publisher.state().fatigue_for(&pid);

        let exertion_ctx = ExertionInjuryContext::new(
            pid,
            team_id,
            &table,
            player_fatigue,
            injury_profile,
            peak_spd,
            critical_speed,
            high_dist_mirim,
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