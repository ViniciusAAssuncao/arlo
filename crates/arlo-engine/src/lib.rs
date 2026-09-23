pub mod error;
pub mod input;
pub mod resolution;
pub mod state;
pub mod step;

pub use error::{EngineError, EngineResult};
pub use input::{MatchInput, TeamInput};
pub use resolution::{
    award_kick_foul_segment, resolve_kick_foul_segment, resolve_missed_shot_recovery,
    resolve_next_segment, resolve_time_call_segment,
};
pub use state::{
    ClockState, DriveProgress, MatchPhase, MatchState, PossessionState, Score, ScoreKind,
    SeriesAdvance, SeriesOut, SeriesState, TeamState,
};
pub use step::{StepOutcome, StepResult};
