use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::apply_saturation;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, GROUP_AGGREGATION_SATURATION_MULTIPLIER,
    GROUP_AGGREGATION_SATURATION_THRESHOLD, GROUP_SATURATION_MULTIPLIER,
    GROUP_SATURATION_THRESHOLD,
};
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_duel_rating_with_state(
    player: &Player,
    functional_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    state: &PhysicalState,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let mut total_weight = 0.0;
    let mut accumulated = 0.0;

    for w in profile.weights() {
        if w.weight > 0.0 {
            let val = extract_effective_attribute_value(&table, w.key, state);
            accumulated += val * w.weight;
            total_weight += w.weight;
        }
    }

    let raw = if total_weight > 0.0 {
        accumulated / total_weight
    } else {
        0.0
    };
    let fit = calculate_fit_for_position(player, functional_position);
    raw * fit.efficiency_multiplier()
}

pub fn calculate_player_duel_rating(
    player: &Player,
    functional_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    calculate_player_duel_rating_with_state(
        player,
        functional_position,
        attribute_keys,
        profile,
        &PhysicalState::initial(),
    )
}

pub fn calculate_group_rating(ratings: &[f64]) -> f64 {
    if ratings.is_empty() {
        return 0.0;
    }
    let mut sorted = ratings.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let lead = sorted[0];
    if sorted.len() == 1 {
        return lead;
    }

    let mut helper_sum = 0.0;
    for &rating in &sorted[1..] {
        let raw_contrib = (rating / ATTRIBUTE_SATURATION_THRESHOLD) * GROUP_SATURATION_THRESHOLD;
        let sat_contrib = apply_saturation(
            raw_contrib,
            GROUP_SATURATION_THRESHOLD,
            GROUP_SATURATION_MULTIPLIER,
        );
        helper_sum += sat_contrib;
    }

    let aggregated_bonus = apply_saturation(
        helper_sum,
        GROUP_AGGREGATION_SATURATION_THRESHOLD,
        GROUP_AGGREGATION_SATURATION_MULTIPLIER,
    );

    lead + aggregated_bonus
}

pub fn calculate_side_rating(
    players: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let ratings: Vec<f64> = players
        .iter()
        .map(|(p, pos)| calculate_player_duel_rating(p, *pos, attribute_keys, profile))
        .collect();
    calculate_group_rating(&ratings)
}

pub fn calculate_side_rating_with_fatigue<F>(
    players: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    fatigue_for: &F,
) -> f64
where
    F: Fn(&Uuid) -> PhysicalState,
{
    let ratings: Vec<f64> = players
        .iter()
        .map(|(p, pos)| {
            let state = fatigue_for(&p.id());
            calculate_player_duel_rating_with_state(p, *pos, attribute_keys, profile, &state)
        })
        .collect();
    calculate_group_rating(&ratings)
}

pub fn calculate_side_rating_from_index(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let players_with_positions: Vec<(&Player, Position)> = players
        .iter()
        .map(|&p| {
            let pos = position_index.get(&p.id()).copied().unwrap_or_else(|| {
                p.positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::CenterOffense)
            });
            (p, pos)
        })
        .collect();
    calculate_side_rating(&players_with_positions, attribute_keys, profile)
}

pub fn calculate_side_rating_from_index_with_fatigue<F>(
    players: &[&Player],
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    fatigue_for: &F,
) -> f64
where
    F: Fn(&Uuid) -> PhysicalState,
{
    let players_with_positions: Vec<(&Player, Position)> = players
        .iter()
        .map(|&p| {
            let pos = position_index.get(&p.id()).copied().unwrap_or_else(|| {
                p.positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::CenterOffense)
            });
            (p, pos)
        })
        .collect();
    calculate_side_rating_with_fatigue(
        &players_with_positions,
        attribute_keys,
        profile,
        fatigue_for,
    )
}

