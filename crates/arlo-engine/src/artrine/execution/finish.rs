use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::distribution::execute_distribution;
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::match_decision::finisher_selection::select_finisher_or_kicker;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt_with_fatigue, ScoringDecision,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::resolution::DuelKind;
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed_with_state, calculate_shot_speed_with_state,
};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

fn calculate_shot_kinematics<F>(
    ctx: &ActionExecutionContext<'_, F>,
    finisher: &Player,
    finisher_pos: VectorPosition,
    spatial_map: &DynamicSpatialMap,
) -> (Duration, Duration)
where
    F: Fn(&Uuid) -> FatigueState,
{
    let goalguard_pos = spatial_map
        .get_position(&ctx.goalguard.id())
        .unwrap_or(finisher_pos);

    let finisher_state = ctx.fatigue(&finisher.id());
    let goalguard_state = ctx.fatigue(&ctx.goalguard.id());

    let finisher_speed =
        calculate_effective_player_speed(finisher, ctx.attribute_keys, &finisher_state);
    let goalguard_speed =
        calculate_effective_player_speed(ctx.goalguard, ctx.attribute_keys, &goalguard_state);
    let finishing_duration =
        derive_duel_duration(finisher_pos, finisher_speed, goalguard_pos, goalguard_speed);

    let shot_speed = calculate_shot_speed_with_state(finisher, ctx.attribute_keys, &finisher_state);
    let finisher_x_mirim = finisher_pos.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if ctx.attacking_positive_x {
        (ctx.pitch.length_mirim() - finisher_x_mirim).max(0.0)
    } else {
        finisher_x_mirim.max(0.0)
    };
    let shot_flight = ball_flight_duration(dist_to_goal_mirim, shot_speed);

    (finishing_duration, shot_flight)
}

pub fn execute_self_finish<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let finisher_pos = spatial_map.get_position(&artrine.id()).unwrap_or(start_pos);
    let (finishing_duration, shot_flight) =
        calculate_shot_kinematics(ctx, artrine, finisher_pos, spatial_map);

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::FinishingEngagement,
        finishing_duration,
    );
    ledger.record_live(DurationComponentKind::ShotFlight, shot_flight);

    execute_finishing_with_player(
        ctx,
        artrine,
        artrine,
        start_pos,
        ledger,
        ctx.accumulated_advance_mirim,
        None,
        rng,
    )
}

pub fn execute_cross_finish<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    additional_advance: f64,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let chosen_finisher_id = select_finisher_or_kicker(
        ctx.offense_helpers,
        ctx.offense_role_index,
        ctx.is_bonus_phase,
        spatial_map,
        ctx.pitch,
        ctx.offense_position_index,
        ctx.offense_instructions_index,
        ctx.attribute_keys,
        ctx.attacking_positive_x,
        ctx.openness_by_player,
        ctx.fatigue_for,
        rng,
    );

    let finisher = chosen_finisher_id
        .and_then(|fid| ctx.offense_helpers.iter().copied().find(|p| p.id() == fid))
        .unwrap_or(artrine);

    let artrine_state = ctx.fatigue(&artrine.id());
    let artrine_pos = spatial_map.get_position(&artrine.id()).unwrap_or(start_pos);
    let finisher_pos = spatial_map
        .get_position(&finisher.id())
        .unwrap_or(start_pos);

    let cross_dist_mirim = calculate_distance_mirim(artrine_pos, finisher_pos);
    let cross_speed = calculate_cross_speed_with_state(artrine, ctx.attribute_keys, &artrine_state);
    let cross_flight = ball_flight_duration(cross_dist_mirim, cross_speed);

    let (finishing_duration, shot_flight) =
        calculate_shot_kinematics(ctx, finisher, finisher_pos, spatial_map);

    let mut ledger = DurationLedger::new();
    ledger.record_live(DurationComponentKind::CrossFlight, cross_flight);
    ledger.record_live(
        DurationComponentKind::FinishingEngagement,
        finishing_duration,
    );
    ledger.record_live(DurationComponentKind::ShotFlight, shot_flight);

    let total_advance = ctx.accumulated_advance_mirim + additional_advance;
    let assister_id = if finisher.id() != artrine.id() {
        Some(artrine.id())
    } else {
        None
    };

    execute_finishing_with_player(
        ctx,
        finisher,
        artrine,
        start_pos,
        ledger,
        total_advance,
        assister_id,
        rng,
    )
}

