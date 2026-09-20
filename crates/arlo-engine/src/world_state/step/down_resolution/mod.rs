pub mod collateral_stage;
pub mod contest_stage;
pub mod context;
pub mod decision_stage;
pub mod executor;
pub mod power_pair;
pub mod progression_stage;
pub mod scoring_stage;

pub use collateral_stage::{resolve_collateral_events, ActionCollateralOutcome};
pub use contest_stage::{resolve_contest, ActionContestOutcome};
pub use context::DownResolutionContext;
pub use decision_stage::resolve_decision;
pub use executor::resolve_down;
pub use power_pair::{derive_power_pair, DuelPowerPair};
pub use progression_stage::{resolve_progression, ActionProgressionOutcome};
pub use scoring_stage::resolve_scoring;