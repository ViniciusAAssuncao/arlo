pub mod list_intent_bridge;
pub mod single_intent_bridge;

pub use list_intent_bridge::{
    apply_forced_substitution_intents, resolve_forced_substitutions_for_team,
    try_apply_human_substitutions,
};
pub use single_intent_bridge::{
    try_apply_human_challenge, try_apply_human_foul_challenge,
    try_apply_human_kick_foul_realignment, try_apply_human_play_call,
    try_apply_human_tactical_switch, try_apply_human_time_call,
};