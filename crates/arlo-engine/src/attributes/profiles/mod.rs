pub mod decision;
pub mod defense;
pub mod goalkeeping;
pub mod offense;
pub mod profile;
pub mod psychology;

pub use decision::*;
pub use defense::*;
pub use goalkeeping::*;
pub use offense::*;
pub use profile::*;
pub use psychology::*;

pub use profile::AttributeProfile as DuelProfile;

use crate::resolution::duel_kind::DuelKind;
use arlo_domain::Position;

pub fn get_position_attribute_profile(position: Position) -> AttributeProfile {
    match position {
        Position::CenterOffense => center_offense_profile(),
        Position::WingOffense => wing_offense_profile(),
        Position::Midcenter => midcenter_profile(),
        Position::TightWing => tight_wing_profile(),
        Position::CenterTight => center_tight_profile(),
        Position::Corridor => corridor_profile(),
        Position::Artrine => artrine_position_profile(),
        Position::Passer => passer_position_profile(),
        Position::PassRusher => pass_rusher_position_profile(),
        Position::WideEnd => wide_end_profile(),
        Position::RunningEnd => running_end_profile(),
        Position::Lineback => lineback_profile(),
        Position::Fullback => fullback_profile(),
        Position::Centerback => centerback_profile(),
        Position::DefensiveEnd => defensive_end_profile(),
        Position::Rougieback => rougieback_profile(),
        Position::DefensiveBlocker => defensive_blocker_profile(),
        Position::WideBlocker => wide_blocker_profile(),
        Position::OutsideZonerback => outside_zonerback_profile(),
        Position::MiddleZonerback => middle_zonerback_profile(),
        Position::Goalguard => goalguard_profile(),
    }
}

pub fn get_duel_attribute_profiles(kind: DuelKind) -> (AttributeProfile, AttributeProfile) {
    match kind {
        DuelKind::PassProtection | DuelKind::KickBlockAttempt => {
            (pass_protection_profile(), pass_rush_profile())
        }
        DuelKind::RouteContest => (route_contest_profile(), coverage_profile()),
        DuelKind::RunBreakthrough => (run_breakthrough_profile(), run_containment_profile()),
        DuelKind::CentralBlock => (central_block_profile(), central_resistance_profile()),
        DuelKind::LateralBlock => (lateral_block_profile(), lateral_resistance_profile()),
        DuelKind::ArtroBreakthrough => (artro_breakthrough_profile(), artro_defense_profile()),
        DuelKind::AerialDuel => (aerial_duel_profile(), aerial_defense_profile()),
        DuelKind::FinishingAttempt => (finishing_attempt_profile(), shot_stopping_profile()),
        DuelKind::FieldGoalAttempt => (field_goal_profile(), shot_stopping_profile()),
        DuelKind::ShortDistribution => (short_distribution_profile(), coverage_profile()),
        DuelKind::LongDistribution => (long_distribution_profile(), aerial_defense_profile()),
        DuelKind::CrossDistribution => (cross_distribution_profile(), coverage_profile()),
        DuelKind::BallSecurityCarry => (ball_security_carry_profile(), dispossession_profile()),
        DuelKind::BallSecurityDistribution => (
            ball_security_distribution_profile(),
            dispossession_profile(),
        ),
    }
}

pub use get_duel_attribute_profiles as get_duel_profiles;