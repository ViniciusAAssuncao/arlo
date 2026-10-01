pub mod live;
pub mod observation;
pub mod profile;
pub mod rating;
pub mod translator;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use live::{
    calculate_confidence, calculate_dual_rating, calculate_rating, calculate_rating_from_latent,
    LivePerformanceSnapshotRecord, LivePlayerState, LiveRatingConfig, PlayerPerformanceAggregator,
};
pub use observation::{
    ObservationCategory, PerformanceObservation, PerformanceObservationBuilder, PossessionPhase,
};
pub use profile::{
    base_weights_for_line, overlay_for_role, weights_for_position, PerformanceProfile,
    PerformanceProfileWeights, SlotRoleOverlay,
};
pub use rating::{
    MatchOutcome, ModelVersion, OutcomeAdjustmentPolicy, PerformanceBreakdown,
    PerformanceConfidence, PerformanceRating, PlayerPerformanceSnapshot,
};
pub use translator::EventPerformanceTranslator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMatchRating {
    player_id: Uuid,
    rating: f64,
}

impl PlayerMatchRating {
    pub fn new(player_id: Uuid, rating: f64) -> Self {
        Self { player_id, rating }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn rating(&self) -> f64 {
        self.rating
    }
}