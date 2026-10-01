use crate::error::{TacticsError, TacticsResult};
use crate::playcall::category::PlayCallCategory;

pub fn parse_play_call_category(code: &str) -> TacticsResult<PlayCallCategory> {
    match code {
        "OpenPlay" | "open_play" => Ok(PlayCallCategory::OpenPlay),
        "BonusPhaseConversion" | "bonus_phase_conversion" => {
            Ok(PlayCallCategory::BonusPhaseConversion)
        }
        _ => Err(TacticsError::InvalidEnum(format!(
            "Invalid play call category: {code}"
        ))),
    }
}

pub fn play_call_category_to_code(category: PlayCallCategory) -> &'static str {
    match category {
        PlayCallCategory::OpenPlay => "OpenPlay",
        PlayCallCategory::BonusPhaseConversion => "BonusPhaseConversion",
    }
}
