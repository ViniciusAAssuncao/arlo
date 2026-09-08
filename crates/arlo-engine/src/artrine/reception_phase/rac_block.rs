use crate::artrine::constants::{
    BLOCK_BONUS_MIN, BLOCK_BONUS_MULTIPLIER, CENTRAL_ZONE_NORMALIZED_Y_FALLBACK,
    CENTRAL_ZONE_NORMALIZED_Y_MAX, CENTRAL_ZONE_NORMALIZED_Y_MIN,
};
use crate::artrine::logistics::{
    collect_drifted_defender_candidates, collect_helper_candidates, collect_swept_participant_ids,
    resolve_primary_lead_defender,
};
use crate::artrine::reception_phase::rac_context::RacContext;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_player_duel_rating_with_state, calculate_side_rating_from_index_with_fatigue,
    identify_lead_player_from_index,
};
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use arlo_domain::Player;
use arlo_math::units::{Duration, Position as VectorPosition, Velocity};
use rand::Rng;
use uuid::Uuid;

pub struct RacBlockResult<'a> {
    pub won: bool,
    pub duel: AttributedDuelOutcome,
    pub duration: Duration,
    pub block_bonus: f64,
    pub blocker_subset: Vec<&'a Player>,
    pub receiver_pos_vec: VectorPosition,
}

pub fn resolve_rac_block<'a, F, R>(ctx: &'a RacContext<'a, F>, rng: &mut R) -> RacBlockResult<'a>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let blocker_subset = ctx.blocker_helpers();
    let receiver_pos_vec = ctx.receiver_pos_vec();
    let total_width = ctx.pitch.width().value();
    let normalized_y = if total_width > 0.0 {
        (receiver_pos_vec.raw().1 / total_width).clamp(0.0, 1.0)
    } else {
        CENTRAL_ZONE_NORMALIZED_Y_FALLBACK
    };

    let is_central =
        (CENTRAL_ZONE_NORMALIZED_Y_MIN..=CENTRAL_ZONE_NORMALIZED_Y_MAX).contains(&normalized_y);
    let block_duel_kind = if is_central {
        DuelKind::CentralBlock
    } else {
        DuelKind::LateralBlock
    };

    let (block_offense_profile, block_defense_profile) = get_duel_profiles(block_duel_kind);

    let lead_blocker = if !blocker_subset.is_empty() {
        identify_lead_player_from_index(
            &blocker_subset,
            ctx.offense_position_index,
            ctx.attribute_keys,
            &block_offense_profile,
        )
        .unwrap_or(blocker_subset[0])
    } else {
        ctx.receiver
    };

    let blocker_rating = if !blocker_subset.is_empty() {
        calculate_side_rating_from_index_with_fatigue(
            &blocker_subset,
            ctx.offense_position_index,
            ctx.attribute_keys,
            &block_offense_profile,
            ctx.fatigue_for,
        )
    } else {
        let receiver_state = ctx.fatigue(&ctx.receiver.id());
        calculate_player_duel_rating_with_state(
            ctx.receiver,
            ctx.receiver_pos_domain,
            ctx.attribute_keys,
            &block_offense_profile,
            &receiver_state,
        )
    };

    let defender_block_rating = calculate_side_rating_from_index_with_fatigue(
        ctx.defenders,
        ctx.defense_position_index,
        ctx.attribute_keys,
        &block_defense_profile,
        ctx.fatigue_for,
    );

    let contest_radius = ctx.contest_radius();
    let lead_block_defender = resolve_primary_lead_defender(
        ctx.receiver.id(),
        ctx.offense_position_index,
        receiver_pos_vec,
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

    let lead_blocker_state = ctx.fatigue(&lead_blocker.id());
    let lead_block_def_state = ctx.fatigue(&lead_block_defender.id());

    let block_context = ctx.duel_context.for_duel_kind(block_duel_kind);
    let raw_block_duel = resolve_duel_with_fatigue(
        block_duel_kind,
        blocker_rating,
        defender_block_rating,
        lead_blocker,
        lead_block_defender,
        &lead_blocker_state,
        &lead_block_def_state,
        ctx.attribute_keys,
        &block_context,
        rng,
    );

    let blocker_pos = ctx
        .spatial_map
        .get_position(&lead_blocker.id())
        .unwrap_or(receiver_pos_vec);
    let blocker_spd =
        calculate_effective_player_speed(lead_blocker, ctx.attribute_keys, &lead_blocker_state);

    let block_def_pos = get_drifted_defender_position(
        lead_block_defender,
        ctx.spatial_map,
        ctx.attribute_keys,
        rng,
    )
    .unwrap_or(receiver_pos_vec);
    let block_def_spd = calculate_effective_player_speed(
        lead_block_defender,
        ctx.attribute_keys,
        &lead_block_def_state,
    );
    let block_duration =
        derive_duel_duration(blocker_pos, blocker_spd, block_def_pos, block_def_spd);

    let block_helper_candidates = collect_helper_candidates(
        ctx.offense_helpers,
        ctx.spatial_map,
        ctx.attribute_keys,
        blocker_pos,
        ctx.fatigue_for,
    );
    let block_attacker_ids = collect_swept_participant_ids(
        lead_blocker.id(),
        blocker_pos,
        Velocity::zero(),
        &block_helper_candidates,
        contest_radius,
        block_duration,
    );

    let block_defender_candidates = collect_drifted_defender_candidates(
        ctx.defenders,
        ctx.spatial_map,
        ctx.attribute_keys,
        blocker_pos,
        ctx.fatigue_for,
        rng,
    );
    let block_defender_ids = collect_swept_participant_ids(
        lead_block_defender.id(),
        blocker_pos,
        Velocity::zero(),
        &block_defender_candidates,
        contest_radius,
        block_duration,
    );

    let block_duel =
        AttributedDuelOutcome::new(raw_block_duel, block_attacker_ids, block_defender_ids);
    let block_bonus =
        (raw_block_duel.net_advantage() * BLOCK_BONUS_MULTIPLIER).max(BLOCK_BONUS_MIN);

    RacBlockResult {
        won: raw_block_duel.attacker_won(),
        duel: block_duel,
        duration: block_duration,
        block_bonus,
        blocker_subset,
        receiver_pos_vec,
    }
}
