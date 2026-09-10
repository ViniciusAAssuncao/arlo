use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error(transparent)]
    Domain(#[from] arlo_domain::DomainError),
    #[error(transparent)]
    Tactics(#[from] arlo_tactics::TacticsError),
    #[error("Lineup must have exactly {expected} players, found {actual}")]
    InvalidLineupSize { expected: usize, actual: usize },
    #[error("Duplicate player '{0}' in lineup")]
    DuplicatePlayer(Uuid),
    #[error("Formation slot count mismatch: expected {expected}, found {actual}")]
    SlotCountMismatch { expected: usize, actual: usize },
    #[error("Player '{0}' not found in lineup")]
    PlayerNotFound(Uuid),
    #[error("Lineup conflict: player '{0}' is present in both home and away lineups")]
    LineupConflict(Uuid),
    #[error("Venue '{0}' has no valid pitch dimensions")]
    MissingPitchDimensions(Uuid),
    #[error("Invalid venue kind for match: expected MatchStadium")]
    InvalidVenueKind,
    #[error("Total anchor count mismatch: expected {expected}, found {actual}")]
    AnchorCountMismatch { expected: usize, actual: usize },
    #[error("Missing required position '{0}' in lineup")]
    MissingRequiredPosition(String),
}

pub type EngineResult<T> = Result<T, EngineError>;
