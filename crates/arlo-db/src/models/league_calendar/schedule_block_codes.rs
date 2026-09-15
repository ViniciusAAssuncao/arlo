use crate::error::{DbError, DbResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleBlockKind {
    GroupRoundRobin,
    CrossGroupPairing,
    RandomPoolRounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamPoolRefKind {
    AllGroups,
    SpecificGroups,
}

pub fn parse_block_kind(code: &str) -> DbResult<ScheduleBlockKind> {
    match code {
        "GroupRoundRobin" | "group_round_robin" => Ok(ScheduleBlockKind::GroupRoundRobin),
        "CrossGroupPairing" | "cross_group_pairing" => Ok(ScheduleBlockKind::CrossGroupPairing),
        "RandomPoolRounds" | "random_pool_rounds" => Ok(ScheduleBlockKind::RandomPoolRounds),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid schedule block kind: {code}"
        ))),
    }
}

pub fn block_kind_to_code(kind: ScheduleBlockKind) -> &'static str {
    match kind {
        ScheduleBlockKind::GroupRoundRobin => "GroupRoundRobin",
        ScheduleBlockKind::CrossGroupPairing => "CrossGroupPairing",
        ScheduleBlockKind::RandomPoolRounds => "RandomPoolRounds",
    }
}

pub fn parse_pool_kind(code: &str) -> DbResult<TeamPoolRefKind> {
    match code {
        "AllGroups" | "all_groups" => Ok(TeamPoolRefKind::AllGroups),
        "SpecificGroups" | "specific_groups" => Ok(TeamPoolRefKind::SpecificGroups),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid team pool kind: {code}"
        ))),
    }
}

pub fn pool_kind_to_code(kind: TeamPoolRefKind) -> &'static str {
    match kind {
        TeamPoolRefKind::AllGroups => "AllGroups",
        TeamPoolRefKind::SpecificGroups => "SpecificGroups",
    }
}
