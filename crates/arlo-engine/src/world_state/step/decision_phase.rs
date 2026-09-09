use crate::ai::cognitive::RiskProfile;
use crate::ai::gravity::calculate_team_max_finishing_gravity_with_fatigue;
use crate::artrine::{
    calculate_normalized_proximity, execute_artrine_decision,
    resolve_artrine_decision_with_context_and_impulse, translate_artrine_decision_made,
    ArtrineExecutionOutcome,
};
use crate::error::EngineResult;
use crate::match_decision::event_translation::create_envelope;
use crate::match_decision::scoring::ScoringDecision;
use crate::match_decision::target_selection::{
    calculate_player_target_weight_with_state, ReceptionRole,
};
use crate::playmaking::resolve_misdirection_logit_offset;
use crate::resolution::DuelContext;
use crate::rng::RngStream;
use crate::spatial::{calculate_artro_advance_pitch_control, find_next_artro_position};
use crate::time::DurationLedger;
use crate::world_state::context_analyzer::analyze_match_state;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use crate::world_state::step::target_weighting::resolve_decision_target_weights;
use arlo_domain::sport_constants::LAUNCHER_TARGET_WEIGHT_MULTIPLIER;
use arlo_domain::{ArtrineDecisionKind, Player, SlotRole};
use arlo_events::EventSink;
use std::collections::HashMap;
use uuid::Uuid;

pub struct DecisionPhaseResult {
    pub chosen_decision: ArtrineDecisionKind,
    pub execution_outcome: ArtrineExecutionOutcome,
}

pub fn run_decision_phase(
    state: &mut MatchState,
    context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    offense_players: &[&Player],
    defense_players: &[&Player],
    sink: &mut impl EventSink,
) -> EngineResult<DecisionPhaseResult> {
    if !pass_phase.pass_completed {
        return Ok(DecisionPhaseResult {
            chosen_decision: ArtrineDecisionKind::SelfCarry,
            execution_outcome: ArtrineExecutionOutcome {
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
                kinematic_trajectories: HashMap::new(),
            },
        });
    }

    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys().clone();
    let is_bonus_phase = state.possession().is_bonus_phase();
    let drives_in_series = state.drives_in_current_series();
    let remaining_downs = state.possession().series_state().remaining_downs();
    let is_last_down = state.possession().series_state().is_last_down();
    let advanced_mirins = state.possession().series_state().advanced_mirins();

    let normalized_proximity =
        calculate_normalized_proximity(pass_phase.reception_point, &pitch, context.is_home_offense);

    let target_candidates: Vec<&Player> = offense_players
        .iter()
        .copied()
        .filter(|p| p.id() != pass_phase.artrine.id())
        .collect();

    let home_fatigue = state.home_fatigue().clone();
    let away_fatigue = state.away_fatigue().clone();
    let fatigue_lookup = move |id: &Uuid| {
        home_fatigue
            .get(id)
            .or_else(|| away_fatigue.get(id))
            .copied()
            .unwrap_or_default()
    };

    let artrine_fatigue = fatigue_lookup(&pass_phase.artrine.id());
    let artrine_impulse = state.impulse_for(&pass_phase.artrine.id());

    let (best_available_target_weight, _long_launch_target_weight, openness_by_player) =
        resolve_decision_target_weights(
            context,
            pass_phase,
            state,
            &target_candidates,
            defense_players,
            &attribute_keys,
            &fatigue_lookup,
        );

    let long_launch_target_weight = target_candidates
        .iter()
        .map(|p| {
            let p_state = fatigue_lookup(&p.id());
            let base_weight = calculate_player_target_weight_with_state(
                p,
                state.spatial_map(),
                &pitch,
                &context.offense_pos_index,
                &context.offense_instructions_index,
                &attribute_keys,
                context.is_home_offense,
                ReceptionRole::OpenPlayReceiver,
                &openness_by_player,
                &p_state,
            );
            if context.offense_role_index.get(&p.id()) == Some(&SlotRole::Launcher) {
                base_weight * LAUNCHER_TARGET_WEIGHT_MULTIPLIER
            } else {
                base_weight
            }
        })
        .fold(0.0_f64, f64::max);

    let offensive_gravity = calculate_team_max_finishing_gravity_with_fatigue(
        &target_candidates,
        &context.offense_pos_index,
        state.spatial_map(),
        &pitch,
        &attribute_keys,
        context.is_home_offense,
        &fatigue_lookup,
    );

    let next_artro_pos =
        find_next_artro_position(pass_phase.reception_point, &pitch, context.is_home_offense);

    let pitch_control_ahead = calculate_artro_advance_pitch_control(
        pass_phase.artrine,
        &target_candidates,
        defense_players,
        state.spatial_map(),
        &attribute_keys,
        &fatigue_lookup,
        pass_phase.reception_point,
        next_artro_pos,
        &pitch,
        &context.offense_role_index,
    );

    let game_state_pressure = analyze_match_state(state);
    let risk_profile = RiskProfile::from_player_with_impulse(
        pass_phase.artrine,
        &attribute_keys,
        &artrine_fatigue,
        &artrine_impulse,
    );

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();

    let seq_decision = state.next_sequence();
    let mut decision_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::ArtrineDecision, seq_decision);

    let decision_result = resolve_artrine_decision_with_context_and_impulse(
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
        long_launch_target_weight,
        pass_phase.reception_point,
        next_artro_pos,
        pitch_control_ahead,
        pitch.length_mirim(),
        offensive_gravity.multiplier(),
        passing_range,
        risk_profile,
        game_state_pressure,
        context.decision_emphasis,
        &artrine_fatigue,
        &artrine_impulse,
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

    let offense_tempo_value = offense_instructions.in_possession().tempo().value();
    let offense_physicality = offense_instructions.in_possession().physicality();
    let physicality_offset =
        crate::team_identity::physicality::offensive_contact_logit_offset(offense_physicality);
    let defense_instructions = *state.instructions_for_team(context.defense_team_id);
    let defense_pressing_multiplier = crate::team_identity::pressing::contest_radius_multiplier(
        defense_instructions
            .out_of_possession()
            .pressing_intensity(),
    );
    let defense_aggression = defense_instructions.out_of_possession().aggression();
    let aggression_offset = crate::team_identity::aggression::duel_logit_offset(defense_aggression);
    let misdirection_offset = resolve_misdirection_logit_offset(
        context.active_play_call.as_ref(),
        &context.offense_route_index,
        &context.offense_lineup,
    );
    let duel_context = DuelContext::with_offsets(
        context.is_home_offense,
        !context.is_home_offense,
        aggression_offset,
        misdirection_offset,
        physicality_offset,
    );

    let seq_execution = state.next_sequence();
    let mut execution_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq_execution);

    let execution_outcome = execute_artrine_decision(
        chosen_decision,
        pass_phase.artrine,
        offense_players,
        &context.offense_pos_index,
        &context.offense_role_index,
        &context.offense_instructions_index,
        &offense_instructions,
        defense_players,
        &context.defense_pos_index,
        &context.defense_instructions_index,
        &attribute_keys,
        &pitch,
        state.spatial_map_mut(),
        pass_phase.reception_point,
        context.is_home_offense,
        context.offense_team_id,
        context.defense_team_id,
        drives_in_series,
        advanced_mirins,
        is_last_down,
        is_bonus_phase,
        &duel_context,
        &fatigue_lookup,
        defense_pressing_multiplier,
        offense_tempo_value,
        &openness_by_player,
        &context.offense_route_index,
        &mut execution_rng,
    )?;

    Ok(DecisionPhaseResult {
        chosen_decision,
        execution_outcome,
    })
}
