use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::validation::{validate_no_duplicate_keys, validate_not_empty};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompetitionGroup {
    id: Uuid,
    order_index: u32,
    name: String,
    team_ids: Vec<Uuid>,
}

impl CompetitionGroup {
    pub fn new(
        id: Uuid,
        order_index: u32,
        name: impl Into<String>,
        team_ids: Vec<Uuid>,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;

        if team_ids.is_empty() {
            return Err(DomainError::InvalidInvariant {
                field: "team_ids".to_string(),
                violation: InvariantViolation::Empty,
            });
        }

        validate_no_duplicate_keys(&team_ids, |id| *id, "team_ids", "team_id")?;

        Ok(Self {
            id,
            order_index,
            name,
            team_ids,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn order_index(&self) -> u32 {
        self.order_index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn team_ids(&self) -> &[Uuid] {
        &self.team_ids
    }
}

pub fn build_team_to_group_index(groups: &[CompetitionGroup]) -> HashMap<Uuid, Uuid> {
    let mut map = HashMap::new();
    for group in groups {
        for &team_id in group.team_ids() {
            map.insert(team_id, group.id());
        }
    }
    map
}
