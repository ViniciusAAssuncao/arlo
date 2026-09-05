use crate::fatigue::{compute_player_fatigue_multiplier, FatigueState};
use crate::match_decision::target_selection::{select_target, ReceptionRole};
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::{derive_duel_duration, nearest_opponent};
use crate::resolution::group_rating::{
    calculate_player_duel_rating, calculate_side_rating_from_index,
    identify_lead_player_from_index,
};
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ReceptionOutcome {
    pub receiver: Uuid,
    pub receiver_player: Player,
    pub caught: bool,
    pub duel: DuelOutcome,
    pub is_aerial: bool,
    pub duration: Duration,
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
    attacking_positive_x: bool,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> ReceptionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let receiver_id = select_target(
        candidates,
        spatial_map,
        pitch,
        attacking_positive_x,
        ReceptionRole::OpenPlayReceiver,
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

    let attacker_rating = calculate_player_duel_rating(
        receiver_player,
        receiver_pos_domain,
        attribute_keys,
        &offense_profile,
    );

    let receiver_pos_vec = spatial_map
        .get_position(&receiver_id)
        .unwrap_or_else(VectorPosition::zero);

    let close_defenders: Vec<&Player> = defenders
        .iter()
        .copied()
        .filter(|cand| {
            spatial_map
                .get_position(&cand.id())
                .map(|p| calculate_distance_mirim(receiver_pos_vec, p) <= PROXIMITY_CONTEST_RADIUS_MIRIM)
                .unwrap_or(false)
        })
        .collect();

    let active_defenders: &[&Player] = if !close_defenders.is_empty() {
        &close_defenders
    } else {
        defenders
    };

    let defender_rating = calculate_side_rating_from_index(
        active_defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
    );

    let lead_defender = identify_lead_player_from_index(
        active_defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
    )
    .unwrap_or(defenders[0]);

    let duel = resolve_duel(
        duel_kind,
        attacker_rating,
        defender_rating,
        receiver_player,
        lead_defender,
        attribute_keys,
        context,
        rng,
    );

    let caught = duel.attacker_won();

    let rec_mult = compute_player_fatigue_multiplier(
        receiver_player,
        &fatigue_for(&receiver_player.id()),
        attribute_keys,
    );
    let receiver_speed = calculate_player_speed(receiver_player, attribute_keys, rec_mult);
    let duration = match nearest_opponent(receiver_pos_vec, defenders, spatial_map) {
        Some((d, pos)) => {
            let d_mult = compute_player_fatigue_multiplier(d, &fatigue_for(&d.id()), attribute_keys);
            let d_spd = calculate_player_speed(d, attribute_keys, d_mult);
            derive_duel_duration(receiver_pos_vec, receiver_speed, pos, d_spd)
        }
        None => Duration::new(MINIMUM_ENGAGEMENT_SECONDS),
    };

    ReceptionOutcome {
        receiver: receiver_id,
        receiver_player: receiver_player.clone(),
        caught,
        duel,
        is_aerial,
        duration,
    }
}
