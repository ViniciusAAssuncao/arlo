use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StoppageEventKind {
    Foul,
    Injury,
    Challenge,
    TimeCall,
    KickFoulAwarded,
    Scoring,
}
