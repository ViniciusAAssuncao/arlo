use crate::match_decision::target_selection::{select_target_with_fatigue, ReceptionRole};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_player_duel_rating_with_state, calculate_side_rating_from_index_with_fatigue,
};
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::positioning_drift::{
    get_drifted_attacker_position, get_drifted_defender_position, nearest_drifted_opponent,
};
use crate::spatial::proximity::{calculate_distance_mirim, filter_active_duelists_swept};
use crate::spatial::DynamicSpatialMap;
use crate::team_identity::marking::resolve_lead_defender_with_marking;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Speed, Velocity, MIRIM_TO_METERS};
use arlo_tactics::{PlayerInstructions, TeamInstructions};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ReceptionOutcome {
    pub receiver: Uuid,
    pub receiver_player: Player,
    pub caught: bool,
    pub duel: AttributedDuelOutcome,
    pub is_aerial: bool,
    pub duration: Duration,
    pub intercepted_by_defender: Option<Uuid>,
}

pub fn resolve_reception<F, R>(
    decision_kind: ArtrineDecisionKind,
    passer_or_artrine: &Player,
    candidates: &[&Player],
    defenders: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    position_index: &HashMap<Uuid, DomainPosition>,
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    offense_instructions: &TeamInstructions,
    attacking_positive_x: bool,
    context: &DuelContext,
    fatigue_for: &F,
    defense_pressing_multiplier: f64,
    rng: &mut R,
) -> ReceptionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let receiver_id = select_target_with_fatigue(
        candidates,
        spatial_map,
        pitch,
        position_index,
        instructions_index,
        attribute_keys,
        attacking_positive_x,
        ReceptionRole::OpenPlayReceiver,
        fatigue_for,
        rng,
    )
    .unwrap_or_else(|| passer_or_artrine.id());

    let receiver_player = candidates
        .iter()
        .copied()
        .find(|p| p.id() == receiver_id)
        .unwrap_or(passer_or_artrine);

    let is_aerial = matches!(
        decision_kind,
        ArtrineDecisionKind::LongLaunch | ArtrineDecisionKind::Cross
    );

    let duel_kind = if is_aerial {
        DuelKind::AerialDuel
    } else {
        DuelKind::RouteContest
    };

    let (offense_profile, defense_profile) = get_duel_profiles(duel_kind);

    let receiver_pos_domain = position_index
        .get(&receiver_id)
        .copied()
        .unwrap_or_else(|| {
            receiver_player
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(DomainPosition::CenterOffense)
        });

    let receiver_state = fatigue_for(&receiver_id);

    let attacker_rating = calculate_player_duel_rating_with_state(
        receiver_player,
        receiver_pos_domain,
        attribute_keys,
        &offense_profile,
        &receiver_state,
    );

    let receiver_player_instructions = instructions_index
        .get(&receiver_id)
        .copied()
        .unwrap_or_default();

    let receiver_pos_vec = get_drifted_attacker_position(
        receiver_player,
        spatial_map,
        attribute_keys,
        offense_instructions,
        receiver_player_instructions,
        rng,
    )
    .or_else(|| spatial_map.get_position(&receiver_id))
    .unwrap_or_else(VectorPosition::zero);

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

    let active_defenders: &[&Player] = if !close_defenders.is_empty() {
        &close_defenders
    } else {
        defenders
    };

    let defender_rating = calculate_side_rating_from_index_with_fatigue(
        active_defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
        fatigue_for,
    );

    let contest_radius = Length::new(PROXIMITY_CONTEST_RADIUS_MIRIM * defense_pressing_multiplier * MIRIM_TO_METERS);
    let lead_defender = resolve_lead_defender_with_marking(
        receiver_id,
        position_index,
        receiver_pos_vec,
        Velocity::zero(),
        active_defenders,
        spatial_map,
        defense_instructions_index,
        attribute_keys,
        fatigue_for,
        contest_radius,
        None,
        rng,
    )
    .unwrap_or(defenders[0]);

    let lead_def_state = fatigue_for(&lead_defender.id());

    let rec_context = context.for_duel_kind(duel_kind);
    let raw_duel = resolve_duel_with_fatigue(
        duel_kind,
        attacker_rating,
        defender_rating,
        receiver_player,
        lead_defender,
        &receiver_state,
        &lead_def_state,
        attribute_keys,
        &rec_context,
        rng,
    );

    let caught = raw_duel.attacker_won();

    let intercepted_by_defender = if !caught {
        let def_hands = extract_attribute_value(lead_defender, attribute_keys, AttributeKey::HandsReception);
        let def_ant = extract_attribute_value(lead_defender, attribute_keys, AttributeKey::Anticipation);
        let att_hands = extract_attribute_value(receiver_player, attribute_keys, AttributeKey::HandsReception);
        let hands_diff = def_hands - att_hands;
        let threshold = -(2.5 - (hands_diff * 0.2 + def_ant * 0.1).clamp(-2.0, 3.0));
        if raw_duel.net_advantage() <= threshold {
            Some(lead_defender.id())
        } else {
            None
        }
    } else {
        None
    };

    let receiver_speed = calculate_effective_player_speed(receiver_player, attribute_keys, &receiver_state);
    let duration = match
        nearest_drifted_opponent(receiver_pos_vec, defenders, spatial_map, attribute_keys, rng)
    {
        Some((d, pos)) => {
            let d_state = fatigue_for(&d.id());
            let d_spd = calculate_effective_player_speed(d, attribute_keys, &d_state);
            derive_duel_duration(receiver_pos_vec, receiver_speed, pos, d_spd)
        }
        None => Duration::new(MINIMUM_ENGAGEMENT_SECONDS),
    };

    let defender_candidates: Vec<(&Player, VectorPosition, Speed)> = active_defenders
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

    let mut active_defender_ids = vec![lead_defender.id()];
    for id in filter_active_duelists_swept(
        receiver_pos_vec,
        Velocity::zero(),
        &defender_candidates,
        contest_radius,
        duration,
    ) {
        if !active_defender_ids.contains(&id) {
            active_defender_ids.push(id);
        }
    }

    let duel = AttributedDuelOutcome::new(
        raw_duel,
        vec![receiver_id],
        active_defender_ids,
    );

    ReceptionOutcome {
        receiver: receiver_id,
        receiver_player: receiver_player.clone(),
        caught,
        duel,
        is_aerial,
        duration,
        intercepted_by_defender,
    }
}