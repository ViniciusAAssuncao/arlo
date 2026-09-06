use crate::artrine::{
    calculate_normalized_proximity, execute_artrine_decision, resolve_artrine_decision,
    translate_artrine_decision_made, ArtrineExecutionOutcome,
};
use crate::error::EngineResult;
use crate::match_decision::event_translation::create_envelope;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::match_decision::target_selection::{calculate_player_target_weight, ReceptionRole};
use crate::resolution::DuelContext;
use crate::rng::RngStream;
use crate::spatial::{calculate_artro_advance_pitch_control, find_next_artro_position};
use crate::time::DurationLedger;
use crate::world_state::cta_pass::resolve_pass_phase;
use crate::world_state::cta_transition::apply_play_transition;
use crate::world_state::match_state::MatchState;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

fn build_finished_match_outcome(state: &MatchState) -> DetailedPlayOutcome {
    let scrimmage = state.possession().scrimmage_point();
    DetailedPlayOutcome {
        offense_team_id: state.possession().offense(),
        defense_team_id: state.possession().defense(),
        passer_id: Uuid::nil(),
        artrine_id: Uuid::nil(),
        down_number: state.possession().down() as u32,
        scrimmage_x_mirim: scrimmage.raw().0 / MIRIM_TO_METERS,
        pass_completed: false,
        pass_is_aerial: false,
        reception_point: scrimmage,
        drives_recorded: 0,
        mirins_advanced: 0.0,
        duels: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        out_of_bounds: false,
        arbitral_stoppage: true,
        last_valid_possession_point: scrimmage,
        possession_control_seconds: None,
        scoring_decision: ScoringDecision::NoOpportunity,
    }
}

pub fn step_call_to_action(
    state: &mut MatchState,
    sink: &mut impl EventSink,
) -> EngineResult<DetailedPlayOutcome> {
    if state.is_match_finished() {
        return Ok(build_finished_match_outcome(state));
    }

    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
    let (offense_team_id, defense_team_id) = if is_home_offense {
        (state.home_team_id(), state.away_team_id())
    } else {
        (state.away_team_id(), state.home_team_id())
    };

    let (offense_lineup, defense_lineup) = if is_home_offense {
        (state.home_lineup().clone(), state.away_lineup().clone())
    } else {
        (state.away_lineup().clone(), state.home_lineup().clone())
    };

    let offense_pos_index = state.offensive_position_index_for_team(offense_team_id).clone();
    let defense_pos_index = state.defensive_position_index_for_team(defense_team_id).clone();

    let offense_players: Vec<&Player> = offense_lineup.players();
    let defense_players: Vec<&Player> = defense_lineup.players();

    let pass_phase = resolve_pass_phase(
        state,
        &offense_players,
        &defense_players,
        is_home_offense,
        offense_team_id,
        defense_team_id,
        sink,
    )?;

    let (chosen_decision, execution_outcome) = if pass_phase.pass_completed {
        let context = if is_home_offense {
            DuelContext::attacker_home()
        } else {
            DuelContext::defender_home()
        };

        let pitch = *state.pitch();
        let attribute_keys = state.attribute_keys().clone();
        let is_bonus_phase = state.possession().is_bonus_phase();
        let drives_in_series = state.drives_in_current_series();
        let remaining_downs = state.possession().series_state().remaining_downs();
        let is_last_down = state.possession().series_state().is_last_down();
        let advanced_mirins = state.possession().series_state().advanced_mirins();

        let normalized_proximity = calculate_normalized_proximity(
            pass_phase.reception_point,
            &pitch,
            is_home_offense,
        );

        let target_candidates: Vec<&Player> = offense_players
            .iter()
            .copied()
            .filter(|p| p.id() != pass_phase.artrine.id())
            .collect();

        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                calculate_player_target_weight(
                    p,
                    state.spatial_map(),
                    &pitch,
                    &offense_pos_index,
                    &attribute_keys,
                    is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                )
            })
            .fold(0.0_f64, f64::max);

        let next_artro_pos = find_next_artro_position(
            pass_phase.reception_point,
            &pitch,
            is_home_offense,
        );

        let home_fatigue = state.home_fatigue().clone();
        let away_fatigue = state.away_fatigue().clone();
        let fatigue_lookup = move |id: &Uuid| {
            home_fatigue
                .get(id)
                .or_else(|| away_fatigue.get(id))
                .copied()
                .unwrap_or_default()
        };

        let pitch_control_ahead = calculate_artro_advance_pitch_control(
            pass_phase.artrine,
            &target_candidates,
            &defense_players,
            state.spatial_map(),
            &attribute_keys,
            &fatigue_lookup,
            pass_phase.reception_point,
            next_artro_pos,
            &pitch,
        );

        let seq_decision = state.next_sequence();
        let mut decision_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::ArtrineDecision, seq_decision);

        let decision_result = resolve_artrine_decision(
            pass_phase.artrine,
            &attribute_keys,
            normalized_proximity,
            drives_in_series,
            remaining_downs,
            pass_phase.pass_duel_outcome.outcome().net_advantage(),
            is_last_down,
            is_bonus_phase,
            advanced_mirins,
            best_available_target_weight,
            pass_phase.reception_point,
            next_artro_pos,
            pitch_control_ahead,
            pitch.length_mirim(),
            &mut decision_rng,
        );

        let chosen_decision = decision_result.chosen();

        let decision_event = translate_artrine_decision_made(
            pass_phase.artrine.id(),
            chosen_decision,
            pass_phase.down_number,
            decision_result.chosen_probability(),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, decision_event));

        let seq_execution = state.next_sequence();
        let mut execution_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::DuelResolution, seq_execution);

        let home_fatigue_exec = state.home_fatigue().clone();
        let away_fatigue_exec = state.away_fatigue().clone();
        let fatigue_lookup_exec = move |id: &Uuid| {
            home_fatigue_exec
                .get(id)
                .or_else(|| away_fatigue_exec.get(id))
                .copied()
                .unwrap_or_default()
        };

        let execution_outcome = execute_artrine_decision(
            chosen_decision,
            pass_phase.artrine,
            &offense_players,
            &offense_pos_index,
            &defense_players,
            &defense_pos_index,
            &attribute_keys,
            &pitch,
            state.spatial_map_mut(),
            pass_phase.reception_point,
            is_home_offense,
            offense_team_id,
            defense_team_id,
            drives_in_series,
            advanced_mirins,
            is_last_down,
            is_bonus_phase,
            &context,
            &fatigue_lookup_exec,
            &mut execution_rng,
        )?;

        (chosen_decision, execution_outcome)
    } else {
        (
            ArtrineDecisionKind::SelfCarry,
            ArtrineExecutionOutcome {
                mirins_advanced: 0.0,
                drives_recorded: 0,
                drive_row_indices: Vec::new(),
                turnover: None,
                recovering_player_id: None,
                scoring_decision: ScoringDecision::NoOpportunity,
                duration_ledger: DurationLedger::new(),
                end_position: pass_phase.scrimmage_point,
                duels: Vec::new(),
                receiver_id: None,
                distribution_flight: None,
            },
        )
    };

    let detailed_outcome = apply_play_transition(
        state,
        pass_phase,
        chosen_decision,
        execution_outcome,
        offense_team_id,
        defense_team_id,
        sink,
    );

    Ok(detailed_outcome)
}
