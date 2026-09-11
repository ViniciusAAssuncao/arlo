use crate::attributes::RefereeAttributeTable;
use arlo_domain::Referee;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefereeRegistry {
    head_referee: Referee,
    peace_referee: Referee,
    head_referee_table: RefereeAttributeTable,
    peace_referee_table: RefereeAttributeTable,
}

impl RefereeRegistry {
    pub fn new(
        head_referee: Referee,
        peace_referee: Referee,
        head_referee_table: RefereeAttributeTable,
        peace_referee_table: RefereeAttributeTable,
    ) -> Self {
        Self {
            head_referee,
            peace_referee,
            head_referee_table,
            peace_referee_table,
        }
    }

    pub fn head_referee(&self) -> &Referee {
        &self.head_referee
    }

    pub fn peace_referee(&self) -> &Referee {
        &self.peace_referee
    }

    pub fn head_referee_table(&self) -> RefereeAttributeTable {
        self.head_referee_table
    }

    pub fn peace_referee_table(&self) -> RefereeAttributeTable {
        self.peace_referee_table
    }
}