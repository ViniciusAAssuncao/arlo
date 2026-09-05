use crate::artrine::execution_security::resolve_ball_security;
use crate::fatigue::{compute_player_fatigue_multiplier, FatigueState};
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index, calculate_player_duel_rating,
    calculate_side_rating_from_index, identify_lead_player_from_index,
};
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct RunAfterCatchOutcome {
    pub additional_mirins_advanced: f64,
    pub duels: Vec<DuelOutcome>,
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
        calculate_side_rating_from_index(
            offense_helpers,
            offense_position_index,
            attribute_keys,
            &block_offense_profile,
        )
    } else {
        calculate_player_duel_rating(
            receiver,
            receiver_pos_domain,
            attribute_keys,
            &block_offense_profile,
        )
    };

    let defender_block_rating = calculate_side_rating_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &block_defense_profile,
    );

    let lead_block_defender = identify_lead_player_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &block_defense_profile,
    )
    .unwrap_or(defenders[0]);

    let block_duel = resolve_duel(
        block_duel_kind,
        blocker_rating,
        defender_block_rating,
        lead_blocker,
        lead_block_defender,
        attribute_keys,
        context,
        rng,
    );

    let blocker_pos = spatial_map
        .get_position(&lead_blocker.id())
        .unwrap_or(receiver_pos_vec);
    let blocker_mult = compute_player_fatigue_multiplier(
        lead_blocker,
        &fatigue_for(&lead_blocker.id()),
        attribute_keys,
    );
    let blocker_spd = calculate_player_speed(lead_blocker, attribute_keys, blocker_mult);

    let block_def_pos = spatial_map
        .get_position(&lead_block_defender.id())
        .unwrap_or(receiver_pos_vec);
    let block_def_mult = compute_player_fatigue_multiplier(
        lead_block_defender,
        &fatigue_for(&lead_block_defender.id()),
        attribute_keys,
    );
    let block_def_spd =
        calculate_player_speed(lead_block_defender, attribute_keys, block_def_mult);
    let block_duration =
        derive_duel_duration(blocker_pos, blocker_spd, block_def_pos, block_def_spd);

    if !block_duel.attacker_won() {
        let close_defenders: Vec<&Player> = defenders
            .iter()
            .copied()
            .filter(|cand| {
                spatial_map
                    .get_position(&cand.id())
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
            (defenders, defenders[0])
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
            rng,
        );

        let sec_def_pos = spatial_map
            .get_position(&sec_lead.id())
            .unwrap_or(receiver_pos_vec);
        let sec_def_mult = compute_player_fatigue_multiplier(
            sec_lead,
            &fatigue_for(&sec_lead.id()),
            attribute_keys,
        );
        let sec_def_spd = calculate_player_speed(sec_lead, attribute_keys, sec_def_mult);

        let rec_mult = compute_player_fatigue_multiplier(
            receiver,
            &fatigue_for(&receiver.id()),
            attribute_keys,
        );
        let rec_spd = calculate_player_speed(receiver, attribute_keys, rec_mult);
        let sec_duration =
            derive_duel_duration(receiver_pos_vec, rec_spd, sec_def_pos, sec_def_spd);

        let mut duration_ledger = DurationLedger::new();
        duration_ledger
            .record_live(DurationComponentKind::RunAfterCatchEngagement, block_duration);
        duration_ledger
            .record_live(DurationComponentKind::BallSecurityEngagement, sec_duration);

        return RunAfterCatchOutcome {
            additional_mirins_advanced: 0.0,
            duels: vec![block_duel, sec_result.duel_outcome],
            turnover: sec_result.turnover_team_id,
            recovering_player_id: sec_result.recovering_player_id,
            duration_ledger,
        };
    }

    let block_bonus = (block_duel.net_advantage() * 0.35).max(0.5);
    let (rb_offense_profile, rb_defense_profile) = get_duel_profiles(DuelKind::RunBreakthrough);

    let attacker_rating = calculate_anchored_side_rating_from_index(
        receiver,
        receiver_pos_domain,
        offense_helpers,
        offense_position_index,
        attribute_keys,
        &rb_offense_profile,
    ) + block_bonus;

    let defender_rating = calculate_side_rating_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &rb_defense_profile,
    );

    let lead_defender = identify_lead_player_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &rb_defense_profile,
    )
    .unwrap_or(defenders[0]);

    let rb_duel = resolve_duel(
        DuelKind::RunBreakthrough,
        attacker_rating,
        defender_rating,
        receiver,
        lead_defender,
        attribute_keys,
        context,
        rng,
    );

    let rb_def_pos = spatial_map
        .get_position(&lead_defender.id())
        .unwrap_or(receiver_pos_vec);
    let rb_def_mult = compute_player_fatigue_multiplier(
        lead_defender,
        &fatigue_for(&lead_defender.id()),
        attribute_keys,
    );
    let rb_def_spd = calculate_player_speed(lead_defender, attribute_keys, rb_def_mult);

    let rec_mult = compute_player_fatigue_multiplier(
        receiver,
        &fatigue_for(&receiver.id()),
        attribute_keys,
    );
    let rec_spd = calculate_player_speed(receiver, attribute_keys, rec_mult);
    let rb_duration = derive_duel_duration(receiver_pos_vec, rec_spd, rb_def_pos, rb_def_spd);

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        DurationComponentKind::RunAfterCatchEngagement,
        Duration::new(block_duration.value() + rb_duration.value()),
    );

    if rb_duel.attacker_won() {
        let progression_strategy = AggregateProgressionStrategy::default();
        let additional_mirins_advanced = progression_strategy.resolve_progression(&rb_duel, rng);

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
                spatial_map
                    .get_position(&cand.id())
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
            (defenders, defenders[0])
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
            rng,
        );

        let sec_def_pos = spatial_map
            .get_position(&sec_lead.id())
            .unwrap_or(receiver_pos_vec);
        let sec_def_mult = compute_player_fatigue_multiplier(
            sec_lead,
            &fatigue_for(&sec_lead.id()),
            attribute_keys,
        );
        let sec_def_spd = calculate_player_speed(sec_lead, attribute_keys, sec_def_mult);
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
