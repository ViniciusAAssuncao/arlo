pub mod error;
pub mod input;
pub mod resolution;
pub mod state;
pub mod step;

pub use error::{EngineError, EngineResult};
pub use input::{MatchInput, TeamInput};
pub use resolution::resolve_next_segment;
pub use state::{
    ClockState, DriveProgress, MatchPhase, MatchState, PossessionState, Score, ScoreKind,
    SeriesAdvance, SeriesOut, SeriesState, TeamState,
};
pub use step::{StepOutcome, StepResult};
