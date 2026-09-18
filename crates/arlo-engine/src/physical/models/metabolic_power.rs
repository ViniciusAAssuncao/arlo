use arlo_domain::Position;

pub fn positional_strain_multiplier(position: Position) -> f64 {
    match position {
        Position::CenterTight | Position::DefensiveBlocker | Position::PassRusher => 1.25,
        Position::Artrine | Position::CenterOffense | Position::Lineback => 1.15,
        Position::Corridor | Position::RunningEnd | Position::TightWing | Position::WideBlocker => 1.10,
        Position::WingOffense | Position::WideEnd | Position::OutsideZonerback | Position::MiddleZonerback => 1.00,
        Position::Passer | Position::Midcenter | Position::Fullback | Position::Centerback | Position::DefensiveEnd | Position::Rougieback => 0.95,
        Position::Goalguard => 0.60,
    }
}