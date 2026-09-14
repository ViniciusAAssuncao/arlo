use crate::error::{DbError, DbResult};
use crate::models::league_calendar::schedule_block_codes::{
    parse_block_kind, parse_pool_kind, ScheduleBlockKind, TeamPoolRefKind,
};
use arlo_domain::{ScheduleAlgorithmKind, ScheduleBlock, TeamPoolRef};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ScheduleBlockRow {
    pub id: String,
    pub league_calendar_stage_definition_id: String,
    pub block_order_index: i32,
    pub block_kind: String,
    pub group_a_id: Option<String>,
    pub group_b_id: Option<String>,
    pub mirrored: Option<bool>,
    pub pool_kind: Option<String>,
    pub rounds_count: Option<i32>,
}

impl ScheduleBlockRow {
    pub fn to_domain(&self, pool_group_ids: Vec<Uuid>) -> DbResult<ScheduleBlock> {
        let block_kind = parse_block_kind(&self.block_kind)?;
        match block_kind {
            ScheduleBlockKind::GroupRoundRobin => {
                let group_id_str = self.group_a_id.as_deref().ok_or_else(|| {
                    DbError::InvalidData("GroupRoundRobin requires group_a_id".to_string())
                })?;
                let group_id = Uuid::parse_str(group_id_str)?;
                Ok(ScheduleBlock::GroupRoundRobin {
                    group_id,
                    algorithm: ScheduleAlgorithmKind::RoundRobinDoubleLeg,
                })
            }
            ScheduleBlockKind::CrossGroupPairing => {
                let group_a_str = self.group_a_id.as_deref().ok_or_else(|| {
                    DbError::InvalidData("CrossGroupPairing requires group_a_id".to_string())
                })?;
                let group_b_str = self.group_b_id.as_deref().ok_or_else(|| {
                    DbError::InvalidData("CrossGroupPairing requires group_b_id".to_string())
                })?;
                let group_a_id = Uuid::parse_str(group_a_str)?;
                let group_b_id = Uuid::parse_str(group_b_str)?;
                let mirrored = self.mirrored.unwrap_or(false);
                Ok(ScheduleBlock::CrossGroupPairing {
                    group_a_id,
                    group_b_id,
                    mirrored,
                })
            }
            ScheduleBlockKind::RandomPoolRounds => {
                let pool_kind_str = self.pool_kind.as_deref().ok_or_else(|| {
                    DbError::InvalidData("RandomPoolRounds requires pool_kind".to_string())
                })?;
                let pool_kind = parse_pool_kind(pool_kind_str)?;
                let pool = match pool_kind {
                    TeamPoolRefKind::AllGroups => TeamPoolRef::AllGroups,
                    TeamPoolRefKind::SpecificGroups => TeamPoolRef::SpecificGroups(pool_group_ids),
                };
                let rounds_count = self.rounds_count.ok_or_else(|| {
                    DbError::InvalidData("RandomPoolRounds requires rounds_count".to_string())
                })? as u32;
                Ok(ScheduleBlock::RandomPoolRounds {
                    pool,
                    rounds_count,
                })
            }
        }
    }
}
