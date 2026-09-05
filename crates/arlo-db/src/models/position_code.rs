use crate::error::{DbError, DbResult};
use arlo_domain::Position;

pub fn parse_position(code: &str) -> DbResult<Position> {
    match code {
        "C-O" => Ok(Position::CenterOffense),
        "W-O" => Ok(Position::WingOffense),
        "MC" => Ok(Position::Midcenter),
        "TW" => Ok(Position::TightWing),
        "CW" => Ok(Position::CenterTight),
        "C" => Ok(Position::Corridor),
        "A" => Ok(Position::Artrine),
        "P" => Ok(Position::Passer),
        "P-R" => Ok(Position::PassRusher),
        "W-E" => Ok(Position::WideEnd),
        "R-E" => Ok(Position::RunningEnd),
        "L" => Ok(Position::Lineback),
        "F" => Ok(Position::Fullback),
        "CB" => Ok(Position::Centerback),
        "DE" => Ok(Position::DefensiveEnd),
        "RB" => Ok(Position::Rougieback),
        "D-B" => Ok(Position::DefensiveBlocker),
        "W-B" => Ok(Position::WideBlocker),
        "OZB" => Ok(Position::OutsideZonerback),
        "MZB" => Ok(Position::MiddleZonerback),
        "G" => Ok(Position::Goalguard),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid position code: {code}"
        ))),
    }
}

pub fn position_to_code(position: Position) -> &'static str {
    match position {
        Position::CenterOffense => "C-O",
        Position::WingOffense => "W-O",
        Position::Midcenter => "MC",
        Position::TightWing => "TW",
        Position::CenterTight => "CW",
        Position::Corridor => "C",
        Position::Artrine => "A",
        Position::Passer => "P",
        Position::PassRusher => "P-R",
        Position::WideEnd => "W-E",
        Position::RunningEnd => "R-E",
        Position::Lineback => "L",
        Position::Fullback => "F",
        Position::Centerback => "CB",
        Position::DefensiveEnd => "DE",
        Position::Rougieback => "RB",
        Position::DefensiveBlocker => "D-B",
        Position::WideBlocker => "W-B",
        Position::OutsideZonerback => "OZB",
        Position::MiddleZonerback => "MZB",
        Position::Goalguard => "G",
    }
}
