use crate::error::TacticsResult;
use crate::persistence::models::rows::series_script_entry_row::SeriesScriptEntryRow;
use crate::series::SeriesScript;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct SeriesScriptRow {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub created_at_unix_seconds: i64,
}

impl SeriesScriptRow {
    pub fn to_domain(&self, entry_rows: &[SeriesScriptEntryRow]) -> TacticsResult<SeriesScript> {
        let id = Uuid::parse_str(&self.id)?;
        let team_id = Uuid::parse_str(&self.team_id)?;

        let mut sorted_entries = entry_rows.to_vec();
        sorted_entries.sort_by_key(|e| e.sequence_index);

        let mut entries = Vec::with_capacity(sorted_entries.len());
        for entry in sorted_entries {
            let play_call_id = Uuid::parse_str(&entry.play_call_id)?;
            entries.push(play_call_id);
        }

        SeriesScript::new(id, team_id, &self.name, entries)
    }
}
