pub mod back_line;
pub mod defense_line;
pub mod goalguard;
pub mod offensive_line;

pub use back_line::*;
pub use defense_line::*;
pub use goalguard::*;
pub use offensive_line::*;

use crate::current_ability::weights::PositionWeightProfile;
use arlo_domain::Position;

pub fn get_profile_for_position(position: Position) -> PositionWeightProfile {
    match position {
        Position::CenterOffense => offensive_line::center_offense_profile(),
        Position::WingOffense => offensive_line::wing_offense_profile(),
        Position::Midcenter => offensive_line::midcenter_profile(),
        Position::TightWing => offensive_line::tight_wing_profile(),
        Position::CenterTight => offensive_line::center_tight_profile(),
        Position::Corridor => offensive_line::corridor_profile(),
        Position::Artrine => back_line::artrine_profile(),
        Position::Passer => back_line::passer_profile(),
        Position::PassRusher => back_line::pass_rusher_profile(),
        Position::WideEnd => back_line::wide_end_profile(),
        Position::RunningEnd => back_line::running_end_profile(),
        Position::Lineback => back_line::lineback_profile(),
        Position::Fullback => back_line::fullback_profile(),
        Position::Centerback => defense_line::centerback_profile(),
        Position::DefensiveEnd => defense_line::defensive_end_profile(),
        Position::Rougieback => defense_line::rougieback_profile(),
        Position::DefensiveBlocker => defense_line::defensive_blocker_profile(),
        Position::WideBlocker => defense_line::wide_blocker_profile(),
        Position::OutsideZonerback => defense_line::outside_zonerback_profile(),
        Position::MiddleZonerback => defense_line::middle_zonerback_profile(),
        Position::Goalguard => goalguard::goalguard_profile(),
    }
}
