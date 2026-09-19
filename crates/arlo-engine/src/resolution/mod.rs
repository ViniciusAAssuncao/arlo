pub mod attributed_outcome;
pub mod context;
pub mod duel_kind;
pub mod duel_noise;
pub mod group_rating;
pub mod orientation;
pub mod outcome;
pub mod outcome_distribution;
pub mod resolver;

pub use attributed_outcome::AttributedDuelOutcome;
pub use context::DuelContext;
pub use duel_kind::{logistic_slope_for, DuelKind};
pub use duel_noise::{player_consistency_noise_scale, sample_player_noise};
pub use group_rating::{
    calculate_anchored_rating, calculate_anchored_side_rating, calculate_group_rating,
    calculate_player_duel_rating, calculate_player_duel_rating_from_table,
    calculate_player_duel_rating_with_state, calculate_side_rating, RatingParticipants,
};
pub use orientation::ContestOrientation;
pub use outcome::{ContestOutcome, DuelOutcome};
pub use outcome_distribution::*;
pub use resolver::{
    calculate_velocity_mitigation, resolve_contest, resolve_duel, ContestRequest,
    DuelResolutionRequest,
};

pub use crate::attributes::profiles::{
    get_duel_attribute_profiles as get_duel_profiles, AttributeProfile as DuelProfile,
};
