use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PostponementReason {
    GamesPerWeekConflict,
    ManualOverride,
    InsufficientRestGap,
    CollectiveAgreementBlackout,
}
