pub mod attributed_outcome;
pub mod context;
pub mod duel_kind;
pub mod duel_noise;
pub mod evaluation;
pub mod execution;
pub mod group_rating;
pub mod outcome;
pub mod outcome_distribution;
pub mod resolver;
pub mod slope_calibration;

pub use attributed_outcome::AttributedDuelOutcome;
pub use context::DuelContext;
pub use duel_kind::DuelKind;
pub use duel_noise::{
    player_consistency_noise_scale, player_consistency_noise_std_dev,
    pressure_urgency_activation, sample_player_noise, sample_player_noise_with_pressure,
};
pub use evaluation::{evaluate_duel, EvaluatedDuel};
pub use execution::{calculate_velocity_mitigation, execute_duel};
pub use group_rating::{
    calculate_anchored_rating, calculate_anchored_side_rating, calculate_group_rating,
    calculate_player_duel_rating, calculate_player_duel_rating_from_table,
    calculate_player_duel_rating_with_state, calculate_side_rating, RatingParticipants,
};
pub use outcome::{ContestOutcome, DuelOutcome};
pub use outcome_distribution::*;
pub use resolver::{resolve_contest, resolve_duel, ContestRequest, DuelResolutionRequest};
pub use slope_calibration::logistic_slope_for;

pub use crate::attributes::profiles::{
    get_duel_attribute_profiles as get_duel_profiles, AttributeProfile as DuelProfile,
};