pub fn execute_cross_pipeline<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let dist_outcome = execute_distribution(
        ctx,
        ArtrineDecisionKind::Cross,
        artrine,
        spatial_map,
        start_pos,
        rng,
    );

    if dist_outcome.turnover.is_some() || dist_outcome.scoring_decision.is_scored() {
        return dist_outcome;
    }

    let finish_outcome = execute_cross_finish(
        ctx,
        artrine,
        spatial_map,
        dist_outcome.end_position,
        dist_outcome.mirins_advanced,
        rng,
    );

    let mut combined_duels = dist_outcome.duels;
    combined_duels.extend(finish_outcome.duels);

    let mut combined_ledger = dist_outcome.duration_ledger;
    combined_ledger.merge(finish_outcome.duration_ledger);

    let mut combined_trajectories = dist_outcome.kinematic_trajectories;
    combined_trajectories.extend(finish_outcome.kinematic_trajectories);

    ArtrineExecutionOutcome {
        mirins_advanced: dist_outcome.mirins_advanced,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover: finish_outcome.turnover,
        recovering_player_id: finish_outcome.recovering_player_id,
        scoring_decision: finish_outcome.scoring_decision,
        duration_ledger: combined_ledger,
        end_position: dist_outcome.end_position,
        duels: combined_duels,
        receiver_id: dist_outcome.receiver_id,
        distribution_flight: dist_outcome.distribution_flight,
        kinematic_trajectories: combined_trajectories,
    }
}

pub fn execute_finishing_with_player<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    finisher: &Player,
    artrine: &Player,
    start_pos: VectorPosition,
    duration_ledger: DurationLedger,
    total_advance_mirim: f64,
    assister_id: Option<Uuid>,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let finisher_state = ctx.fatigue(&finisher.id());
    let goalguard_state = ctx.fatigue(&ctx.goalguard.id());

    let (attacker_profile, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let finisher_rating = calculate_player_duel_rating_with_state(
        finisher,
        DomainPosition::CenterOffense,
        ctx.attribute_keys,
        &attacker_profile,
        &finisher_state,
    );

    let opportunity = evaluate_scoring_opportunity(
        ctx.is_bonus_phase,
        ctx.drives_in_series,
        total_advance_mirim,
        finisher_rating,
    );

    let finish_context = ctx.duel_context.for_duel_kind(DuelKind::FinishingAttempt);
    let (scoring_decision, finish_duel) = resolve_scoring_attempt_with_fatigue(
        finisher,
        ctx.goalguard,
        ctx.attribute_keys,
        ctx.offense_team_id,
        artrine.id(),
        assister_id,
        opportunity,
        ctx.drives_in_series,
        total_advance_mirim,
        &finisher_state,
        &goalguard_state,
        &finish_context,
        rng,
    );

    let (turnover, recovering_player_id) = match &scoring_decision {
        ScoringDecision::Missed { .. } => (Some(ctx.defense_team_id), None),
        _ => (None, None),
    };

    ArtrineExecutionOutcome {
        mirins_advanced: 0.0,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover,
        recovering_player_id,
        scoring_decision,
        duration_ledger,
        end_position: start_pos,
        duels: vec![finish_duel],
        receiver_id: Some(finisher.id()),
        distribution_flight: None,
        kinematic_trajectories: HashMap::new(),
    }
}