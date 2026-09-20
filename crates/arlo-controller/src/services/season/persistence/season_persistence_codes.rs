use crate::domain::season::{
    FixtureStatus, PostponementReason, SeasonInstanceStatus, StageStatus,
};
use arlo_domain::StageType;

pub fn fixture_status_to_code(status: FixtureStatus) -> &'static str {
    match status {
        FixtureStatus::Scheduled => "Scheduled",
        FixtureStatus::Postponed => "Postponed",
        FixtureStatus::Completed => "Completed",
        FixtureStatus::Cancelled => "Cancelled",
    }
}

pub fn season_instance_status_to_code(status: SeasonInstanceStatus) -> &'static str {
    match status {
        SeasonInstanceStatus::Pending => "Pending",
        SeasonInstanceStatus::Active => "Active",
        SeasonInstanceStatus::Completed => "Completed",
    }
}

pub fn stage_status_to_code(status: StageStatus) -> &'static str {
    match status {
        StageStatus::Pending => "Pending",
        StageStatus::Active => "Active",
        StageStatus::Completed => "Completed",
    }
}

pub fn stage_type_to_code(stage_type: StageType) -> &'static str {
    match stage_type {
        StageType::RoundRobinTable => "RoundRobinTable",
        StageType::KnockoutBracket => "KnockoutBracket",
        StageType::GroupedCompetitionTable => "GroupedCompetitionTable",
    }
}

pub fn postponement_reason_to_code(reason: PostponementReason) -> &'static str {
    match reason {
        PostponementReason::GamesPerWeekConflict => "GamesPerWeekConflict",
        PostponementReason::ManualOverride => "ManualOverride",
        PostponementReason::InsufficientRestGap => "InsufficientRestGap",
        PostponementReason::CollectiveAgreementBlackout => "CollectiveAgreementBlackout",
    }
}