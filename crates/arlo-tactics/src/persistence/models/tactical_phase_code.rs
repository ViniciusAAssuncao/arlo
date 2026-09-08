use crate::error::{TacticsError, TacticsResult};
use crate::persistence::models::tactical_phase::TacticalPhase;

pub fn parse_tactical_phase(code: &str) -> TacticsResult<TacticalPhase> {
    match code {
        "InPossession" | "in_possession" => Ok(TacticalPhase::InPossession),
        "OutOfPossession" | "out_of_possession" => Ok(TacticalPhase::OutOfPossession),
        "Transition" | "transition" => Ok(TacticalPhase::Transition),
        _ => Err(TacticsError::InvalidEnum(format!(
            "Invalid tactical phase: {code}"
        ))),
    }
}

pub fn tactical_phase_to_code(phase: TacticalPhase) -> &'static str {
    match phase {
        TacticalPhase::InPossession => "InPossession",
        TacticalPhase::OutOfPossession => "OutOfPossession",
        TacticalPhase::Transition => "Transition",
    }
}