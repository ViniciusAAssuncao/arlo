pub mod aggregator;
pub(crate) mod category_signal;
pub mod config;
pub mod diagnostics;
pub mod finalizer;
pub mod player_state;
pub mod rating_calculator;
pub mod replay;
pub mod seed;
pub mod snapshot;

pub use aggregator::PlayerPerformanceAggregator;
pub use config::LiveRatingConfig;
pub use diagnostics::LivePerformanceDiagnosticsState;
pub use finalizer::{
    apply_outcome_to_players, clear_players_outcome, extract_player_match_ratings,
    MatchFinalizationState,
};
pub use player_state::LivePlayerState;
pub use rating_calculator::{
    calculate_confidence, calculate_confidence_from_evidence, calculate_dual_rating,
    calculate_dual_rating_with_exposure, calculate_impact_adjustment, calculate_impact_signal,
    calculate_quality_latent, calculate_rating, calculate_rating_from_latent,
    calculate_rating_with_exposure, calculate_shrunk_latent,
};
pub use replay::filter_surviving_envelopes;
pub use seed::InitialParticipantSeed;
pub use snapshot::LivePerformanceSnapshotRecord;