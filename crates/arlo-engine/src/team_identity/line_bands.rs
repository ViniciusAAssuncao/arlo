use arlo_domain::sport_constants::{
    BACK_RX_MAX, BACK_RX_MIN, DEFENSE_RX_MAX, DEFENSE_RX_MIN, GOALGUARD_RX_MAX, GOALGUARD_RX_MIN,
    OFFENSIVE_RX_MAX, OFFENSIVE_RX_MIN,
};
use arlo_domain::PositionLine;

pub fn line_rx_band(line: PositionLine) -> (f64, f64) {
    match line {
        PositionLine::Goalguard => (GOALGUARD_RX_MIN, GOALGUARD_RX_MAX),
        PositionLine::DefenseLine => (DEFENSE_RX_MIN, DEFENSE_RX_MAX),
        PositionLine::BackLine => (BACK_RX_MIN, BACK_RX_MAX),
        PositionLine::OffensiveLine => (OFFENSIVE_RX_MIN, OFFENSIVE_RX_MAX),
    }
}
