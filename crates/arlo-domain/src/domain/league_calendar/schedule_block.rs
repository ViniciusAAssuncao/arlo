use crate::domain::league_calendar::schedule_algorithm_kind::ScheduleAlgorithmKind;
use crate::domain::league_calendar::team_pool_ref::TeamPoolRef;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleBlock {
    GroupRoundRobin {
        group_id: Uuid,
        algorithm: ScheduleAlgorithmKind,
    },
    CrossGroupPairing {
        group_a_id: Uuid,
        group_b_id: Uuid,
        mirrored: bool,
    },
    RandomPoolRounds {
        pool: TeamPoolRef,
        rounds_count: u32,
    },
}
