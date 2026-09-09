use crate::match_decision::finisher_selection::select_finisher_or_kicker;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt_with_fatigue,
};
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::resolution::DuelKind;
use crate::rng::RngStream;
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_cross_speed_with_state};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::time::DurationComponentKind;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{Player, Position as DomainPosition};
use uuid::Uuid;

pub fn find_defense_goalguard<'a>(defense_players: &[&'a Player]) -> &'a Player {
    defense_players
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
        })
        .unwrap_or(defense_players[0])
}

pub fn execute_cross_action<F>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    fatigue_lookup: &F,
) where
    F: Fn(&Uuid) -> FatigueState,
{
    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys().clone();
    let is_bonus_phase = state.possession().is_bonus_phase();
    let carrier_pos = loop_state.current_carrier_pos;

    let seq_fin = state.next_sequence();
    let mut fin_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::FinisherSelection, seq_fin);

    let chosen_finisher_id = select_finisher_or_kicker(
        &iter_ctx.target_candidates,
        &context.offense_role_index,
        is_bonus_phase,
        state.spatial_map(),
        &pitch,
        &context.offense_pos_index,
        &context.offense_instructions_index,
        &attribute_keys,
        context.is_home_offense,
        &iter_ctx.openness_by_player,
        fatigue_lookup,
        &mut fin_rng,
    );

    let finisher = chosen_finisher_id
        .and_then(|fid| iter_ctx.target_candidates.iter().copied().find(|p| p.id() == fid))
        .unwrap_or(current_carrier);

    let cross_speed = calculate_cross_speed_with_state(
        current_carrier,
        &attribute_keys,
        &fatigue_lookup(&current_carrier.id()),
    );
    let finisher_pos = state
        .spatial_map()
        .get_position(&finisher.id())
        .unwrap_or(carrier_pos);
    let cross_dist = calculate_distance_mirim(carrier_pos, finisher_pos);
    let cross_flight = ball_flight_duration(cross_dist, cross_speed);
    loop_state
        .accumulated_duration_ledger
        .record_live(DurationComponentKind::CrossFlight, cross_flight);

    let (att_prof, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let fin_rating = calculate_player_duel_rating_with_state(
        finisher,
        DomainPosition::CenterOffense,
        &attribute_keys,
        &att_prof,
        &fatigue_lookup(&finisher.id()),
    );

    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_advance = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;

    let opportunity = evaluate_scoring_opportunity(
        is_bonus_phase,
        total_drives,
        total_advance,
        fin_rating,
    );

    let goalguard = find_defense_goalguard(defense_players);
    let seq_duel = state.next_sequence();
    let mut duel_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq_duel);

    let (score_dec, fin_duel) = resolve_scoring_attempt_with_fatigue(
        finisher,
        goalguard,
        &attribute_keys,
        context.offense_team_id,
        pass_phase.artrine.id(),
        Some(current_carrier.id()),
        opportunity,
        total_drives,
        total_advance,
        &fatigue_lookup(&finisher.id()),
        &fatigue_lookup(&goalguard.id()),
        &iter_ctx.duel_context.for_duel_kind(DuelKind::FinishingAttempt),
        &mut duel_rng,
    );

    loop_state.accumulated_duels.push(fin_duel);
    loop_state.scoring_decision = score_dec;
    loop_state.last_receiver_id = Some(finisher.id());
    loop_state.ball_in_play = false;
}

pub fn execute_self_finish_action<F>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    fatigue_lookup: &F,
) where
    F: Fn(&Uuid) -> FatigueState,
{
    let attribute_keys = state.attribute_keys().clone();
    let is_bonus_phase = state.possession().is_bonus_phase();

    let (att_prof, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let fin_rating = calculate_player_duel_rating_with_state(
        current_carrier,
        DomainPosition::CenterOffense,
        &attribute_keys,
        &att_prof,
        &fatigue_lookup(&current_carrier.id()),
    );

    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_advance = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;

    let opportunity = evaluate_scoring_opportunity(
        is_bonus_phase,
        total_drives,
        total_advance,
        fin_rating,
    );

    let goalguard = find_defense_goalguard(defense_players);
    let seq_duel = state.next_sequence();
    let mut duel_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq_duel);

    let (score_dec, fin_duel) = resolve_scoring_attempt_with_fatigue(
        current_carrier,
        goalguard,
        &attribute_keys,
        context.offense_team_id,
        pass_phase.artrine.id(),
        None,
        opportunity,
        total_drives,
        total_advance,
        &fatigue_lookup(&current_carrier.id()),
        &fatigue_lookup(&goalguard.id()),
        &iter_ctx.duel_context.for_duel_kind(DuelKind::FinishingAttempt),
        &mut duel_rng,
    );

    loop_state.accumulated_duels.push(fin_duel);
    loop_state.scoring_decision = score_dec;
    loop_state.last_receiver_id = Some(current_carrier.id());
    loop_state.ball_in_play = false;
}