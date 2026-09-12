pub mod foul_record;
pub mod kick_foul_record;
pub mod punishment_record;
pub mod referee_record;

pub use foul_record::{PlayerFoulAggregator, PlayerFoulStats};
pub use kick_foul_record::{PlayerKickFoulAggregator, PlayerKickFoulStats};
pub use punishment_record::{PlayerPunishmentAggregator, PlayerPunishmentStats};
pub use referee_record::{RefereeMatchStats, RefereeStatsAggregator};
