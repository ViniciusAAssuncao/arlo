use crate::attributes::profiles::AttributeProfile as DuelProfile;
use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct RatingParticipants<'a> {
    pub players: &'a [&'a Player],
    pub position_index: Option<&'a HashMap<Uuid, Position>>,
    pub fatigue_lookup: Option<&'a dyn Fn(&Uuid) -> PhysicalState>,
    pub attribute_tables: Option<&'a HashMap<Uuid, PlayerAttributeTable>>,
    pub team_power: Option<f64>,
}

impl<'a> RatingParticipants<'a> {
    pub fn new(players: &'a [&'a Player]) -> Self {
        Self {
            players,
            position_index: None,
            fatigue_lookup: None,
            attribute_tables: None,
            team_power: None,
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
            team_power: None,
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

    pub fn with_team_power(mut self, power: f64) -> Self {
        self.team_power = Some(power);
        self
    }

    pub fn with_optional_team_power(mut self, power: Option<f64>) -> Self {
        self.team_power = power;
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
    let is_cerebral = matches!(functional_position, Position::Artrine | Position::Passer);
    let deg_ctx = DegradationContext::new(state).with_cerebral_role(is_cerebral);
    let raw = profile.evaluate_weighted_average(|key| {
        extract_effective_attribute_value(table, key, &deg_ctx)
    });
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
    if ratings.len() == 1 {
        return ratings[0];
    }
    let mut max_rating = ratings[0];
    let mut sum_helpers = 0.0;
    for &rating in ratings {
        if rating > max_rating {
            sum_helpers += max_rating;
            max_rating = rating;
        } else {
            sum_helpers += rating;
        }
    }
    let helper_count = (ratings.len() - 1) as f64;
    let avg_helper = sum_helpers / helper_count;
    max_rating * 0.75 + avg_helper * 0.25
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
        let table = tables
            .get(&player.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        return calculate_player_duel_rating_from_table(player, pos, table, profile, &state);
    }
    calculate_player_duel_rating_with_state(player, pos, attribute_keys, profile, &state)
}

pub fn calculate_side_rating(
    participants: RatingParticipants<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    if let Some(power) = participants.team_power {
        return power;
    }
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
    let sum: f64 = helper_ratings.iter().copied().sum();
    let avg_helper = sum / (helper_ratings.len() as f64);
    anchor_rating * 0.75 + avg_helper * 0.25
}

pub fn calculate_anchored_side_rating(
    anchor: &Player,
    anchor_position: Position,
    helpers: RatingParticipants<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    if let Some(power) = helpers.team_power {
        return power;
    }
    let default_state = PhysicalState::initial();
    let anchor_state = match helpers.fatigue_lookup {
        Some(lookup) => lookup(&anchor.id()),
        None => default_state,
    };

    let anchor_rating = match helpers.attribute_tables {
        Some(tables) => {
            let table = tables
                .get(&anchor.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
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