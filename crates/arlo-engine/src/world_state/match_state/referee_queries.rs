use crate::attributes::RefereeAttributeTable;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::Referee;

impl MatchState {
    pub fn head_referee(&self) -> &Referee {
        self.referees.head_referee()
    }

    pub fn peace_referee(&self) -> &Referee {
        self.referees.peace_referee()
    }

    pub fn head_referee_attribute_table(&self) -> RefereeAttributeTable {
        self.referees.head_referee_table()
    }

    pub fn peace_referee_attribute_table(&self) -> RefereeAttributeTable {
        self.referees.peace_referee_table()
    }
}