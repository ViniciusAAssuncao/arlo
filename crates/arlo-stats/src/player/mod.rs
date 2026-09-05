pub mod drives;
pub mod duel_record;
pub mod touches;

pub use drives::{PlayerDriveStats, PlayerDrivesAggregator};
pub use duel_record::{DuelKindStats, PlayerDuelAggregator, PlayerDuelStats};
pub use touches::{PlayerTouchStats, PlayerTouchesAggregator};
