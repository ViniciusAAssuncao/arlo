use arlo_domain::{ArtrineDecisionKind, Position, SlotRole};
use arlo_tactics::PlayerInstructions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarrierTacticalBias;

impl CarrierTacticalBias {
    pub fn calculate_bias(
        position: Position,
        role: SlotRole,
        instructions: &PlayerInstructions,
        decision_kind: ArtrineDecisionKind,
        normalized_proximity: f64,
        is_lateral: bool,
    ) -> f64 {
        let base_position_bias = match decision_kind {
            ArtrineDecisionKind::SelfFinish => match position {
                Position::CenterOffense => {
                    if normalized_proximity >= 0.85 {
                        3.20
                    } else if normalized_proximity >= 0.65 {
                        2.40
                    } else {
                        1.60
                    }
                }
                Position::Corridor => {
                    if normalized_proximity >= 0.80 {
                        1.60
                    } else {
                        1.20
                    }
                }
                Position::RunningEnd => {
                    if normalized_proximity >= 0.80 {
                        1.45
                    } else {
                        1.15
                    }
                }
                Position::WingOffense => {
                    if normalized_proximity >= 0.85 {
                        1.30
                    } else {
                        0.95
                    }
                }
                Position::TightWing => {
                    if normalized_proximity >= 0.85 {
                        1.20
                    } else {
                        0.90
                    }
                }
                Position::Artrine => 1.05,
                Position::CenterTight => 0.80,
                Position::Midcenter => 0.70,
                Position::WideEnd => 0.90,
                Position::Passer => 0.50,
                Position::Fullback | Position::Lineback => 0.60,
                _ => 0.50,
            },
            ArtrineDecisionKind::Cross => match position {
                Position::WingOffense => {
                    if is_lateral {
                        2.40
                    } else {
                        1.60
                    }
                }
                Position::TightWing => {
                    if is_lateral {
                        1.80
                    } else {
                        1.30
                    }
                }
                Position::WideEnd => {
                    if is_lateral {
                        1.60
                    } else {
                        1.20
                    }
                }
                Position::Corridor => {
                    if is_lateral {
                        1.30
                    } else {
                        0.90
                    }
                }
                Position::Artrine => 1.15,
                Position::Midcenter => 1.00,
                Position::Passer => 1.10,
                Position::CenterTight => 0.70,
                Position::CenterOffense => 0.50,
                _ => 0.80,
            },
            ArtrineDecisionKind::SelfCarry => match position {
                Position::Corridor => 2.50,
                Position::RunningEnd => 1.85,
                Position::CenterTight => 1.30,
                Position::Artrine => 1.25,
                Position::TightWing => 1.20,
                Position::WingOffense => 1.10,
                Position::CenterOffense => 1.00,
                Position::Midcenter => 0.90,
                Position::Passer => 0.45,
                _ => 0.85,
            },
            ArtrineDecisionKind::ShortPass => match position {
                Position::Midcenter => 2.20,
                Position::Passer => 1.65,
                Position::Artrine => 1.45,
                Position::CenterTight => 1.30,
                Position::TightWing => 1.20,
                Position::Fullback | Position::Lineback => 1.30,
                Position::WingOffense => 1.00,
                Position::Corridor => 0.80,
                Position::CenterOffense => 0.75,
                _ => 1.00,
            },
            ArtrineDecisionKind::LongLaunch => match position {
                Position::Passer => 2.20,
                Position::Artrine => 1.40,
                Position::Midcenter => 1.20,
                Position::WideEnd => 1.15,
                Position::WingOffense => 1.05,
                Position::CenterTight => 0.55,
                Position::Corridor => 0.50,
                Position::CenterOffense => 0.45,
                _ => 0.70,
            },
        };

        let role_multiplier = match role {
            SlotRole::Kicker => match decision_kind {
                ArtrineDecisionKind::SelfFinish => 1.60,
                ArtrineDecisionKind::Cross => 0.80,
                _ => 1.00,
            },
            SlotRole::Launcher => match decision_kind {
                ArtrineDecisionKind::LongLaunch => 2.20,
                ArtrineDecisionKind::ShortPass => 0.90,
                ArtrineDecisionKind::SelfCarry => 0.70,
                _ => 1.00,
            },
            SlotRole::FalseArtrine => match decision_kind {
                ArtrineDecisionKind::SelfCarry => 1.30,
                ArtrineDecisionKind::ShortPass => 1.10,
                _ => 1.00,
            },
            SlotRole::Safeguard | SlotRole::Blocker => match decision_kind {
                ArtrineDecisionKind::ShortPass => 1.20,
                ArtrineDecisionKind::SelfCarry => 0.80,
                ArtrineDecisionKind::SelfFinish => 0.70,
                _ => 1.00,
            },
            SlotRole::Standard => 1.00,
        };

        let creative_license = instructions.in_possession().creative_license().value();
        let involvement = instructions.in_possession().involvement_priority().value();
        let positioning_bias = instructions.in_possession().positioning_bias().value();

        let instruction_modifier = match decision_kind {
            ArtrineDecisionKind::SelfFinish => {
                1.0 + positioning_bias * 0.20 + creative_license * 0.15 + involvement * 0.20
            }
            ArtrineDecisionKind::Cross => 1.0 + creative_license * 0.20 + involvement * 0.10,
            ArtrineDecisionKind::LongLaunch => 1.0 + creative_license * 0.30 - involvement * 0.05,
            ArtrineDecisionKind::ShortPass => 1.0 - creative_license * 0.15 + involvement * 0.10,
            ArtrineDecisionKind::SelfCarry => 1.0 + positioning_bias * 0.15 + involvement * 0.20,
        };

        (base_position_bias * role_multiplier * instruction_modifier).clamp(0.20, 5.00)
    }
}
