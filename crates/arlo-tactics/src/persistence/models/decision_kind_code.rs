use crate::error::{TacticsError, TacticsResult};
use arlo_domain::ArtrineDecisionKind;

pub fn parse_artrine_decision_kind(code: &str) -> TacticsResult<ArtrineDecisionKind> {
    match code {
        "self_carry" | "SelfCarry" => Ok(ArtrineDecisionKind::SelfCarry),
        "short_pass" | "ShortPass" => Ok(ArtrineDecisionKind::ShortPass),
        "long_launch" | "LongLaunch" => Ok(ArtrineDecisionKind::LongLaunch),
        "cross" | "Cross" => Ok(ArtrineDecisionKind::Cross),
        "self_finish" | "SelfFinish" => Ok(ArtrineDecisionKind::SelfFinish),
        _ => Err(TacticsError::InvalidEnum(format!(
            "Invalid artrine decision kind: {code}"
        ))),
    }
}

pub fn artrine_decision_kind_to_code(kind: ArtrineDecisionKind) -> &'static str {
    match kind {
        ArtrineDecisionKind::SelfCarry => "self_carry",
        ArtrineDecisionKind::ShortPass => "short_pass",
        ArtrineDecisionKind::LongLaunch => "long_launch",
        ArtrineDecisionKind::Cross => "cross",
        ArtrineDecisionKind::SelfFinish => "self_finish",
    }
}
