mod adaptation;
mod matching;

use crate::TacticalLineup;
pub use adaptation::adapt_layout;
use arlo_domain::Formation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalLayout {
    pub formation: Formation,
    pub lineup: TacticalLineup,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedTacticalPlan {
    pub id: Uuid,
    pub name: String,
    pub profile_id: Uuid,
    pub layout: TacticalLayout,
}
