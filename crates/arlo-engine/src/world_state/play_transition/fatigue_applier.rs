use crate::injury::exertion::{evaluate_and_resolve_exertion_injury, ExertionInjuryContext};
use crate::physical::models::aerobic::calculate_player_age;
use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::team_identity::intensity_multiplier_scale;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use std::collections::HashSet;
use uuid::Uuid;

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
                    .apply_duel_contest_strain(*attacker_id, base_mult);
            publisher.emit_physical_strain(
                *attacker_id,
                energy,
                w_bal,
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
                    .apply_duel_contest_strain(*defender_id, mult);
            publisher.emit_physical_strain(
                *defender_id,
                energy,
                w_bal,
                0.0,
            );
        }
    }
}

pub fn apply_movement_strain(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    participated_ids: &HashSet<Uuid>,
    live_seconds: f64,
) {
    for &pid in participated_ids {
        let is_home = publisher.state().teams.is_home_player(&pid);
        let team_id = if is_home {
            publisher.state().home_team_id()
        } else {
            publisher.state().away_team_id()
        };
        let player = match publisher.state().teams.find_player(&pid) {
            Some(p) => p.clone(),
            None => continue,
        };
        let table = *publisher.state().attribute_table_for(&pid);

        let (energy, w_bal) = publisher
            .state_mut()
            .apply_event_energy_decay(pid, live_seconds);

        publisher.emit_physical_strain(
            pid,
            energy,
            w_bal,
            0.0,
        );

        let age_years = calculate_player_age(&player, 0);
        let injury_profile = publisher.state().player_injury_profile(&pid);
        let player_fatigue = publisher.state().fatigue_for(&pid);
        let intensity_strain = 1.0 - w_bal;

        let exertion_ctx = ExertionInjuryContext::new(
            pid,
            team_id,
            &table,
            player_fatigue,
            injury_profile,
            intensity_strain,
            live_seconds,
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
