use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::artrine::execution::security::resolve_proximity_ball_security;
use crate::artrine::logistics::{
    collect_drifted_defender_candidates, collect_helper_candidates, collect_swept_participant_ids,
    resolve_primary_lead_defender,
};
use crate::artrine::reception_phase::distribution_reception::execute_post_throw_reception;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_side_rating_from_index_with_fatigue,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed_with_state, calculate_pass_speed_with_state,
};
use crate::spatial::positioning_drift::nearest_drifted_opponent;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::sport_constants::MINIMUM_ENGAGEMENT_SECONDS;
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, Speed, Velocity};
use rand::Rng;
use uuid::Uuid;

struct PreparedDistributionDuel<'a> {
    dist_duel: AttributedDuelOutcome,
    dist_duration: Duration,
    nearest_def_opt: Option<(&'a Player, VectorPosition)>,
    artrine_speed: Speed,
}

fn determine_distribution_duel_kind(decision_kind: ArtrineDecisionKind) -> DuelKind {
    match decision_kind {
        ArtrineDecisionKind::ShortPass => DuelKind::ShortDistribution,
        ArtrineDecisionKind::LongLaunch => DuelKind::LongDistribution,
        ArtrineDecisionKind::Cross => DuelKind::CrossDistribution,
        _ => DuelKind::ShortDistribution,
    }
}

fn prepare_distribution_duel<'a, F, R>(
    ctx: &ActionExecutionContext<'a, F>,
    decision_kind: ArtrineDecisionKind,
    artrine: &'a Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> PreparedDistributionDuel<'a>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let duel_kind = determine_distribution_duel_kind(decision_kind);
    let (offense_profile, defense_profile) = get_duel_profiles(duel_kind);

    let attacker_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        artrine,
        DomainPosition::Artrine,
        ctx.offense_helpers,
        ctx.offense_position_index,
        ctx.attribute_keys,
        &offense_profile,
        ctx.fatigue_for,
    );

    let defender_rating = calculate_side_rating_from_index_with_fatigue(
        ctx.defenders,
        ctx.defense_position_index,
        ctx.attribute_keys,
        &defense_profile,
        ctx.fatigue_for,
    );

    let contest_radius = ctx.contest_radius();
    let lead_defender = resolve_primary_lead_defender(
        artrine.id(),
        ctx.offense_position_index,
        start_pos,
        Velocity::zero(),
        ctx.defenders,
        spatial_map,
        ctx.defense_instructions_index,
        ctx.attribute_keys,
        ctx.fatigue_for,
        contest_radius,
        None,
        rng,
    );

    let artrine_state = ctx.fatigue(&artrine.id());
    let lead_def_state = ctx.fatigue(&lead_defender.id());

    let dist_context = ctx.duel_context.for_duel_kind(duel_kind);
    let raw_dist_duel = resolve_duel_with_fatigue(
        duel_kind,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        &artrine_state,
        &lead_def_state,
        ctx.attribute_keys,
        &dist_context,
        rng,
    );

    let artrine_speed =
        calculate_effective_player_speed(artrine, ctx.attribute_keys, &artrine_state);

    let (dist_duration, nearest_def_opt) = match nearest_drifted_opponent(
        start_pos,
        ctx.defenders,
        spatial_map,
        ctx.attribute_keys,
        rng,
    ) {
        Some((d, pos)) => {
            let d_state = ctx.fatigue(&d.id());
            let d_spd = calculate_effective_player_speed(d, ctx.attribute_keys, &d_state);
            (
                derive_duel_duration(start_pos, artrine_speed, pos, d_spd),
                Some((d, pos)),
            )
        }
        None => (Duration::new(MINIMUM_ENGAGEMENT_SECONDS), None),
    };

    let helper_candidates = collect_helper_candidates(
        ctx.offense_helpers,
        spatial_map,
        ctx.attribute_keys,
        start_pos,
        ctx.fatigue_for,
    );
    let dist_attacker_ids = collect_swept_participant_ids(
        artrine.id(),
        start_pos,
        Velocity::zero(),
        &helper_candidates,
        contest_radius,
        dist_duration,
    );

    let defender_candidates = collect_drifted_defender_candidates(
        ctx.defenders,
        spatial_map,
        ctx.attribute_keys,
        start_pos,
        ctx.fatigue_for,
        rng,
    );
    let dist_defender_ids = collect_swept_participant_ids(
        lead_defender.id(),
        start_pos,
        Velocity::zero(),
        &defender_candidates,
        contest_radius,
        dist_duration,
    );

    let dist_duel = AttributedDuelOutcome::new(raw_dist_duel, dist_attacker_ids, dist_defender_ids);

    PreparedDistributionDuel {
        dist_duel,
        dist_duration,
        nearest_def_opt,
        artrine_speed,
    }
}

fn process_failed_distribution<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    prep: PreparedDistributionDuel<'_>,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::DistributionEngagement,
        prep.dist_duration,
    );

    let security_result = resolve_proximity_ball_security(
        ctx,
        artrine,
        DomainPosition::Artrine,
        start_pos,
        prep.artrine_speed,
        prep.nearest_def_opt,
        spatial_map,
        DuelKind::BallSecurityDistribution,
        &mut ledger,
        rng,
    );

    let mut duels = vec![prep.dist_duel];
    if let Some(sec_duel) = security_result.duel_outcome {
        duels.push(sec_duel);
    }

    ArtrineExecutionOutcome::stopped(
        start_pos,
        duels,
        ledger,
        security_result.turnover_team_id,
        security_result.recovering_player_id,
    )
}

fn calculate_distribution_flight<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    dist_duel: &AttributedDuelOutcome,
    rng: &mut R,
) -> (f64, Duration)
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let progression_strategy = AggregateProgressionStrategy::default();
    let throw_advance = progression_strategy.resolve_progression(dist_duel.outcome(), rng);
    let artrine_state = ctx.fatigue(&artrine.id());

    let ball_speed = match decision_kind {
        ArtrineDecisionKind::Cross => {
            calculate_cross_speed_with_state(artrine, ctx.attribute_keys, &artrine_state)
        }
        _ => calculate_pass_speed_with_state(artrine, ctx.attribute_keys, &artrine_state),
    };
    let flight_duration = ball_flight_duration(throw_advance, ball_speed);
    (throw_advance, flight_duration)
}

pub fn execute_distribution<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let prep = prepare_distribution_duel(ctx, decision_kind, artrine, spatial_map, start_pos, rng);
    if !prep.dist_duel.outcome().attacker_won() {
        return process_failed_distribution(ctx, artrine, spatial_map, start_pos, prep, rng);
    }
    let (throw_advance, flight_duration) =
        calculate_distribution_flight(ctx, decision_kind, artrine, &prep.dist_duel, rng);

    execute_post_throw_reception(
        ctx,
        decision_kind,
        artrine,
        spatial_map,
        start_pos,
        throw_advance,
        flight_duration,
        prep.dist_duration,
        prep.dist_duel,
        rng,
    )
}
