use crate::artrine::constants::{
    BLOCK_BONUS_MIN, BLOCK_BONUS_MULTIPLIER, CENTRAL_ZONE_NORMALIZED_Y_FALLBACK,
    CENTRAL_ZONE_NORMALIZED_Y_MAX, CENTRAL_ZONE_NORMALIZED_Y_MIN,
};
use crate::artrine::execution::resolve_ball_security;
use crate::artrine::logistics::{
    collect_drifted_defender_candidates, collect_helper_candidates, collect_swept_participant_ids,
    resolve_primary_lead_defender,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_player_duel_rating_with_state, calculate_side_rating_from_index_with_fatigue,
    identify_lead_player_from_index,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition, SlotRole};
use arlo_math::units::{
    Duration, Length, Position as VectorPosition, Velocity, MIRIM_TO_METERS,
};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct RunAfterCatchOutcome {
    pub additional_mirins_advanced: f64,
    pub duels: Vec<AttributedDuelOutcome>,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duration_ledger: DurationLedger,
}

pub fn resolve_run_after_catch<F, R>(
    receiver: &Player,
    receiver_pos_domain: DomainPosition,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    defense_team_id: Uuid,
    context: &DuelContext,
    fatigue_for: &F,
    defense_pressing_multiplier: f64,
    rng: &mut R,
) -> RunAfterCatchOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let blocker_subset: Vec<&Player> = {
        let blockers: Vec<&Player> = offense_helpers
            .iter()
            .copied()
            .filter(|p| offense_role_index.get(&p.id()) == Some(&SlotRole::Blocker))
            .collect();
        if blockers.is_empty() {
            offense_helpers.to_vec()
        } else {
            blockers
        }
    };

    let receiver_pos_vec = spatial_map
        .get_position(&receiver.id())
        .unwrap_or_else(VectorPosition::zero);

    let total_width = pitch.width().value();
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
            offense_position_index,
            attribute_keys,
            &block_offense_profile,
        )
        .unwrap_or(blocker_subset[0])
    } else {
        receiver
    };

    let blocker_rating = if !blocker_subset.is_empty() {
        calculate_side_rating_from_index_with_fatigue(
            &blocker_subset,
            offense_position_index,
            attribute_keys,
            &block_offense_profile,
            fatigue_for,
        )
    } else {
        let receiver_state = fatigue_for(&receiver.id());
        calculate_player_duel_rating_with_state(
            receiver,
            receiver_pos_domain,
            attribute_keys,
            &block_offense_profile,
            &receiver_state,
        )
    };

    let defender_block_rating = calculate_side_rating_from_index_with_fatigue(
        defenders,
        defense_position_index,
        attribute_keys,
        &block_defense_profile,
        fatigue_for,
    );

    let contest_radius = Length::new(
        PROXIMITY_CONTEST_RADIUS_MIRIM * defense_pressing_multiplier * MIRIM_TO_METERS,
    );
    let lead_block_defender = resolve_primary_lead_defender(
        receiver.id(),
        offense_position_index,
        receiver_pos_vec,
        Velocity::zero(),
        defenders,
        spatial_map,
        defense_instructions_index,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    );

    let lead_blocker_state = fatigue_for(&lead_blocker.id());
    let lead_block_def_state = fatigue_for(&lead_block_defender.id());

    let block_context = context.for_duel_kind(block_duel_kind);
    let raw_block_duel = resolve_duel_with_fatigue(
        block_duel_kind,
        blocker_rating,
        defender_block_rating,
        lead_blocker,
        lead_block_defender,
        &lead_blocker_state,
        &lead_block_def_state,
        attribute_keys,
        &block_context,
        rng,
    );

    let blocker_pos = spatial_map
        .get_position(&lead_blocker.id())
        .unwrap_or(receiver_pos_vec);
    let blocker_spd =
        calculate_effective_player_speed(lead_blocker, attribute_keys, &lead_blocker_state);

    let block_def_pos =
        get_drifted_defender_position(lead_block_defender, spatial_map, attribute_keys, rng)
            .unwrap_or(receiver_pos_vec);
    let block_def_spd = calculate_effective_player_speed(
        lead_block_defender,
        attribute_keys,
        &lead_block_def_state,
    );
    let block_duration =
        derive_duel_duration(blocker_pos, blocker_spd, block_def_pos, block_def_spd);

    let block_helper_candidates = collect_helper_candidates(
        offense_helpers,
        spatial_map,
        attribute_keys,
        blocker_pos,
        fatigue_for,
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
        defenders,
        spatial_map,
        attribute_keys,
        blocker_pos,
        fatigue_for,
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

    if !raw_block_duel.attacker_won() {
        let mut duration_ledger = DurationLedger::new();
        duration_ledger
            .record_live(DurationComponentKind::RunAfterCatchEngagement, block_duration);

        return RunAfterCatchOutcome {
            additional_mirins_advanced: 0.0,
            duels: vec![block_duel],
            turnover: None,
            recovering_player_id: None,
            duration_ledger,
        };
    }

    let block_bonus =
        (raw_block_duel.net_advantage() * BLOCK_BONUS_MULTIPLIER).max(BLOCK_BONUS_MIN);
    let (rb_offense_profile, rb_defense_profile) = get_duel_profiles(DuelKind::RunBreakthrough);

    let attacker_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        receiver,
        receiver_pos_domain,
        &blocker_subset,
        offense_position_index,
        attribute_keys,
        &rb_offense_profile,
        fatigue_for,
    ) + block_bonus;

    let defender_rating = calculate_side_rating_from_index_with_fatigue(
        defenders,
        defense_position_index,
        attribute_keys,
        &rb_defense_profile,
        fatigue_for,
    );

    let lead_defender = resolve_primary_lead_defender(
        receiver.id(),
        offense_position_index,
        receiver_pos_vec,
        Velocity::zero(),
        defenders,
        spatial_map,
        defense_instructions_index,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    );

    let receiver_state = fatigue_for(&receiver.id());
    let lead_def_state = fatigue_for(&lead_defender.id());

    let rb_context = context.for_duel_kind(DuelKind::RunBreakthrough);
    let raw_rb_duel = resolve_duel_with_fatigue(
        DuelKind::RunBreakthrough,
        attacker_rating,
        defender_rating,
        receiver,
        lead_defender,
        &receiver_state,
        &lead_def_state,
        attribute_keys,
        &rb_context,
        rng,
    );

    let rb_def_pos =
        get_drifted_defender_position(lead_defender, spatial_map, attribute_keys, rng)
            .unwrap_or(receiver_pos_vec);
    let rb_def_spd =
        calculate_effective_player_speed(lead_defender, attribute_keys, &lead_def_state);

    let rec_spd = calculate_effective_player_speed(receiver, attribute_keys, &receiver_state);
    let rb_duration = derive_duel_duration(receiver_pos_vec, rec_spd, rb_def_pos, rb_def_spd);

    let rb_helper_candidates = collect_helper_candidates(
        offense_helpers,
        spatial_map,
        attribute_keys,
        receiver_pos_vec,
        fatigue_for,
    );
    let rb_attacker_ids = collect_swept_participant_ids(
        receiver.id(),
        receiver_pos_vec,
        Velocity::zero(),
        &rb_helper_candidates,
        contest_radius,
        rb_duration,
    );

    let rb_defender_candidates = collect_drifted_defender_candidates(
        defenders,
        spatial_map,
        attribute_keys,
        receiver_pos_vec,
        fatigue_for,
        rng,
    );
    let rb_defender_ids = collect_swept_participant_ids(
        lead_defender.id(),
        receiver_pos_vec,
        Velocity::zero(),
        &rb_defender_candidates,
        contest_radius,
        rb_duration,
    );

    let rb_duel = AttributedDuelOutcome::new(raw_rb_duel, rb_attacker_ids, rb_defender_ids);

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        DurationComponentKind::RunAfterCatchEngagement,
        Duration::new(block_duration.value() + rb_duration.value()),
    );

    if raw_rb_duel.attacker_won() {
        let progression_strategy = AggregateProgressionStrategy::default();
        let additional_mirins_advanced =
            progression_strategy.resolve_progression(&raw_rb_duel, rng);

        RunAfterCatchOutcome {
            additional_mirins_advanced,
            duels: vec![block_duel, rb_duel],
            turnover: None,
            recovering_player_id: None,
            duration_ledger,
        }
    } else {
        let close_defenders: Vec<&Player> = defenders
            .iter()
            .copied()
            .filter(|cand| {
                get_drifted_defender_position(cand, spatial_map, attribute_keys, rng)
                    .map(|p| {
                        calculate_distance_mirim(receiver_pos_vec, p)
                            <= PROXIMITY_CONTEST_RADIUS_MIRIM
                    })
                    .unwrap_or(false)
            })
            .collect();

        let (sec_defenders, sec_lead) = if !close_defenders.is_empty() {
            (close_defenders.as_slice(), close_defenders[0])
        } else {
            (&defenders[..1], defenders[0])
        };

        let sec_context = context.for_duel_kind(DuelKind::BallSecurityCarry);
        let sec_result = resolve_ball_security(
            DuelKind::BallSecurityCarry,
            receiver,
            receiver_pos_domain,
            sec_defenders,
            defense_position_index,
            attribute_keys,
            defense_team_id,
            &sec_context,
            fatigue_for,
            rng,
        );

        let sec_lead_state = fatigue_for(&sec_lead.id());
        let sec_def_pos =
            get_drifted_defender_position(sec_lead, spatial_map, attribute_keys, rng)
                .unwrap_or(receiver_pos_vec);
        let sec_def_spd =
            calculate_effective_player_speed(sec_lead, attribute_keys, &sec_lead_state);

        let rec_spd = calculate_effective_player_speed(receiver, attribute_keys, &receiver_state);
        let sec_duration =
            derive_duel_duration(receiver_pos_vec, rec_spd, sec_def_pos, sec_def_spd);
        duration_ledger.record_live(
            DurationComponentKind::BallSecurityEngagement,
            sec_duration,
        );

        RunAfterCatchOutcome {
            additional_mirins_advanced: 0.0,
            duels: vec![block_duel, rb_duel, sec_result.duel_outcome],
            turnover: sec_result.turnover_team_id,
            recovering_player_id: sec_result.recovering_player_id,
            duration_ledger,
        }
    }
}
