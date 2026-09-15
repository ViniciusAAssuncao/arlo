pub mod challenge_bridge;
pub mod foul_challenge_bridge;
pub mod kick_foul_realignment_bridge;
pub mod play_call_bridge;
pub mod substitution_bridge;
pub mod tactical_switch_bridge;
pub mod time_call_bridge;

pub use challenge_bridge::try_apply_human_challenge;
pub use foul_challenge_bridge::try_apply_human_foul_challenge;
pub use kick_foul_realignment_bridge::try_apply_human_kick_foul_realignment;
pub use play_call_bridge::try_apply_human_play_call;
pub use substitution_bridge::try_apply_human_substitutions;
pub use tactical_switch_bridge::try_apply_human_tactical_switch;
pub use time_call_bridge::try_apply_human_time_call;
