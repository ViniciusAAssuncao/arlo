pub mod aggregate_progression;
pub mod context;
pub mod duel_kind;
pub mod duel_profiles;
pub mod duel_timing;
pub mod group_rating;
pub mod outcome;
pub mod progression_strategy;
pub mod resolver;

pub use aggregate_progression::AggregateProgressionStrategy;
pub use context::DuelContext;
pub use duel_kind::DuelKind;
pub use duel_profiles::{get_duel_profiles, DuelProfile};
pub use duel_timing::{derive_duel_duration, nearest_opponent, time_to_close};
pub use group_rating::{
    calculate_anchored_rating, calculate_anchored_side_rating,
    calculate_anchored_side_rating_from_index, calculate_group_rating,
    calculate_player_duel_rating, calculate_side_rating, calculate_side_rating_from_index,
    identify_lead_player, identify_lead_player_from_index,
};
pub use outcome::DuelOutcome;
pub use progression_strategy::ProgressionResolutionStrategy;
pub use resolver::{
    resolve_duel, resolve_duel_for_participants, resolve_duel_for_participants_from_index,
};
