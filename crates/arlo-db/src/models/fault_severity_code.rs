use crate::error::{DbError, DbResult};
use arlo_domain::FaultSeverity;

pub fn parse_fault_severity(code: &str) -> DbResult<FaultSeverity> {
    match code {
        "Minor" | "minor" => Ok(FaultSeverity::Minor),
        "Moderate" | "moderate" => Ok(FaultSeverity::Moderate),
        "Severe" | "severe" => Ok(FaultSeverity::Severe),
        "Flagrant" | "flagrant" => Ok(FaultSeverity::Flagrant),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid fault severity: {code}"
        ))),
    }
}

pub fn fault_severity_to_code(severity: FaultSeverity) -> &'static str {
    match severity {
        FaultSeverity::Minor => "Minor",
        FaultSeverity::Moderate => "Moderate",
        FaultSeverity::Severe => "Severe",
        FaultSeverity::Flagrant => "Flagrant",
    }
}