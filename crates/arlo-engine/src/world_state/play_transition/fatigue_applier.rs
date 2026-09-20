use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::resolution::AttributedDuelOutcome;
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
    let all_on_field_ids: Vec<Uuid> = publisher
        .state()
        .home_lineup()
        .assignments()
        .iter()
        .chain(publisher.state().away_lineup().assignments().iter())
        .map(|a| a.player().id())
        .collect();

    for pid in all_on_field_ids {
        let participated = participated_ids.contains(&pid);
        let (energy, w_bal) = publisher
            .state_mut()
            .apply_event_energy_decay(pid, live_seconds, participated);

        if participated {
            publisher.emit_physical_strain(
                pid,
                energy,
                w_bal,
                0.0,
            );
        }
    }
}

pub fn apply_dead_ball_recovery(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    dead_ball_seconds: f64,
    is_time_call: bool,
) {
    if dead_ball_seconds <= 0.0 {
        return;
    }
    let recoveries = publisher
        .state_mut()
        .apply_dead_ball_recovery(dead_ball_seconds, is_time_call);
    for (pid, recovery_amount, new_w_bal) in recoveries {
        if recovery_amount > 0.0 {
            publisher.emit_recovery_processed(pid, recovery_amount, dead_ball_seconds, new_w_bal);
        }
    }
}