pub fn calculate_anchored_rating(anchor_rating: f64, helper_ratings: &[f64]) -> f64 {
    if helper_ratings.is_empty() {
        return anchor_rating;
    }

    let mut helper_sum = 0.0;
    for &rating in helper_ratings {
        let raw_contrib = (rating / ATTRIBUTE_SATURATION_THRESHOLD) * GROUP_SATURATION_THRESHOLD;
        let sat_contrib = apply_saturation(
            raw_contrib,
            GROUP_SATURATION_THRESHOLD,
            GROUP_SATURATION_MULTIPLIER,
        );
        helper_sum += sat_contrib;
    }

    let aggregated_bonus = apply_saturation(
        helper_sum,
        GROUP_AGGREGATION_SATURATION_THRESHOLD,
        GROUP_AGGREGATION_SATURATION_MULTIPLIER,
    );

    anchor_rating + aggregated_bonus
}

pub fn calculate_anchored_side_rating(
    anchor: &Player,
    anchor_position: Position,
    helpers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let anchor_rating =
        calculate_player_duel_rating(anchor, anchor_position, attribute_keys, profile);
    let helper_ratings: Vec<f64> = helpers
        .iter()
        .map(|(p, pos)| calculate_player_duel_rating(p, *pos, attribute_keys, profile))
        .collect();
    calculate_anchored_rating(anchor_rating, &helper_ratings)
}

pub fn calculate_anchored_side_rating_with_fatigue<F>(
    anchor: &Player,
    anchor_position: Position,
    helpers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    fatigue_for: &F,
) -> f64
where
    F: Fn(&Uuid) -> PhysicalState,
{
    let anchor_state = fatigue_for(&anchor.id());
    let anchor_rating = calculate_player_duel_rating_with_state(
        anchor,
        anchor_position,
        attribute_keys,
        profile,
        &anchor_state,
    );
    let helper_ratings: Vec<f64> = helpers
        .iter()
        .map(|(p, pos)| {
            let state = fatigue_for(&p.id());
            calculate_player_duel_rating_with_state(p, *pos, attribute_keys, profile, &state)
        })
        .collect();
    calculate_anchored_rating(anchor_rating, &helper_ratings)
}

pub fn calculate_anchored_side_rating_from_index(
    anchor: &Player,
    anchor_position: Position,
    helpers: &[&Player],
    helpers_position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let helpers_with_positions: Vec<(&Player, Position)> = helpers
        .iter()
        .map(|&p| {
            let pos = helpers_position_index
                .get(&p.id())
                .copied()
                .unwrap_or_else(|| {
                    p.positions()
                        .first()
                        .map(|pp| pp.position())
                        .unwrap_or(Position::CenterOffense)
                });
            (p, pos)
        })
        .collect();
    calculate_anchored_side_rating(
        anchor,
        anchor_position,
        &helpers_with_positions,
        attribute_keys,
        profile,
    )
}

pub fn calculate_anchored_side_rating_from_index_with_fatigue<F>(
    anchor: &Player,
    anchor_position: Position,
    helpers: &[&Player],
    helpers_position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    fatigue_for: &F,
) -> f64
where
    F: Fn(&Uuid) -> PhysicalState,
{
    let helpers_with_positions: Vec<(&Player, Position)> = helpers
        .iter()
        .map(|&p| {
            let pos = helpers_position_index
                .get(&p.id())
                .copied()
                .unwrap_or_else(|| {
                    p.positions()
                        .first()
                        .map(|pp| pp.position())
                        .unwrap_or(Position::CenterOffense)
                });
            (p, pos)
        })
        .collect();
    calculate_anchored_side_rating_with_fatigue(
        anchor,
        anchor_position,
        &helpers_with_positions,
        attribute_keys,
        profile,
        fatigue_for,
    )
}

pub fn identify_lead_player<'a>(
    players: &[(&'a Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> Option<&'a Player> {
    players
        .iter()
        .max_by(|(a, pos_a), (b, pos_b)| {
            let rating_a = calculate_player_duel_rating(a, *pos_a, attribute_keys, profile);
            let rating_b = calculate_player_duel_rating(b, *pos_b, attribute_keys, profile);
            rating_a
                .partial_cmp(&rating_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(p, _)| *p)
}

pub fn identify_lead_player_from_index<'a>(
    players: &[&'a Player],
    position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> Option<&'a Player> {
    let players_with_positions: Vec<(&'a Player, Position)> = players
        .iter()
        .map(|&p| {
            let pos = position_index.get(&p.id()).copied().unwrap_or_else(|| {
                p.positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(Position::CenterOffense)
            });
            (p, pos)
        })
        .collect();
    identify_lead_player(&players_with_positions, attribute_keys, profile)
}