use crate::match_decision::target_selection::{
    calculate_player_target_weight_with_state, ReceptionRole,
};
use crate::physical::FatigueState;
use crate::playmaking::routes::simulate_route_development;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_decision_target_weights<F>(
    context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    state: &mut MatchState,
    target_candidates: &[&Player],
    defenders: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_lookup: &F,
) -> (f64, f64, HashMap<Uuid, f64>)
where
    F: Fn(&Uuid) -> FatigueState,
{
    if context.offense_route_index.is_empty() {
        let empty_openness = HashMap::new();
        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = fatigue_lookup(&p.id());
                calculate_player_target_weight_with_state(
                    p,
                    state.spatial_map(),
                    state.pitch(),
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    attribute_keys,
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &empty_openness,
                    &p_state,
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
        let openness_by_player = simulate_route_development(
            &pitch,
            context.is_home_offense,
            &offense_route_runners,
            &context.offense_route_index,
            &context.offense_pos_index,
            defenders,
            &context.defense_pos_index,
            &context.defense_instructions_index,
            attribute_keys,
            state.spatial_map_mut(),
            fatigue_lookup,
            available_duration,
            &mut drift_rng,
        );

        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = fatigue_lookup(&p.id());
                calculate_player_target_weight_with_state(
                    p,
                    state.spatial_map(),
                    state.pitch(),
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    attribute_keys,
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &openness_by_player,
                    &p_state,
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
