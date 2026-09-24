use crate::error::DbResult;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueCalendarCollectiveAgreementRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub collective_agreement_id: String,
}

impl LeagueCalendarCollectiveAgreementRow {
    pub fn collective_agreement_id(&self) -> DbResult<Uuid> {
        Uuid::parse_str(&self.collective_agreement_id).map_err(Into::into)
    }
}
