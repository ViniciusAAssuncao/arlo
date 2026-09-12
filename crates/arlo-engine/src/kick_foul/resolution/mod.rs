pub mod block_phase;
pub mod dispatcher;
pub mod outcome;
pub mod participants;
pub mod restart_phase;
pub mod shoot_phase;
pub mod tier_opportunity;

pub use block_phase::resolve_kick_block_duel;
pub use dispatcher::resolve_kick_foul;
pub use outcome::KickFoulResolutionOutcome;
pub use participants::{select_kick_foul_participants, KickFoulParticipants};
pub use restart_phase::{resolve_kick_foul_restart, KickFoulRestartResult};
pub use shoot_phase::resolve_kick_foul_shot;
pub use tier_opportunity::evaluate_kick_foul_scoring_opportunity;
