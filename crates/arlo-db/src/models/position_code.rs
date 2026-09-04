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
