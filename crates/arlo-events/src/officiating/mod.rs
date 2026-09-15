pub mod added_time_awarded;
pub mod foul_origin;
pub mod foul_raised;

pub use added_time_awarded::AddedTimeAwarded;
pub use foul_origin::FoulOrigin;
pub use foul_raised::FoulRaised;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OfficiatingEvent {
    FoulRaised(FoulRaised),
    AddedTimeAwarded(AddedTimeAwarded),
}

impl From<FoulRaised> for OfficiatingEvent {
    fn from(ev: FoulRaised) -> Self {
        Self::FoulRaised(ev)
    }
}

impl From<AddedTimeAwarded> for OfficiatingEvent {
    fn from(ev: AddedTimeAwarded) -> Self {
        Self::AddedTimeAwarded(ev)
    }
}
