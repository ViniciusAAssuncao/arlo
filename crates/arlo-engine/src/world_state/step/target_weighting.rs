use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::match_decision::target_selection::{
    calculate_player_target_weight_from_table, ReceptionRole,
};
use crate::playmaking::routes::simulate_route_development_from_tables;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_decision_target_weights(
    context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    state: &mut MatchState,
    target_candidates: &[&Player],
    defenders: &[&Player],
    _attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> (f64, f64, HashMap<Uuid, f64>) {
    let tables = state.teams.player_attribute_tables().clone();

    if context.offense_route_index.is_empty() {
        let empty_openness = HashMap::new();
        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = state.fatigue_lookup().get(&p.id());
                let table = tables
                    .get(&p.id())
                    .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
                calculate_player_target_weight_from_table(
                    p,
                    table,
                    state.spatial_map(),
                    state.pitch(),
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &empty_openness,
                    Some(&p_state),
                )
            })
            .fold(0.0_f64, f64::max);

        (
            best_available_target_weight,
            best_available_target_weight,
            empty_openness,
        )
    } else {
        let available_duration = pass_phase.duration_ledger.total_live();
        let seq = state.next_sequence();
        let mut drift_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::PositionalDrift, seq);

        let offense_route_runners: Vec<&Player> = target_candidates
            .iter()
            .copied()
            .filter(|p| context.offense_route_index.contains_key(&p.id()))
            .collect();

        let pitch = *state.pitch();
        let fatigue_tracker = state.fatigue.clone();
        let openness_by_player = simulate_route_development_from_tables(
            &pitch,
            context.is_home_offense,
            &offense_route_runners,
            &context.offense_route_index,
            &context.offense_pos_index,
            defenders,
            &context.defense_pos_index,
            &context.defense_instructions_index,
            &tables,
            &mut state.spatial_map,
            &|id| fatigue_tracker.fatigue_for(id),
            available_duration,
            &mut drift_rng,
        );

        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = state.fatigue_lookup().get(&p.id());
                let table = tables
                    .get(&p.id())
                    .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
                calculate_player_target_weight_from_table(
                    p,
                    table,
                    state.spatial_map(),
                    state.pitch(),
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &openness_by_player,
                    Some(&p_state),
                )
            })
            .fold(0.0_f64, f64::max);

        (
            best_available_target_weight,
            best_available_target_weight,
            openness_by_player,
        )
    }
}
