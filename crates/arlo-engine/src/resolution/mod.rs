pub mod aggregate_progression;
pub mod attributed_outcome;
pub mod context;
pub mod duel_kind;
pub mod duel_noise;
pub mod duel_profiles;
pub mod duel_timing;
pub mod group_rating;
pub mod outcome;
pub mod progression_strategy;
pub mod resolver;

pub use aggregate_progression::AggregateProgressionStrategy;
pub use attributed_outcome::AttributedDuelOutcome;
pub use context::DuelContext;
pub use duel_kind::{logistic_slope_for, DuelKind};
pub use duel_noise::{
    player_noise_distribution, player_noise_distribution_from_table_with_impulse,
    sample_player_noise, sample_player_noise_from_table_with_baseline,
    sample_player_noise_from_table_with_impulse, sample_player_noise_with_impulse,
    SkewNormalParams,
};
pub use duel_profiles::{get_duel_profiles, DuelProfile};
pub use duel_timing::{derive_duel_duration, nearest_opponent, time_to_close};
pub use group_rating::{
    calculate_anchored_rating, calculate_anchored_side_rating, calculate_group_rating,
    calculate_player_duel_rating, calculate_player_duel_rating_from_table,
    calculate_player_duel_rating_with_state, calculate_side_rating, identify_lead_player,
    identify_lead_player_from_index, RatingParticipants,
};
pub use outcome::DuelOutcome;
pub use progression_strategy::ProgressionResolutionStrategy;
pub use resolver::{calculate_velocity_mitigation, resolve_duel, DuelResolutionRequest};