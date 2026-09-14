use crate::error::DbResult;
use crate::models::league_calendar::stage_definition_codes::{
    parse_knockout_leg_format, parse_stage_entry_rule, parse_stage_type,
};
use arlo_domain::StageDefinition;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueCalendarStageDefinitionRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub stage_order_index: i32,
    pub stage_type: String,
    pub entry_rule_kind: String,
    pub entry_rule_count: Option<i32>,
    pub leg_format: Option<String>,
}

impl LeagueCalendarStageDefinitionRow {
    pub fn to_domain(&self) -> DbResult<StageDefinition> {
        let stage_type = parse_stage_type(&self.stage_type)?;
        let entry_rule = parse_stage_entry_rule(&self.entry_rule_kind, self.entry_rule_count)?;
        let knockout_leg_format = parse_knockout_leg_format(self.leg_format.as_deref())?;

        StageDefinition::new(
            self.stage_order_index as u32,
            stage_type,
            entry_rule,
            knockout_leg_format,
        )
        .map_err(Into::into)
    }
}
