use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct TacticalInstructionValueRow {
    pub id: String,
    pub team_tactical_profile_id: String,
    pub phase: String,
    pub instruction_key: String,
    pub value: f64,
}
