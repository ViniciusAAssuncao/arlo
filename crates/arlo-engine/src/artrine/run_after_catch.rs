use crate::artrine::execution_security::resolve_ball_security;
use crate::physical::FatigueState;
use crate::physical::systems::degradation::calculate_effective_player_speed;
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
use crate::spatial::interception::identify_kinematic_lead_defender_with_drift;
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::{calculate_distance_mirim, filter_active_duelists_swept};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Speed, Velocity, MIRIM_TO_METERS};
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
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    defense_team_id: Uuid,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> RunAfterCatchOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let receiver_pos_vec = spatial_map
        .get_position(&receiver.id())
        .unwrap_or_else(VectorPosition::zero);

    let total_width = pitch.width().value();
    let normalized_y = if total_width > 0.0 {
        (receiver_pos_vec.raw().1 / total_width).clamp(0.0, 1.0)
    } else {
        0.5
    };

    let is_central = (0.25..=0.75).contains(&normalized_y);
    let block_duel_kind = if is_central {
        DuelKind::CentralBlock
    } else {
        DuelKind::LateralBlock
    };

    let (block_offense_profile, block_defense_profile) = get_duel_profiles(block_duel_kind);

    let lead_blocker = if !offense_helpers.is_empty() {
        identify_lead_player_from_index(
            offense_helpers,
            offense_position_index,
            attribute_keys,
            &block_offense_profile,
        )
        .unwrap_or(offense_helpers[0])
    } else {
        receiver
    };

    let blocker_rating = if !offense_helpers.is_empty() {
        calculate_side_rating_from_index_with_fatigue(
            offense_helpers,
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

    let contest_radius = Length::new(PROXIMITY_CONTEST_RADIUS_MIRIM * MIRIM_TO_METERS);
    let lead_block_defender = identify_kinematic_lead_defender_with_drift(
        receiver_pos_vec,
        Velocity::zero(),
        defenders,
        spatial_map,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    )
    .unwrap_or(defenders[0]);

    let lead_blocker_state = fatigue_for(&lead_blocker.id());
    let lead_block_def_state = fatigue_for(&lead_block_defender.id());

    let raw_block_duel = resolve_duel_with_fatigue(
        block_duel_kind,
        blocker_rating,
        defender_block_rating,
        lead_blocker,
        lead_block_defender,
        &lead_blocker_state,
        &lead_block_def_state,
        attribute_keys,
        context,
        rng,
    );

    let blocker_pos = spatial_map
        .get_position(&lead_blocker.id())
        .unwrap_or(receiver_pos_vec);
    let blocker_spd = calculate_effective_player_speed(lead_blocker, attribute_keys, &lead_blocker_state);

    let block_def_pos = get_drifted_defender_position(lead_block_defender, spatial_map, attribute_keys, rng)
        .unwrap_or(receiver_pos_vec);
    let block_def_spd =
        calculate_effective_player_speed(lead_block_defender, attribute_keys, &lead_block_def_state);
    let block_duration =
        derive_duel_duration(blocker_pos, blocker_spd, block_def_pos, block_def_spd);

    let block_helper_candidates: Vec<(&Player, VectorPosition, Speed)> = offense_helpers
        .iter()
        .map(|&p| {
            let pos = spatial_map.get_position(&p.id()).unwrap_or(blocker_pos);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
            (p, pos, spd)
        })
        .collect();

    let mut block_attacker_ids = vec![lead_blocker.id()];
    for id in filter_active_duelists_swept(
        blocker_pos,
        Velocity::zero(),
        &block_helper_candidates,
        contest_radius,
        block_duration,
    ) {
        if !block_attacker_ids.contains(&id) {
            block_attacker_ids.push(id);
        }
    }

    let block_defender_candidates: Vec<(&Player, VectorPosition, Speed)> = defenders
        .iter()
        .map(|&p| {
            let pos = get_drifted_defender_position(p, spatial_map, attribute_keys, rng)
                .or_else(|| spatial_map.get_position(&p.id()))
                .unwrap_or(blocker_pos);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
            (p, pos, spd)
        })
        .collect();

    let mut block_defender_ids = vec![lead_block_defender.id()];
    for id in filter_active_duelists_swept(
        blocker_pos,
        Velocity::zero(),
        &block_defender_candidates,
        contest_radius,
        block_duration,
    ) {
        if !block_defender_ids.contains(&id) {
            block_defender_ids.push(id);
        }
    }

    let block_duel = AttributedDuelOutcome::new(
        raw_block_duel,
        block_attacker_ids,
        block_defender_ids,
    );

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

    let block_bonus = (raw_block_duel.net_advantage() * 0.35).max(0.5);
    let (rb_offense_profile, rb_defense_profile) = get_duel_profiles(DuelKind::RunBreakthrough);

    let attacker_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        receiver,
        receiver_pos_domain,
        offense_helpers,
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

    let lead_defender = identify_kinematic_lead_defender_with_drift(
        receiver_pos_vec,
        Velocity::zero(),
        defenders,
        spatial_map,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    )
    .unwrap_or(defenders[0]);

    let receiver_state = fatigue_for(&receiver.id());
    let lead_def_state = fatigue_for(&lead_defender.id());

    let raw_rb_duel = resolve_duel_with_fatigue(
        DuelKind::RunBreakthrough,
        attacker_rating,
        defender_rating,
        receiver,
        lead_defender,
        &receiver_state,
        &lead_def_state,
        attribute_keys,
        context,
        rng,
    );

    let rb_def_pos = get_drifted_defender_position(lead_defender, spatial_map, attribute_keys, rng)
        .unwrap_or(receiver_pos_vec);
    let rb_def_spd = calculate_effective_player_speed(lead_defender, attribute_keys, &lead_def_state);

    let rec_spd = calculate_effective_player_speed(receiver, attribute_keys, &receiver_state);
    let rb_duration = derive_duel_duration(receiver_pos_vec, rec_spd, rb_def_pos, rb_def_spd);

    let rb_helper_candidates: Vec<(&Player, VectorPosition, Speed)> = offense_helpers
        .iter()
        .map(|&p| {
            let pos = spatial_map.get_position(&p.id()).unwrap_or(receiver_pos_vec);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
            (p, pos, spd)
        })
        .collect();

    let mut rb_attacker_ids = vec![receiver.id()];
    for id in filter_active_duelists_swept(
        receiver_pos_vec,
        Velocity::zero(),
        &rb_helper_candidates,
        contest_radius,
        rb_duration,
    ) {
        if !rb_attacker_ids.contains(&id) {
            rb_attacker_ids.push(id);
        }
    }

    let rb_defender_candidates: Vec<(&Player, VectorPosition, Speed)> = defenders
        .iter()
        .map(|&p| {
            let pos = get_drifted_defender_position(p, spatial_map, attribute_keys, rng)
                .or_else(|| spatial_map.get_position(&p.id()))
                .unwrap_or(receiver_pos_vec);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed(p, attribute_keys, &st);
            (p, pos, spd)
        })
        .collect();

    let mut rb_defender_ids = vec![lead_defender.id()];
    for id in filter_active_duelists_swept(
        receiver_pos_vec,
        Velocity::zero(),
        &rb_defender_candidates,
        contest_radius,
        rb_duration,
    ) {
        if !rb_defender_ids.contains(&id) {
            rb_defender_ids.push(id);
        }
    }

    let rb_duel = AttributedDuelOutcome::new(
        raw_rb_duel,
        rb_attacker_ids,
        rb_defender_ids,
    );

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        DurationComponentKind::RunAfterCatchEngagement,
        Duration::new(block_duration.value() + rb_duration.value()),
    );

    if raw_rb_duel.attacker_won() {
        let progression_strategy = AggregateProgressionStrategy::default();
        let additional_mirins_advanced = progression_strategy.resolve_progression(&raw_rb_duel, rng);

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

        let sec_result = resolve_ball_security(
            DuelKind::BallSecurityCarry,
            receiver,
            receiver_pos_domain,
            sec_defenders,
            defense_position_index,
            attribute_keys,
            defense_team_id,
            context,
            fatigue_for,
            rng,
        );

        let sec_lead_state = fatigue_for(&sec_lead.id());
        let sec_def_pos = get_drifted_defender_position(sec_lead, spatial_map, attribute_keys, rng)
            .unwrap_or(receiver_pos_vec);
        let sec_def_spd = calculate_effective_player_speed(sec_lead, attribute_keys, &sec_lead_state);

        let rec_spd = calculate_effective_player_speed(receiver, attribute_keys, &receiver_state);
        let sec_duration =
            derive_duel_duration(receiver_pos_vec, rec_spd, sec_def_pos, sec_def_spd);
        duration_ledger
            .record_live(DurationComponentKind::BallSecurityEngagement, sec_duration);

        RunAfterCatchOutcome {
            additional_mirins_advanced: 0.0,
            duels: vec![block_duel, rb_duel, sec_result.duel_outcome],
            turnover: sec_result.turnover_team_id,
            recovering_player_id: sec_result.recovering_player_id,
            duration_ledger,
        }
    }
}
