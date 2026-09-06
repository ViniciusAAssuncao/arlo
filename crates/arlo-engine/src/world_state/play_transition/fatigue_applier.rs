use crate::artrine::ArtrineExecutionOutcome;
use crate::physical::models::anaerobic::calculate_duel_intensity_multiplier;
use crate::physical::systems::pacing::{calculate_player_pacing_state, is_player_near_ball};
use crate::physical::systems::positional_strain::calculate_transit_strain_multiplier;
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::context_analyzer::analyze_match_state;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::event_dispatcher::{
    emit_physical_strain, emit_recovery_processed,
};
use arlo_domain::{Player, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;

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

pub fn apply_transit_movement_strain(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    pass_phase: &PassPhaseResult<'_>,
    execution_outcome: &ArtrineExecutionOutcome,
    live_seconds: f64,
) {
    let game_state_pressure = analyze_match_state(state);
    let ball_pos = execution_outcome.end_position;
    let runner_id = execution_outcome
        .receiver_id
        .unwrap_or(pass_phase.artrine.id());

    let all_players: Vec<Player> = state
        .home_lineup()
        .players()
        .into_iter()
        .chain(state.away_lineup().players().into_iter())
        .cloned()
        .collect();

    for p in &all_players {
        let pid = p.id();
        let p_fatigue = state.fatigue_for(&pid);
        let is_home = state.home_offensive_position_index().contains_key(&pid);
        let team_id = if is_home {
            state.home_team_id()
        } else {
            state.away_team_id()
        };
        let p_pos = state
            .position_index_for_team(team_id)
            .get(&pid)
            .copied()
            .unwrap_or(DomainPosition::CenterOffense);

        let is_near = state
            .spatial_map()
            .get_position(&pid)
            .map_or(false, |pos| is_player_near_ball(pos, ball_pos, 15.0))
            || pid == runner_id
            || pid == pass_phase.passer.id();

        let pacing_state = calculate_player_pacing_state(
            p,
            state.attribute_keys(),
            is_near,
            &game_state_pressure,
            &p_fatigue,
            0,
        );

        let transit_mult = calculate_transit_strain_multiplier(p_pos);
        let cruise_speed_m_s = pacing_state.target_cruise_speed().value();
        let base_transit_mirim = (cruise_speed_m_s * live_seconds) / MIRIM_TO_METERS;
        let mut player_dist = base_transit_mirim * transit_mult;

        if pid == runner_id && execution_outcome.mirins_advanced > 0.0 {
            player_dist += execution_outcome.mirins_advanced;
        }

        if player_dist > 0.0 {
            let (energy, w_bal) = state.record_distance(pid, player_dist);
            emit_physical_strain(state, sink, pid, energy, w_bal, player_dist);
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