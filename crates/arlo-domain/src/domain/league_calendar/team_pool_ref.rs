use crate::domain::league_calendar::competition_group::CompetitionGroup;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TeamPoolRef {
    AllGroups,
    SpecificGroups(Vec<Uuid>),
}

impl TeamPoolRef {
    pub fn resolve(&self, groups: &[CompetitionGroup]) -> Vec<Uuid> {
        match self {
            Self::AllGroups => groups
                .iter()
                .flat_map(|group| group.team_ids().iter().copied())
                .collect(),
            Self::SpecificGroups(group_ids) => {
                let mut team_ids = Vec::new();
                for group_id in group_ids {
                    if let Some(group) = groups.iter().find(|g| g.id() == *group_id) {
                        team_ids.extend(group.team_ids().iter().copied());
                    }
                }
                team_ids
            }
        }
    }
}
