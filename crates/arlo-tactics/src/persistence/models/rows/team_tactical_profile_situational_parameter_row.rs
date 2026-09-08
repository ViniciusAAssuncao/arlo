use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct TeamTacticalProfileSituationalParameterRow {
    pub id: String,
    pub team_tactical_profile_id: String,
    pub parameter_key: String,
    pub value: f64,
}
