use crate::artrine::execution::carry_targeting::{
    compute_carry_target_lane, compute_forward_target_pos, filter_blocker_helpers,
};
use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::artrine::execution::security::resolve_proximity_ball_security;
use crate::artrine::logistics::{
    collect_drifted_defender_candidates, collect_helper_candidates, collect_swept_participant_ids,
    resolve_primary_lead_defender,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_side_rating_from_index_with_fatigue,
};
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::decision_vector::derive_velocity_towards_target;
use crate::spatial::positioning_drift::nearest_drifted_opponent;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::sport_constants::MINIMUM_ENGAGEMENT_SECONDS;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, Speed};
use rand::Rng;
use uuid::Uuid;

pub struct PreparedCarryDuel<'a> {
    pub artro_duel: AttributedDuelOutcome,
    pub artro_duration: Duration,
    pub nearest_def_opt: Option<(&'a Player, VectorPosition)>,
    pub artrine_speed: Speed,
    pub target_channel_y_m: f64,
}

pub fn prepare_artro_duel<'a, F, R>(
    ctx: &ActionExecutionContext<'a, F>,
    artrine: &'a Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> PreparedCarryDuel<'a>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let blocker_helpers = filter_blocker_helpers(ctx.offense_helpers, ctx.offense_role_index);
    let (offense_profile, defense_profile) = get_duel_profiles(DuelKind::ArtroBreakthrough);

    let attacker_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        artrine,
        DomainPosition::Artrine,
        &blocker_helpers,
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

    let artrine_state = ctx.fatigue(&artrine.id());
    let artrine_speed =
        calculate_effective_player_speed(artrine, ctx.attribute_keys, &artrine_state);
    let target_channel_y_m = compute_carry_target_lane(start_pos, ctx.pitch);
    let target_carry_pos = compute_forward_target_pos(
        start_pos,
        target_channel_y_m,
        ctx.pitch,
        ctx.attacking_positive_x,
    );
    let carrier_vel = derive_velocity_towards_target(start_pos, target_carry_pos, artrine_speed);
    let contest_radius = ctx.contest_radius();

    let lead_defender = resolve_primary_lead_defender(
        artrine.id(),
        ctx.offense_position_index,
        start_pos,
        carrier_vel,
        ctx.defenders,
        spatial_map,
        ctx.defense_instructions_index,
        ctx.attribute_keys,
        ctx.fatigue_for,
        contest_radius,
        None,
        rng,
    );

    let artro_context = ctx.duel_context.for_duel_kind(DuelKind::ArtroBreakthrough);
    let raw_artro_duel = resolve_duel_with_fatigue(
        DuelKind::ArtroBreakthrough,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        &artrine_state,
        &ctx.fatigue(&lead_defender.id()),
        ctx.attribute_keys,
        &artro_context,
        rng,
    );

    let (artro_duration, nearest_def_opt) = match nearest_drifted_opponent(
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
    let artro_attacker_ids = collect_swept_participant_ids(
        artrine.id(),
        start_pos,
        carrier_vel,
        &helper_candidates,
        contest_radius,
        artro_duration,
    );

    let defender_candidates = collect_drifted_defender_candidates(
        ctx.defenders,
        spatial_map,
        ctx.attribute_keys,
        start_pos,
        ctx.fatigue_for,
        rng,
    );
    let artro_defender_ids = collect_swept_participant_ids(
        lead_defender.id(),
        start_pos,
        carrier_vel,
        &defender_candidates,
        contest_radius,
        artro_duration,
    );

    let artro_duel =
        AttributedDuelOutcome::new(raw_artro_duel, artro_attacker_ids, artro_defender_ids);

    PreparedCarryDuel {
        artro_duel,
        artro_duration,
        nearest_def_opt,
        artrine_speed,
        target_channel_y_m,
    }
}

pub fn process_failed_carry<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    prep: PreparedCarryDuel<'_>,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::ArtroBreakthroughEngagement,
        prep.artro_duration,
    );

    let security_result = resolve_proximity_ball_security(
        ctx,
        artrine,
        DomainPosition::Artrine,
        start_pos,
        prep.artrine_speed,
        prep.nearest_def_opt,
        spatial_map,
        DuelKind::BallSecurityCarry,
        &mut ledger,
        rng,
    );

    let mut duels = vec![prep.artro_duel];
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
