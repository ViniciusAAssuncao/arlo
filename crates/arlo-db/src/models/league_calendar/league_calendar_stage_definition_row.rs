use crate::error::DbResult;
use crate::models::league_calendar::stage_definition_codes::{
    parse_knockout_leg_format, parse_stage_type,
};
use arlo_domain::{QualificationPoolRule, ScheduleBlock, StageDefinition, StageEntryRule};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueCalendarStageDefinitionRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub stage_order_index: i32,
    pub stage_type: String,
    pub leg_format: Option<String>,
    pub entry_gap_days: i32,
}

impl LeagueCalendarStageDefinitionRow {
    pub fn to_domain(
        &self,
        schedule_blocks: Option<Vec<ScheduleBlock>>,
        entry_rule_pools: Vec<QualificationPoolRule>,
    ) -> DbResult<StageDefinition> {
        let stage_type = parse_stage_type(&self.stage_type)?;
        let entry_rule = StageEntryRule::new(entry_rule_pools)?;
        let knockout_leg_format = parse_knockout_leg_format(self.leg_format.as_deref())?;

        StageDefinition::new(
            self.stage_order_index as u32,
            stage_type,
            entry_rule,
            knockout_leg_format,
            schedule_blocks,
            self.entry_gap_days as u32,
        )
        .map_err(Into::into)
    }
}
