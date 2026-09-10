use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
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

#[derive(Clone, Copy)]
pub struct RatingParticipants<'a> {
    pub players: &'a [&'a Player],
    pub position_index: Option<&'a HashMap<Uuid, Position>>,
    pub fatigue_lookup: Option<&'a dyn Fn(&Uuid) -> PhysicalState>,
    pub attribute_tables: Option<&'a HashMap<Uuid, PlayerAttributeTable>>,
}

impl<'a> RatingParticipants<'a> {
    pub fn new(players: &'a [&'a Player]) -> Self {
        Self {
            players,
            position_index: None,
            fatigue_lookup: None,
            attribute_tables: None,
        }
    }

    pub fn from_slice_with_index(
        players: &'a [&'a Player],
        position_index: &'a HashMap<Uuid, Position>,
    ) -> Self {
        Self {
            players,
            position_index: Some(position_index),
            fatigue_lookup: None,
            attribute_tables: None,
        }
    }

    pub fn with_fatigue(mut self, fatigue_lookup: &'a dyn Fn(&Uuid) -> PhysicalState) -> Self {
        self.fatigue_lookup = Some(fatigue_lookup);
        self
    }

    pub fn with_optional_fatigue(
        mut self,
        fatigue_lookup: Option<&'a dyn Fn(&Uuid) -> PhysicalState>,
    ) -> Self {
        self.fatigue_lookup = fatigue_lookup;
        self
    }

    pub fn with_attribute_tables(
        mut self,
        tables: &'a HashMap<Uuid, PlayerAttributeTable>,
    ) -> Self {
        self.attribute_tables = Some(tables);
        self
    }
}

pub fn calculate_player_duel_rating_from_table(
    player: &Player,
    functional_position: Position,
    table: &PlayerAttributeTable,
    profile: &DuelProfile,
    state: &PhysicalState,
) -> f64 {
    let mut total_weight = 0.0;
    let mut accumulated = 0.0;

    for w in profile.weights() {
        if w.weight > 0.0 {
            let val = extract_effective_attribute_value(table, w.key, state);
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

pub fn calculate_player_duel_rating_with_state(
    player: &Player,
    functional_position: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    state: &PhysicalState,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_duel_rating_from_table(player, functional_position, &table, profile, state)
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

fn resolve_participant_rating(
    participants: &RatingParticipants<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
    player: &Player,
) -> f64 {
    let default_state = PhysicalState::initial();
    let pos = participants
        .position_index
        .and_then(|idx| idx.get(&player.id()).copied())
        .unwrap_or_else(|| {
            player
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(Position::CenterOffense)
        });
    let state = match participants.fatigue_lookup {
        Some(lookup) => lookup(&player.id()),
        None => default_state,
    };
    if let Some(tables) = participants.attribute_tables {
        let table = tables.get(&player.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        return calculate_player_duel_rating_from_table(player, pos, table, profile, &state);
    }
    calculate_player_duel_rating_with_state(player, pos, attribute_keys, profile, &state)
}

pub fn calculate_side_rating(
    participants: RatingParticipants<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    if participants.players.is_empty() {
        return 0.0;
    }

    let ratings: Vec<f64> = participants
        .players
        .iter()
        .map(|&p| resolve_participant_rating(&participants, attribute_keys, profile, p))
        .collect();

    calculate_group_rating(&ratings)
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
    helpers: RatingParticipants<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let default_state = PhysicalState::initial();
    let anchor_state = match helpers.fatigue_lookup {
        Some(lookup) => lookup(&anchor.id()),
        None => default_state,
    };

    let anchor_rating = match helpers.attribute_tables {
        Some(tables) => {
            let table = tables.get(&anchor.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            calculate_player_duel_rating_from_table(
                anchor,
                anchor_position,
                table,
                profile,
                &anchor_state,
            )
        }
        None => calculate_player_duel_rating_with_state(
            anchor,
            anchor_position,
            attribute_keys,
            profile,
            &anchor_state,
        ),
    };

    if helpers.players.is_empty() {
        return anchor_rating;
    }

    let helper_ratings: Vec<f64> = helpers
        .players
        .iter()
        .map(|&p| resolve_participant_rating(&helpers, attribute_keys, profile, p))
        .collect();

    calculate_anchored_rating(anchor_rating, &helper_ratings)
}
