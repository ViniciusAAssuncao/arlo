use crate::error::TacticsResult;
use crate::series::series_script::SeriesScript;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesScriptBuilder {
    id: Uuid,
    team_id: Uuid,
    name: String,
    entries: Vec<Uuid>,
}

impl SeriesScriptBuilder {
    pub fn new(id: Uuid, team_id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            team_id,
            name: name.into(),
            entries: Vec::new(),
        }
    }

    pub fn append(mut self, play_call_id: Uuid) -> Self {
        self.entries.push(play_call_id);
        self
    }

    pub fn with_entries(mut self, entries: impl IntoIterator<Item = Uuid>) -> Self {
        self.entries.extend(entries);
        self
    }

    pub fn build(self) -> TacticsResult<SeriesScript> {
        SeriesScript::new(self.id, self.team_id, self.name, self.entries)
    }
}
