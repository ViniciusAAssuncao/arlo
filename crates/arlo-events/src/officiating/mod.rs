pub mod foul_origin;
pub mod foul_raised;

pub use foul_origin::FoulOrigin;
pub use foul_raised::FoulRaised;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfficiatingEvent {
    FoulRaised(FoulRaised),
}

impl From<FoulRaised> for OfficiatingEvent {
    fn from(ev: FoulRaised) -> Self {
        Self::FoulRaised(ev)
    }
}