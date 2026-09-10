use crate::artrine::logistics::{
    collect_drifted_defender_candidates, collect_helper_candidates, collect_swept_participant_ids,
    resolve_primary_lead_defender,
};
use crate::artrine::reception_phase::rac_block::RacBlockResult;
use crate::artrine::reception_phase::rac_context::RacContext;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_side_rating, RatingParticipants,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use arlo_math::units::{Duration, Speed, Velocity};
use rand::Rng;
use uuid::Uuid;

pub struct RacBreakthroughResult {
    pub won: bool,
    pub additional_advance: f64,
    pub duel: AttributedDuelOutcome,
    pub duration: Duration,
    pub rec_spd: Speed,
}

pub fn resolve_rac_breakthrough<F, R>(
    ctx: &RacContext<'_, F>,
    block_res: &RacBlockResult<'_>,
    rng: &mut R,
) -> RacBreakthroughResult
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let (rb_offense_profile, rb_defense_profile) = get_duel_profiles(DuelKind::RunBreakthrough);

    let attacker_rating = calculate_anchored_side_rating(
        ctx.receiver,
        ctx.receiver_pos_domain,
        RatingParticipants::from_slice_with_index(
            &block_res.blocker_subset,
            ctx.offense_position_index,
        )
        .with_fatigue(ctx.fatigue_for)
        .with_attribute_tables(ctx.attribute_tables),
        ctx.attribute_keys,
        rb_offense_profile,
    ) + block_res.block_bonus;

    let defender_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(ctx.defenders, ctx.defense_position_index)
            .with_fatigue(ctx.fatigue_for)
            .with_attribute_tables(ctx.attribute_tables),
        ctx.attribute_keys,
        rb_defense_profile,
    );

    let contest_radius = ctx.contest_radius();
    let lead_defender = resolve_primary_lead_defender(
        ctx.receiver.id(),
        ctx.offense_position_index,
        block_res.receiver_pos_vec,
        Velocity::zero(),
        ctx.defenders,
        ctx.spatial_map,
        ctx.defense_instructions_index,
        ctx.attribute_keys,
        ctx.fatigue_for,
        contest_radius,
        None,
        rng,
    );

    let receiver_state = ctx.fatigue(&ctx.receiver.id());
    let lead_def_state = ctx.fatigue(&lead_defender.id());

    let rb_context = ctx.duel_context.for_duel_kind(DuelKind::RunBreakthrough);
    let attacker_table = ctx.attribute_tables.get(&ctx.receiver.id());
    let defender_table = ctx.attribute_tables.get(&lead_defender.id());
    let req = DuelResolutionRequest::with_states(
        DuelKind::RunBreakthrough,
        attacker_rating,
        defender_rating,
        ctx.receiver,
        lead_defender,
        receiver_state,
        lead_def_state,
        ctx.attribute_keys,
        &rb_context,
    )
    .with_tables(attacker_table, defender_table);
    let raw_rb_duel = resolve_duel(req, rng);

    let rb_def_pos =
        get_drifted_defender_position(lead_defender, ctx.spatial_map, ctx.attribute_keys, rng)
            .unwrap_or(block_res.receiver_pos_vec);
    let rb_def_spd =
        calculate_effective_player_speed(lead_defender, ctx.attribute_keys, &lead_def_state);

    let rec_spd =
        calculate_effective_player_speed(ctx.receiver, ctx.attribute_keys, &receiver_state);
    let rb_duration =
        derive_duel_duration(block_res.receiver_pos_vec, rec_spd, rb_def_pos, rb_def_spd);

    let rb_helper_candidates = collect_helper_candidates(
        ctx.offense_helpers,
        ctx.spatial_map,
        ctx.attribute_keys,
        block_res.receiver_pos_vec,
        ctx.fatigue_for,
    );
    let rb_attacker_ids = collect_swept_participant_ids(
        ctx.receiver.id(),
        block_res.receiver_pos_vec,
        Velocity::zero(),
        &rb_helper_candidates,
        contest_radius,
        rb_duration,
    );

    let rb_defender_candidates = collect_drifted_defender_candidates(
        ctx.defenders,
        ctx.spatial_map,
        ctx.attribute_keys,
        block_res.receiver_pos_vec,
        ctx.fatigue_for,
        rng,
    );
    let rb_defender_ids = collect_swept_participant_ids(
        lead_defender.id(),
        block_res.receiver_pos_vec,
        Velocity::zero(),
        &rb_defender_candidates,
        contest_radius,
        rb_duration,
    );

    let rb_duel = AttributedDuelOutcome::new(raw_rb_duel, rb_attacker_ids, rb_defender_ids);

    let additional_advance = if raw_rb_duel.attacker_won() {
        let progression_strategy = AggregateProgressionStrategy::default();
        progression_strategy.resolve_progression(&raw_rb_duel, rng)
    } else {
        0.0
    };

    RacBreakthroughResult {
        won: raw_rb_duel.attacker_won(),
        additional_advance,
        duel: rb_duel,
        duration: rb_duration,
        rec_spd,
    }
}