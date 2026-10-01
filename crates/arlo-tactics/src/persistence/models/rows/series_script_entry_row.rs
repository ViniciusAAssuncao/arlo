use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SeriesScriptEntryRow {
    pub id: String,
    pub series_script_id: String,
    pub sequence_index: i32,
    pub play_call_id: String,
}
