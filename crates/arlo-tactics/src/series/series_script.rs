use crate::error::{TacticsError, TacticsResult};
use crate::series::builder::SeriesScriptBuilder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeriesScript {
    id: Uuid,
    team_id: Uuid,
    name: String,
    entries: Vec<Uuid>,
}

impl SeriesScript {
    pub fn new(
        id: Uuid,
        team_id: Uuid,
        name: impl Into<String>,
        entries: Vec<Uuid>,
    ) -> TacticsResult<Self> {
        if entries.is_empty() {
            return Err(TacticsError::InvalidPlayCall(
                "series script requires at least one play call".to_string(),
            ));
        }

        Ok(Self {
            id,
            team_id,
            name: name.into(),
            entries,
        })
    }

    pub fn builder(id: Uuid, team_id: Uuid, name: impl Into<String>) -> SeriesScriptBuilder {
        SeriesScriptBuilder::new(id, team_id, name)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn entries(&self) -> &[Uuid] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
