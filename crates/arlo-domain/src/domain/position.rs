use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionLine {
    OffensiveLine,
    BackLine,
    DefenseLine,
    Goalguard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    CenterOffense,
    WingOffense,
    Midcenter,
    TightWing,
    CenterTight,
    Corridor,
    Artrine,
    Passer,
    PassRusher,
    WideEnd,
    RunningEnd,
    Lineback,
    Fullback,
    Centerback,
    DefensiveEnd,
    Rougieback,
    DefensiveBlocker,
    WideBlocker,
    OutsideZonerback,
    MiddleZonerback,
    Goalguard,
}

impl Position {
    pub fn all() -> [Position; 21] {
        [
            Position::CenterOffense,
            Position::WingOffense,
            Position::Midcenter,
            Position::TightWing,
            Position::CenterTight,
            Position::Corridor,
            Position::Artrine,
            Position::Passer,
            Position::PassRusher,
            Position::WideEnd,
            Position::RunningEnd,
            Position::Lineback,
            Position::Fullback,
            Position::Centerback,
            Position::DefensiveEnd,
            Position::Rougieback,
            Position::DefensiveBlocker,
            Position::WideBlocker,
            Position::OutsideZonerback,
            Position::MiddleZonerback,
            Position::Goalguard,
        ]
    }

    pub fn line(&self) -> PositionLine {
        match self {
            Position::CenterOffense
            | Position::WingOffense
            | Position::Midcenter
            | Position::TightWing
            | Position::CenterTight
            | Position::Corridor => PositionLine::OffensiveLine,
            Position::Artrine
            | Position::Passer
            | Position::PassRusher
            | Position::WideEnd
            | Position::RunningEnd
            | Position::Lineback
            | Position::Fullback => PositionLine::BackLine,
            Position::Centerback
            | Position::DefensiveEnd
            | Position::Rougieback
            | Position::DefensiveBlocker
            | Position::WideBlocker
            | Position::OutsideZonerback
            | Position::MiddleZonerback => PositionLine::DefenseLine,
            Position::Goalguard => PositionLine::Goalguard,
        }
    }
}
